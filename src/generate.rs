use colors_transform::AlphaColor;
use regex::Regex;
use std::str::FromStr;
use strum::VariantNames;

use crate::{
    palette::{Role, Variant},
    utils::replace_substring,
    Config, Format,
};

#[derive(Debug)]
struct Capture {
    role: Role,
    format: Option<Format>,
    opacity: Option<f32>,
    start: usize,
    end: usize,
}

impl Capture {
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

fn parse_capture_role(value: &str, variant: Variant) -> Role {
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

    Role::from_str(role_name.trim()).expect("valid role name")
}

fn parse_capture(m: &regex::Match<'_>, variant: Variant, config: &Config) -> Capture {
    let result = m
        .as_str()
        .strip_prefix(config.prefix)
        .expect("capture to start with configured prefix");

    let (ident, opacity) = match result.split_once("/") {
        Some((format, opacity)) => (format, opacity.parse::<f32>().ok()),
        None => (result, None),
    };
    let (role, format) = match ident.split_once(":") {
        Some((role, format)) => (
            role,
            Some(Format::from_str(format.trim()).expect("valid format name")),
        ),
        None => (result, None),
    };

    Capture {
        role: parse_capture_role(role, variant),
        opacity,
        start: m.start(),
        end: m.end(),
        format,
    }
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
            let capture = parse_capture(m, variant, config);
            let formatted_color = capture.format_role(variant, config);
            buffer = replace_substring(&buffer, capture.start, capture.end, &formatted_color);
        });

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

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
            replace_templates("$love:hex/100", Variant::Moon, &Config::default()),
            "#EB6F92FF"
        );
        assert_eq!(
            replace_templates("$love:hex/0", Variant::Moon, &Config::default()),
            "#EB6F9200"
        );
        assert_eq!(
            replace_templates("$love:ahex_ns/100", Variant::Moon, &Config::default()),
            "FFEB6F92"
        );
    }

    #[test]
    fn role_variation() {
        assert_eq!(parse_capture_role("(love|rose)", Variant::Main), Role::Love);
        assert_eq!(parse_capture_role("(love|rose)", Variant::Moon), Role::Love);
        assert_eq!(parse_capture_role("(love|rose)", Variant::Dawn), Role::Rose);
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
    }
}
