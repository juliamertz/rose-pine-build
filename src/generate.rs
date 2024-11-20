use colors_transform::AlphaColor;
use regex::Regex;
use std::{num::ParseFloatError, str::FromStr};
use strum::VariantNames;

use crate::{
    palette::{Role, Variant},
    utils::replace_substring,
    Config, Format,
};

#[derive(Debug, PartialEq)]
struct Template {
    role: Role,
    format: Option<Format>,
    opacity: Option<f32>,
}

impl Template {
    fn format_role(&self, variant: Variant, config: &Config) -> String {
        let mut color = self.role.get_color(variant);
        if let Some(opacity) = self.opacity {
            color = color.set_alpha(opacity / 100.0)
        }
        let format = match self.format {
            Some(ref format) => format,
            None => &config.format,
        };

        format.format_color(color, self.opacity)
    }
}

fn parse_template_role(value: &str, variant: Variant) -> Result<Role, strum::ParseError> {
    let role_name = match value.split_once("|") {
        Some((dark, light)) => {
            if variant.is_light() {
                light.strip_suffix(")").expect("Closing suffix")
            } else {
                dark.strip_prefix("(").expect("Opening prefix")
            }
        }
        None => value,
    };

    Role::from_str(role_name.trim())
}

#[derive(Debug)]
pub enum ParseError {
    RoleNotFound,
    FormatNotFound,
    PrefixExpected,
    InvalidOpacity(ParseFloatError),
}

fn parse_template(value: &str, variant: Variant, config: &Config) -> Result<Template, ParseError> {
    let value = match value.strip_prefix(config.prefix) {
        Some(value) => value,
        None => return Err(ParseError::PrefixExpected),
    };

    let (value, opacity) = match value.split_once("/") {
        Some((value, opacity)) => {
            let opacity = opacity
                .parse::<f32>()
                .map(|x| x / 100.0)
                .map_err(ParseError::InvalidOpacity)?;

            (value, Some(opacity))
        }
        None => (value, None),
    };

    let (role, format) = match value.split_once(":") {
        Some((role, format)) => {
            let format = Format::from_str(format.trim()).map_err(|_| ParseError::FormatNotFound)?;
            (role, Some(format))
        }
        None => (value, None),
    };

    Ok(Template {
        role: parse_template_role(role, variant).map_err(|_| ParseError::RoleNotFound)?,
        opacity,
        format,
    })
}

pub fn replace_templates(text: &str, variant: Variant, config: &Config) -> String {
    let mut buffer = text.to_owned();

    let roles = Role::VARIANTS.join("|");
    let formats = Format::VARIANTS.join("|");

    let pattern = format!(
        // this should be considered a crime
        r#"\{}(({roles})|(\(({roles})\|({roles}))\))(:({formats}))?(\/\d{{1,3}})?"#,
        config.prefix,
    );

    Regex::new(&pattern)
        .expect("valid regex pattern")
        .find_iter(text)
        .collect::<Vec<_>>() // HACK: Collect so we can reverse?
        .iter()
        .rev()
        .for_each(|m| {
            let template = match parse_template(m.as_str(), variant, config) {
                Ok(template) => template,
                Err(err) => {
                    eprintln!("unable to parse template, error: {err:?}");
                    return;
                }
            };

            let formatted_color = template.format_role(variant, config);
            buffer = replace_substring(&buffer, m.start(), m.end(), &formatted_color);
        });

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_parsing() -> Result<(), ParseError> {
        let config = &Config::default();
        assert_eq!(
            parse_template("$base:rgb", Variant::Main, config)?,
            Template::new(Role::Base, Some(Format::Rgb), None)
        );
        assert_eq!(
            parse_template("$surface:hsl", Variant::Main, config)?,
            Template::new(Role::Surface, Some(Format::Hsl), None)
        );
        assert_eq!(
            parse_template("$highlightMed:ahex_ns/80", Variant::Main, config)?,
            Template::new(Role::HighlightMed, Some(Format::AhexNs), Some(0.8))
        );
        assert_eq!(
            parse_template("$(foam|pine):hex", Variant::Main, config)?,
            Template::new(Role::Foam, Some(Format::Hex), None)
        );
        assert_eq!(
            parse_template("$(foam|pine):hex", Variant::Dawn, config)?,
            Template::new(Role::Pine, Some(Format::Hex), None)
        );

        Ok(())
    }

    #[test]
    fn format_rgb() {
        assert_eq!(
            replace_templates("$love:rgb", Variant::Moon, &Config::default()),
            "235, 111, 146"
        );
    }

    #[test]
    fn format_parse_order() {
        assert_eq!(
            replace_templates(
                "$love:rgb_function,$love:rgb,$love:hex_ns,$love:hex",
                Variant::Moon,
                &Config::default()
            ),
            "rgb(235, 111, 146),235, 111, 146,EB6F92,#EB6F92"
        );
    }

    #[test]
    fn format_hsl() {
        assert_eq!(
            replace_templates("$love:hsl_function", Variant::Moon, &Config::default()),
            "hsl(343, 76%, 68%)"
        );
    }

    #[test]
    fn opacity() {
        assert_eq!(
            replace_templates("$love:rgb_function/50", Variant::Moon, &Config::default()),
            "rgba(235, 111, 146, 0.5)"
        );
        assert_eq!(
            replace_templates("$love:hsl_function/50", Variant::Moon, &Config::default()),
            "hsla(343, 76%, 68%, 0.5%)"
        );
        assert_eq!(
            replace_templates("$love:hex/100", Variant::Moon, &Config::default()),
            "#EB6F92FF"
        );
        assert_eq!(
            replace_templates("$love:hex/0", Variant::Moon, &Config::default()),
            "#EB6F9200"
        );
        assert_eq!(
            replace_templates("$love:ahex_ns/50", Variant::Moon, &Config::default()),
            "80EB6F92"
        );
        assert_eq!(
            replace_templates("$love:ahex_ns/100", Variant::Moon, &Config::default()),
            "FFEB6F92"
        );
    }

    #[test]
    fn role_variation() -> Result<(), strum::ParseError> {
        assert_eq!(
            parse_template_role("(love|rose)", Variant::Main)?,
            Role::Love
        );
        assert_eq!(
            parse_template_role("(love|rose)", Variant::Moon)?,
            Role::Love
        );
        assert_eq!(
            parse_template_role("(love|rose)", Variant::Dawn)?,
            Role::Rose
        );
        assert_eq!(
            replace_templates("$(pine|foam)", Variant::Main, &Config::default()),
            "#31748F"
        );
        assert_eq!(
            replace_templates("$(rose|love):hex", Variant::Main, &Config::default()),
            "#EBBCBA"
        );
        assert_eq!(
            replace_templates("$(rose|love):hex", Variant::Dawn, &Config::default()),
            "#B4637A"
        );

        Ok(())
    }

    impl Template {
        fn new(role: Role, format: Option<Format>, opacity: Option<f32>) -> Self {
            Self {
                role,
                format,
                opacity,
            }
        }
    }
}
