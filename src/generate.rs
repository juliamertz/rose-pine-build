use std::str::FromStr;

use regex::Regex;
use strum::IntoEnumIterator;

use crate::{colors::{Role, Variant}, Config};


#[derive(Debug)]
struct Capture {
    role: Role,
    format: Option<crate::Format>,
    start: usize,
    end: usize,
}

fn parse_capture(m: &regex::Match<'_>, config: &Config) -> Capture {
    let result = m
        .as_str()
        .strip_prefix(config.prefix)
        .expect("capture to start with configured prefix");

    match result.split_once(":") {
        Some((role_name, format_name)) => {
            dbg!(&format_name);
            let format = crate::Format::from_str(format_name.trim()).expect("valid format name");
            dbg!(&format);
            Capture {
                role: Role::from_str(role_name).expect("valid role name"),
                format: Some(format),
                start: m.start(),
                end: m.end(),
            }
        }
        None => Capture {
            role: Role::from_str(result).expect("valid role name"),
            format: None,
            start: m.start(),
            end: m.end(),
        },
    }
}

fn replace_substring(text: &str, start: usize, end: usize, replacement: &str) -> String {
    let (before, after) = text.split_at(start);
    let (_, after_replace) = after.split_at(end - start);

    format!("{}{}{}", before, replacement, after_replace)
}

pub fn replace_templates(text: &str, variant: Variant, config: &Config) -> String {
    let mut buffer = text.to_owned();

    let roles = Role::iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let formats = crate::Format::iter().map(|x| x.to_string()).collect::<Vec<_>>();

    let pattern = format!(
        "\\{}({})(:({}))?",
        config.prefix,
        roles.join("|"),
        formats.join("|")
    );

    Regex::new(&pattern)
        .expect("valid regex pattern")
        .find_iter(text)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .for_each(|m| {
            let capture = parse_capture(m, config);
            let format = match capture.format {
                Some(ref format) => format,
                None => &config.format,
            };
            let color = capture.role.get_color(variant);
            dbg!(&color);
            let formatted_color = format.to_color_string(color, false);
            dbg!(&capture, &color, &formatted_color);
            let replacement =
                replace_substring(&buffer, capture.start, capture.end, &formatted_color);
            buffer = replacement;
        });

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_rgb() {
        assert_eq!(
            replace_templates("$love:rgb_function", Variant::Moon, &Config::default()),
            "rgb(235, 111, 146)"
        );
    }

    #[test]
    fn format_hsl() {
        assert_eq!(
            replace_templates("$love:hsl_function", Variant::Moon, &Config::default()),
            "hsl(343, 76%, 68%)"
        );
    }
}
