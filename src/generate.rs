use regex::Regex;
use strum::VariantNames;

use crate::{
    palette::{Role, Variant},
    parse::{self, ParseError},
    utils::{Reversed, Substitutable},
    Config, Format,
};

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

    // fn generate_variant(
    //     &self,
    //     variant: Variant,
    //     captures: &[Match<'_>],
    //     templates: &[Template],
    //     text: &str,
    // ) -> Result<String, ParseError> {
    //     let mut buffer = text.to_owned();
    //     for (index, template) in templates.reversed().iter().enumerate() {
    //         let capture = captures
    //             .get(index)
    //             .expect("template to have capture at index");
    //         buffer.gsub(
    //             template.format_role(variant, &self.config),
    //             capture.start(),
    //             capture.end(),
    //         );
    //     }
    //     Ok(buffer)
    // }

    pub fn generate_variant(&self, variant: Variant, text: &str) -> Result<String, ParseError> {
        let captures = self.pattern.find_iter(text).collect::<Vec<_>>();
        let mut buffer = text.to_owned();

        for capture in captures.reversed() {
            let template = parse::parse_template(capture.as_str(), variant, &self.config)?;
            buffer.gsub(
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
    use super::*;

    #[test]
    fn format_rgb() -> Result<(), ParseError> {
        let generator = Generator::new(Config::default());
        assert_eq!(
            generator.generate_variant(Variant::Moon, "$love:rgb")?,
            "235, 111, 146"
        );
        Ok(())
    }

    #[test]
    fn format_parse_order() -> Result<(), ParseError> {
        let generator = Generator::new(Config::default());
        assert_eq!(
            generator.generate_variant(
                Variant::Moon,
                "$love:rgb_function,$love:rgb,$love:hex_ns,$love:hex",
            )?,
            "rgb(235, 111, 146),235, 111, 146,EB6F92,#EB6F92"
        );
        Ok(())
    }

    #[test]
    fn format_hsl() -> Result<(), ParseError> {
        let g = Generator::new(Config::default());
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hsl_function")?,
            "hsl(343, 76%, 68%)"
        );
        Ok(())
    }

    #[test]
    fn opacity() -> Result<(), ParseError> {
        let g = Generator::new(Config::default());
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:rgb_function/50")?,
            "rgba(235, 111, 146, 0.5)"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hsl_function/50")?,
            "hsla(343, 76%, 68%, 0.5%)"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hex/100")?,
            "#EB6F92FF"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:hex/0")?,
            "#EB6F9200"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:ahex_ns/50")?,
            "80EB6F92"
        );
        assert_eq!(
            g.generate_variant(Variant::Moon, "$love:ahex_ns/100")?,
            "FFEB6F92"
        );
        Ok(())
    }

    #[test]
    fn role_variation() -> Result<(), ParseError> {
        let g = Generator::new(Config::default());
        assert_eq!(
            g.generate_variant(Variant::Main, "$(pine|foam)")?,
            "#31748F"
        );
        assert_eq!(
            g.generate_variant(Variant::Main, "$(rose|love):hex")?,
            "#EBBCBA"
        );
        assert_eq!(
            g.generate_variant(Variant::Dawn, "$(rose|love):hex")?,
            "#B4637A"
        );

        Ok(())
    }
}
