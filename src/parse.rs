use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
    vec,
};

use heck::{ToLowerCamelCase, ToSnakeCase};
use strum::{IntoEnumIterator, VariantNames};

use crate::{
    generate::{Config, Format},
    palette::{Role, Variant},
};

pub enum RoleCapture {
    /// Role for each variant
    All(Role),
    /// Seperate roles for light & dark variants
    DarkLight(Role,Role),
    /// Each variant has gets its own role
    Seperate(Role,Role,Role),
}

#[derive(Debug, PartialEq)]
pub struct Capture {
    pub role: Role,
    pub format: Option<Format>,
    pub opacity: Option<u16>,
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

    fn match_role(&self) -> Option<Role> {
        dbg!("match_role line :137");
        dbg!(self, self.lookhead());
        Role::iter().find(|v| self.matchahead(&v.to_string().to_lower_camel_case()))
    }

    fn match_format(&mut self) -> Option<Format> {
        Format::iter().find(|v| self.matchahead(&v.to_string().to_snake_case()))
    }
}

fn parse_capture(p: &mut Parser, _config: &Config) -> Result<Capture, ParseError> {
    if p.current() == Some(&'(') {}
    let role = match p.match_role() {
        Some(v) => v,
        None => return Err(ParseError::RoleNotFound),
    };
    p.advance_n(role.to_string().len() - 1);

    let format = if p.lookhead() == Some(&':') {
        p.advance_n(2);
        println!("{p}");
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
        role,
        format,
        opacity,
    })
}

pub fn parse_template(content: &str, config: &Config) -> Vec<Result<Capture, ParseError>> {
    let mut captures = vec![];
    let mut parser = Parser::new("$love:rgb/80", &Config::default());

    while let Some(c) = parser.lookhead() {
        dbg!(&c);

        if parser.current() == Some(&'$') {
            parser.advance();
            let capture = parse_capture(&mut parser, config);
            dbg!(capture);
        }

        parser.advance();
    }
    // dbg!(&captures);

    captures
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parsing() -> Result<(), ParseError> {
//         parse_template("", &Config::default());
//         panic!("aah");
//
//         Ok(())
//     }
// }

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
                "base:rgb",
                Capture::new(Role::Base, Some(Format::Rgb), None),
            ),
            (
                "$surface:hsl",
                Capture::new(Role::Surface, Some(Format::Hsl), None),
            ),
            (
                "$highlightMed:ahex_ns/80",
                Capture::new(Role::HighlightMed, Some(Format::AhexNs), Some(80)),
            ),
            (
                "$(foam|pine):hex",
                Capture::new(Role::Foam, Some(Format::Hex), None),
            ),
            (
                "$(rose|love):hsl/50",
                Capture::new(Role::Foam, Some(Format::Hex), None),
            ),
        ];

        for (template, correct) in asserts {
            match parse_capture(template) {
                Ok(capture) => assert_eq!(capture, correct,),
                Err(e) => {
                    eprintln!("Unable to parse capture {correct:?}, error: {e:?}");
                    return Err(ParseError::OpenParenExpected);
                }
            }
        }

        Ok(())
    }

    impl Capture {
        fn new(role: Role, format: Option<Format>, opacity: Option<u16>) -> Self {
            Self {
                role,
                format,
                opacity,
            }
        }
    }
}
