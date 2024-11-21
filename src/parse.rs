use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
    vec,
};

use colors_transform::Rgb;
use heck::{ToLowerCamelCase, ToSnakeCase};
use strum::{IntoEnumIterator, VariantNames};

use crate::{
    generate::{Config, Format},
    palette::{Role, Variant},
};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum VariantRoles {
//     /// Role for each variant
//     All(Role),
//     /// Seperate roles for light & dark variants
//     DarkLight(Role, Role),
//     /// Each variant gets its own role
//     Seperate(Role, Role, Role),
// }

// impl VariantRoles {
//     pub fn get_color(&self, variant: &Variant) -> Rgb {
//         match self {
//             Self::All(role) => role,
//             Self::DarkLight(dark, light) => {
//                 if variant.is_dark() {
//                     dark
//                 } else {
//                     light
//                 }
//             }
//             Self::Seperate(main, moon, dawn) => match variant {
//                 Variant::Main => main,
//                 Variant::Moon => moon,
//                 Variant::Dawn => dawn,
//             },
//         }
//         .get_color(variant)
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantRoles {
    roles: Vec<Role>,
}

impl VariantRoles {
    fn new() -> Self {
        Self {
            roles: Vec::with_capacity(3),
        }
    }

    fn push(&mut self, val: Role) {
        if self.roles.len() < 3 {
            self.roles.push(val)
        }
    }

    fn get_color(&self, variant: &Variant) -> Rgb {
        match self.roles.as_slice() {
            [role] => role,
            [dark, light] => {
                if variant.is_dark() {
                    dark
                } else {
                    light
                }
            }
            [main, moon, dawn] => match variant {
                Variant::Main => main,
                Variant::Moon => moon,
                Variant::Dawn => dawn,
            },
            _ => unreachable!(),
        }
        .get_color(variant)
    }
}

#[derive(Debug, PartialEq)]
pub struct Capture {
    pub role: VariantRoles,
    pub format: Option<Format>,
    pub opacity: Option<u16>,
    pub start: usize,
    pub end: usize,
}

impl Capture {
    pub fn format_role(&self, variant: Variant, config: &Config) -> String {
        let format = match self.format {
            Some(ref format) => format,
            None => &config.format,
        };

        format.format_color(self.role.get_color(&variant), self.opacity)
    }
}

pub fn parse_capture_role(value: &str, variant: Variant) -> Result<Role, ParseError> {
    let role_name = match value.split_once("|") {
        Some((dark, light)) => {
            if variant.is_light() {
                light
                    .strip_suffix(")")
                    .ok_or(ParseError::CloseParenExpected)?
            } else {
                dark.strip_prefix("(")
                    .ok_or(ParseError::OpenParenExpected)?
            }
        }
        None => value,
    };

    Role::from_str(role_name.trim()).map_err(|_| ParseError::RoleNotFound)
}

#[derive(Debug)]
pub enum ParseError {
    RoleNotFound,
    FormatNotFound,
    PrefixExpected,
    OpenParenExpected,
    CloseParenExpected,
    InvalidOpacity(ParseIntError),
}

#[derive(Debug)]
struct Parser {
    index: Option<usize>,
    content: Vec<char>,
    config: Config,
}

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "At index: {:?}, current char: {:?}, next char: {:?}",
            self.index,
            self.current(),
            self.lookhead()
        )?;
        writeln!(f, "{:?}", self.content)
    }
}

impl Parser {
    fn new(content: &str, config: &Config) -> Self {
        let mut p = Self {
            index: None,
            content: content.chars().collect(),
            config: config.clone(),
        };
        p.advance();
        p
    }

    fn current(&self) -> Option<&char> {
        self.index.and_then(|i| self.content.get(i))
    }

    fn advance_n(&mut self, n: usize) {
        self.index = Some(self.index.unwrap_or_default() + n)
    }

    fn advance(&mut self) -> Option<usize> {
        if self.index.is_none() {
            self.index = Some(0);
            return self.index;
        }

        let index = self.index.expect("index to be initialized");
        if index >= self.content.len() - 1 {
            return None;
        }

        self.index = Some(index + 1);
        self.index
    }

    fn lookhead(&self) -> Option<&char> {
        self.index.and_then(|i| self.content.get(i + 1))
    }

    fn lookhead_n(&self, n: usize) -> Option<&char> {
        self.index.and_then(|i| self.content.get(i + n))
    }

    fn matchahead(&self, pattern: &str) -> bool {
        for (i, a) in pattern.char_indices() {
            if let Some(b) = self.lookhead_n(i) {
                if a != *b {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn match_format(&mut self) -> Option<Format> {
        Format::iter()
            .rev()
            .find(|v| self.matchahead(&v.to_string().to_snake_case()))
    }
}

fn scan_role(p: &mut Parser) -> Result<Role, ParseError> {
    let role = match Role::iter().find(|v| p.matchahead(&v.to_string().to_lower_camel_case())) {
        Some(v) => {
            p.advance_n(v.to_string().len() - 1);
            v
        }
        None => return Err(ParseError::RoleNotFound),
    };

    Ok(role)
}

fn parse_capture(p: &mut Parser, config: &Config) -> Result<Capture, ParseError> {
    let mut roles = VariantRoles::new();
    let start = p.index.expect("index to be set");

    if p.current() != Some(&config.prefix) {
        return Err(ParseError::PrefixExpected);
    }
    p.advance();

    if p.current() == Some(&'(') {
        p.advance();
        roles.push(scan_role(p)?);

        if p.lookhead() == Some(&'|') {
            p.advance_n(2);
            roles.push(scan_role(p)?);

            if p.lookhead() == Some(&'|') {
                p.advance_n(2);
                roles.push(scan_role(p)?);
            }
        }

        if p.lookhead() != Some(&')') {
            return Err(ParseError::CloseParenExpected);
        }

        p.advance();
    } else {
        roles.push(scan_role(p)?);
    }

    let format = if p.lookhead() == Some(&':') {
        p.advance_n(2);
        p.match_format().inspect(|format| {
            p.advance_n(format.to_string().len() - 1);
        })
    } else {
        None
    };

    dbg!(format);

    let opacity = if p.lookhead() == Some(&'/') {
        p.advance();
        let mut buf: Vec<char> = vec![];

        while let Some(c) = p.lookhead() {
            if c.is_ascii_digit() && buf.len() < 3 {
                buf.push(*c);
                p.advance();
            } else {
                break;
            }
        }

        let joined = buf.into_iter().collect::<String>();
        let parsed: u16 = joined.parse::<u16>().map_err(ParseError::InvalidOpacity)?;
        Some(parsed)
    } else {
        None
    };

    Ok(Capture {
        role: roles,
        format,
        opacity,
        start,
        end: p.index.unwrap() + 1,
    })
}

pub fn parse_template(content: &str, config: &Config) -> Vec<Result<Capture, ParseError>> {
    let mut parser = Parser::new(content, &Config::default());
    let mut captures = vec![];

    while parser.lookhead().is_some() {
        if parser.current() == Some(&config.prefix) {
            let capture = parse_capture(&mut parser, config);
            captures.push(capture);
        }

        parser.advance();
    }

    captures
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_capture(content: &str) -> Result<Capture, ParseError> {
        let config = Config::default();
        let mut parser = Parser::new(content, &config);
        let capture = super::parse_capture(&mut parser, &config)?;
        Ok(capture)
    }

    #[test]
    fn template_parsing() -> Result<(), ParseError> {
        let asserts = [
            (
                "$base:rgb",
                Capture::new(vec![Role::Base], Some(Format::Rgb), None),
            ),
            (
                "$surface:hsl",
                Capture::new(vec![Role::Surface], Some(Format::Hsl), None),
            ),
            (
                "$highlightMed:ahex_ns/80",
                Capture::new(vec![Role::HighlightMed], Some(Format::AhexNs), Some(80)),
            ),
            (
                "$(foam|pine):hex",
                Capture::new(vec![Role::Foam, Role::Pine], Some(Format::Hex), None),
            ),
            (
                "$(rose|love):hsl/50",
                Capture::new(vec![Role::Rose, Role::Love], Some(Format::Hsl), Some(50)),
            ),
            (
                "$(iris|foam|pine):hsl_function/75",
                Capture::new(
                    vec![Role::Iris, Role::Foam, Role::Pine],
                    Some(Format::HslFunction),
                    Some(75),
                ),
            ),
        ];

        for (template, correct) in asserts {
            match parse_capture(template) {
                Ok(mut capture) => {
                    // reset positions for testing purposes
                    capture.start = 0;
                    capture.end = 0;
                    assert_eq!(capture, correct)
                }
                Err(e) => {
                    eprintln!("Unable to parse capture {correct:?}, error: {e:?}");
                    return Err(ParseError::OpenParenExpected);
                }
            }
        }

        Ok(())
    }

    impl Capture {
        fn new(roles: Vec<Role>, format: Option<Format>, opacity: Option<u16>) -> Self {
            Self {
                role: VariantRoles { roles },
                format,
                opacity,
                start: 0,
                end: 0,
            }
        }
    }
}
