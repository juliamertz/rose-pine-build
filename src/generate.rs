use std::str::FromStr;

use regex::Regex;
use strum::IntoEnumIterator;

use crate::{
    colors::{Role, Variant},
    Config, Format,
};

#[derive(Debug)]
struct Capture {
    role: Role,
    format: Option<Format>,
    start: usize,
    end: usize,
}

fn parse_capture_role(value: &str, variant: Variant) -> Role {
    let role_name = match value.split_once("|") {
        Some((light, dark)) => {
            if variant.is_light() {
                light.strip_prefix("(").expect("Opening prefix")
            } else {
                dark.strip_suffix(")").expect("Closing suffix")
            }
        }
        None => value,
    };

    dbg!(&role_name);
    Role::from_str(role_name.trim()).expect("valid role name")
}

fn parse_capture(m: &regex::Match<'_>, variant: Variant, config: &Config) -> Capture {
    let result = m
        .as_str()
        .strip_prefix(config.prefix)
        .expect("capture to start with configured prefix");

    let (role, format) = match result.split_once(":") {
        Some((role, format)) => (
            role,
            Some(Format::from_str(format.trim()).expect("valid format name")),
        ),
        None => (result, None),
    };

    Capture {
        role: parse_capture_role(role, variant),
        start: m.start(),
        end: m.end(),
        format,
    }
}

fn replace_substring(text: &str, start: usize, end: usize, replacement: &str) -> String {
    let (before, after) = text.split_at(start);
    let (_, after_replace) = after.split_at(end - start);

    format!("{}{}{}", before, replacement, after_replace)
}

pub fn replace_templates(text: &str, variant: Variant, config: &Config) -> String {
    let mut buffer = text.to_owned();

    let roles = Role::iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let formats = Format::iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("|");

    let pattern = format!(
        // this should be considered a crime
        r#"\{}(({roles})|(\(({roles})\|({roles}))\))(:({formats}))?"#,
        config.prefix,
    );

    Regex::new(&pattern)
        .expect("valid regex pattern")
        .find_iter(text)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .for_each(|m| {
            let capture = parse_capture(m, variant, config);
            let format = match capture.format {
                Some(ref format) => format,
                None => &config.format,
            };

            let color = capture.role.get_color(variant);
            let formatted_color = format.to_color_string(color, false);
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
    fn role_variation() {
        assert_eq!(parse_capture_role("(love|rose)", Variant::Main), Role::Rose);
        assert_eq!(parse_capture_role("(love|rose)", Variant::Moon), Role::Rose);
        assert_eq!(parse_capture_role("(love|rose)", Variant::Dawn), Role::Love);
        assert_eq!(
            replace_templates("$(foam|pine)", Variant::Main, &Config::default()),
            "#31748F"
        );
        assert_eq!(
            replace_templates("$(love|rose):hex", Variant::Main, &Config::default()),
            "#EBBCBA"
        );
        assert_eq!(
            replace_templates("$(love|rose):hex", Variant::Dawn, &Config::default()),
            "#B4637A"
        );
    }
}
