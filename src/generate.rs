use regex::Regex;
use strum::VariantNames;

use crate::{
    palette::{Role, Variant},
    parse::{self},
    utils::{replace_substring, Reversed},
    Config, Format,
};

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

    pub fn generate_variant(&self, variant: Variant, text: &str) -> String {
        let mut buffer = text.to_owned();

        let captures = self.pattern.find_iter(text).collect::<Vec<_>>().reversed();
        for capture in captures {
            let template = match parse::parse_template(capture.as_str(), variant, &self.config) {
                Ok(template) => template,
                Err(err) => {
                    eprintln!("unable to parse template, error: {err:?}");
                    continue;
                }
            };

            let formatted_color = template.format_role(variant, &self.config);
            buffer = replace_substring(&buffer, capture.start(), capture.end(), &formatted_color);
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_rgb() {
        let generator = Generator::new(Config::default());
        assert_eq!(
            generator.generate_variant(Variant::Moon, "$love:rgb"),
            "235, 111, 146"
        );
    }

    #[test]
    fn format_parse_order() {
        let generator = Generator::new(Config::default());
        assert_eq!(
            generator.generate_variant(
                Variant::Moon,
                "$love:rgb_function,$love:rgb,$love:hex_ns,$love:hex",
            ),
            "rgb(235, 111, 146),235, 111, 146,EB6F92,#EB6F92"
        );
    }

    #[test]
    fn format_hsl() {
        let g = Generator::new(Config::default());
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hsl_function"),
            "hsl(343, 76%, 68%)"
        );
    }

    #[test]
    fn opacity() {
        let g = Generator::new(Config::default());
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:rgb_function/50"),
            "rgba(235, 111, 146, 0.5)"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hsl_function/50"),
            "hsla(343, 76%, 68%, 0.5%)"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hex/100"),
            "#EB6F92FF"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hex/0"),
            "#EB6F9200"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:ahex_ns/50"),
            "80EB6F92"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:ahex_ns/100"),
            "FFEB6F92"
        );
    }

    #[test]
    fn role_variation() -> Result<(), strum::ParseError> {
        let g = Generator::new(Config::default());
        assert_eq!(g.generate_variant(Variant::Main, "$(pine|foam)"), "#31748F");
        assert_eq!(
            g.generate_variant(Variant::Main, "$(rose|love):hex"),
            "#EBBCBA"
        );
        assert_eq!(
            g.generate_variant(Variant::Dawn, "$(rose|love):hex"),
            "#B4637A"
        );

        Ok(())
    }
}
