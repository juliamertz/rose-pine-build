use clap::ValueEnum;
use std::{char, collections::HashMap};
use strum::IntoEnumIterator;

use crate::{
    format::Format,
    palette::Variant,
    parse::{self, ParseError},
    utils::Substitutable,
};

/// HashMap containing output strings for each variant
pub type Outputs = HashMap<Variant, String>;

#[derive(Clone, Debug)]
pub struct Generator {
    config: Config,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Delimiter {
    Parenthesis,
    CurlyBracket,
    AngleBracket,
    SqaureBracket,
}

impl Delimiter {
    pub fn open(&self) -> char {
        match self {
            Self::Parenthesis => '(',
            Self::CurlyBracket => '{',
            Self::AngleBracket => '<',
            Self::SqaureBracket => '[',
        }
    }

    pub fn close(&self) -> char {
        match self {
            Self::Parenthesis => ')',
            Self::CurlyBracket => '}',
            Self::AngleBracket => '>',
            Self::SqaureBracket => ']',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub format: Format,
    pub prefix: char,
    pub seperator: char,
    pub delimiter: Delimiter,
}

impl Config {
    pub fn new(prefix: char, format: Format, seperator: char, delimiter: Delimiter) -> Self {
        Self {
            prefix,
            format,
            seperator,
            delimiter,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: Format::Hex,
            delimiter: Delimiter::Parenthesis,
            prefix: '$',
            seperator: '|',
        }
    }
}

impl Generator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_variants(&self, text: &str) -> Result<Outputs, ParseError> {
        let mut outputs = HashMap::new();
        for v in Variant::iter() {
            outputs.insert(v, self.generate_variant(v, text)?);
        }
        Ok(outputs)
    }

    pub fn generate_variant(&self, variant: Variant, text: &str) -> Result<String, ParseError> {
        let mut buffer = text.to_owned();
        for capture in parse::parse_template(text, &self.config).into_iter().rev() {
            match capture {
                Ok(capture) => {
                    buffer.substitute(
                        capture.format_role(variant, &self.config),
                        capture.start,
                        capture.end,
                    );
                }
                Err(err) => {
                    eprintln!("Unable to parse template, error: {err:?}");
                }
            }
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use super::*;

    static GENERATOR: OnceLock<Generator> = OnceLock::new();
    pub fn generate_variant(variant: Variant, text: &str) -> Result<String, ParseError> {
        GENERATOR
            .get_or_init(|| Generator::new(Config::default()))
            .generate_variant(variant, text)
    }

    #[test]
    fn format_rgb() -> Result<(), ParseError> {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb")?,
            "235, 111, 146"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb_function")?,
            "rgb(235, 111, 146)"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$pine:rgb_function/80")?,
            "rgba(62, 143, 176, 0.8)"
        );
        Ok(())
    }

    #[test]
    fn format_parse_order() -> Result<(), ParseError> {
        assert_eq!(
            generate_variant(
                Variant::Moon,
                "$love:rgb_function; $love:rgb; $love:hex_ns; $love:hex",
            )?,
            "rgb(235, 111, 146); 235, 111, 146; eb6f92; #eb6f92"
        );
        Ok(())
    }

    #[test]
    fn format_hsl() -> Result<(), ParseError> {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:hsl_function")?,
            "hsl(343, 76%, 68%)"
        );
        Ok(())
    }

    #[test]
    fn opacity() -> Result<(), ParseError> {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb_function/50")?,
            "rgba(235, 111, 146, 0.5)"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:hsl_function/50")?,
            "hsla(343, 76%, 68%, 0.5%)"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:hex/100")?,
            "#eb6f92ff"
        );
        assert_eq!(generate_variant(Variant::Moon, "$love:hex/0")?, "#eb6f9200");
        assert_eq!(
            generate_variant(Variant::Moon, "$love:ahex_ns/50")?,
            "80eb6f92"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:ahex_ns/100")?,
            "ffeb6f92"
        );
        Ok(())
    }

    #[test]
    fn role_variation() -> Result<(), ParseError> {
        assert_eq!(generate_variant(Variant::Main, "$(pine|foam)")?, "#31748f");
        assert_eq!(
            generate_variant(Variant::Main, "$(rose|love):hex")?,
            "#ebbcba"
        );
        assert_eq!(
            generate_variant(Variant::Dawn, "$(rose|love):hex")?,
            "#b4637a"
        );

        Ok(())
    }
}
