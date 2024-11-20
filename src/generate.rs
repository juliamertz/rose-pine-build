use std::collections::HashMap;

use regex::Regex;
use strum::{IntoEnumIterator, VariantNames};

use crate::{
    palette::{Role, Variant},
    parse::{self, ParseError},
    utils::{Reversed, Substitutable},
    Config, Format,
};

/// HashMap containing generation output strings for each variant
pub type Outputs = HashMap<Variant, String>;

#[derive(Clone, Debug)]
pub struct Generator {
    config: Config,
    pattern: Regex,
}

impl Generator {
    fn make_pattern(config: &Config) -> String {
        let roles = Role::VARIANTS.join("|");
        // format order is reversed to avoid shorter names matching first
        let formats = Format::VARIANTS.to_vec().reversed().join("|");

        format!(
            // this should be considered a crime
            r#"\{}(({roles})|(\(({roles})\|({roles}))\))(:({formats}))?(\/\d{{1,3}})?"#,
            config.prefix,
        )
    }

    pub fn new(config: Config) -> Self {
        Self {
            pattern: Regex::new(&Self::make_pattern(&config)).expect("valid regex pattern"),
            config,
        }
    }

    pub fn generate_variants(&self, text: &str) -> Result<Outputs, ParseError> {
        let mut outputs = HashMap::new();
        for v in Variant::iter() {
            outputs.insert(v, self.generate_variant(v, text)?);
        }
        Ok(outputs)
    }

    pub fn generate_variant(&self, variant: Variant, text: &str) -> Result<String, ParseError> {
        let captures = self.pattern.find_iter(text).collect::<Vec<_>>();
        let mut buffer = text.to_owned();

        for capture in captures.reversed() {
            let template = parse::parse_template(capture.as_str(), variant, &self.config)?;
            buffer.substitute(
                template.format_role(variant, &self.config),
                capture.start(),
                capture.end(),
            );
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
        GENERATOR.get_or_init(|| Generator::new(Config::default()))
            .generate_variant(variant, text)
    }

    #[test]
    fn format_rgb() -> Result<(), ParseError> {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb")?,
            "235, 111, 146"
        );
        Ok(())
    }

    #[test]
    fn format_parse_order() -> Result<(), ParseError> {
        assert_eq!(
            generate_variant(
                Variant::Moon,
                "$love:rgb_function,$love:rgb,$love:hex_ns,$love:hex",
            )?,
            "rgb(235, 111, 146),235, 111, 146,eb6f92,#eb6f92"
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
