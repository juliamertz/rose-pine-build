use colors_transform::AlphaColor;
use std::{num::ParseFloatError, str::FromStr};

use crate::{
    palette::{Role, Variant},
    Config, Format,
};

#[derive(Debug, PartialEq)]
pub struct Template {
    pub role: Role,
    pub format: Option<Format>,
    pub opacity: Option<f32>,
}

impl Template {
    pub fn format_role(&self, variant: Variant, config: &Config) -> String {
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

pub fn parse_template_role(value: &str, variant: Variant) -> Result<Role, strum::ParseError> {
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

pub fn parse_template(
    value: &str,
    variant: Variant,
    config: &Config,
) -> Result<Template, ParseError> {
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
