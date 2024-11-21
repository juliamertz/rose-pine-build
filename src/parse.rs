use std::{fmt::Display, num::ParseIntError, vec};
use strum::{IntoEnumIterator, VariantNames};

use crate::{
    casing::{Case, Casing},
    format::Format,
    generate::Config,
    palette::{transform::Rgb, Role, Variant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantRoles {
    roles: Vec<Role>,
}

#[derive(Debug, PartialEq)]
pub struct Capture {
    pub role: VariantRoles,
    pub format: Option<Format>,
    pub opacity: Option<u16>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum ParseError {
    VariantNotFound,
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

impl Parser {
    fn new(content: &str, config: &Config) -> Self {
        let mut parser = Self {
            index: None,
            content: content.chars().collect(),
            config: config.clone(),
        };
        parser.advance();
        parser
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

        let index = self.index.unwrap();
        if index >= self.content.len() - 1 {
            return None;
        }

        self.index = Some(index + 1);
        self.index
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ') = self.lookahead() {
            self.advance();
        }
    }

    fn lookahead(&self) -> Option<&char> {
        self.index.and_then(|i| self.content.get(i + 1))
    }

    fn lookahead_n(&self, n: usize) -> Option<&char> {
        self.index.and_then(|i| self.content.get(i + n))
    }

    fn match_ahead(&self, pattern: &str) -> bool {
        for (i, a) in pattern.char_indices() {
            if let Some(b) = self.lookahead_n(i) {
                if a != *b {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn parse_enumvalue<T>(&mut self, case: Case) -> Result<T, ParseError>
    where
        T: VariantNames + IntoEnumIterator + Display,
    {
        let val = match T::iter()
            .rev()
            .find(|v| self.match_ahead(&v.to_string().to_case(case)))
        {
            Some(v) => {
                self.advance_n(v.to_string().len() - 1);
                v
            }
            None => return Err(ParseError::VariantNotFound),
        };

        Ok(val)
    }
}

pub fn parse_template(content: &str, config: &Config) -> Vec<Result<Capture, ParseError>> {
    let mut parser = Parser::new(content, config);
    let mut captures = vec![];

    while parser.lookahead().is_some() {
        if parser.current() == Some(&config.prefix) {
            captures.push(parse_capture(&mut parser));
        }

        parser.advance();
    }

    captures
}

fn parse_capture(p: &mut Parser) -> Result<Capture, ParseError> {
    let mut roles = VariantRoles::new();
    let start = p.index.expect("index to be set");

    if p.current() != Some(&p.config.prefix) {
        return Err(ParseError::PrefixExpected);
    }
    p.advance();

    // Grouped roles
    if p.current() == Some(&p.config.delimiter.open()) {
        p.skip_whitespace();
        p.advance();
        roles.push(p.parse_enumvalue(Case::Snake)?);
        p.skip_whitespace();

        if p.lookahead() == Some(&p.config.seperator) {
            p.advance();
            p.skip_whitespace();
            p.advance();
            roles.push(p.parse_enumvalue(Case::Snake)?);
            p.skip_whitespace();

            if p.lookahead() == Some(&p.config.seperator) {
                p.advance();
                p.skip_whitespace();
                p.advance();
                roles.push(p.parse_enumvalue(Case::Snake)?);
            }
            p.skip_whitespace();
        }

        if p.lookahead() != Some(&p.config.delimiter.close()) {
            return Err(ParseError::CloseParenExpected);
        }

        p.advance();
    }
    // Role name without group
    else {
        roles.push(p.parse_enumvalue(Case::Snake)?);
    }

    let format = if p.lookahead() == Some(&':') {
        p.advance_n(2);
        Some(p.parse_enumvalue(Case::Snake)?)
    } else {
        None
    };

    let opacity = if p.lookahead() == Some(&'/') {
        p.advance();
        let mut buf: Vec<char> = vec![];

        while let Some(c) = p.lookahead() {
            if c.is_ascii_digit() && buf.len() < 3 {
                buf.push(*c);
                p.advance();
            } else {
                break;
            }
        }

        let parsed: u16 = buf
            .into_iter()
            .collect::<String>()
            .parse::<u16>()
            .map_err(ParseError::InvalidOpacity)?;

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

impl Capture {
    pub fn format_role(&self, variant: Variant, config: &Config) -> String {
        let format = match self.format {
            Some(ref format) => format,
            None => &config.format,
        };

        format.format_color(self.role.get_color(&variant), self.opacity)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_variants() {
        assert_capture("$(pine)", Capture::new(vec![Role::Pine], None, None));
        assert_capture(
            "$(rose|love)",
            Capture::new(vec![Role::Rose, Role::Love], None, None),
        );
        assert_capture(
            "$(foam|pine|iris)",
            Capture::new(vec![Role::Foam, Role::Pine, Role::Iris], None, None),
        );
    }

    #[test]
    fn role_variants_whitespace() {
        assert_capture("$( pine )", Capture::new(vec![Role::Pine], None, None));
        assert_capture(
            "$( rose | love )",
            Capture::new(vec![Role::Rose, Role::Love], None, None),
        );
        assert_capture(
            "$( foam | pine | iris )",
            Capture::new(vec![Role::Foam, Role::Pine, Role::Iris], None, None),
        );
        assert_capture(
            "$(    rose   |   love )",
            Capture::new(vec![Role::Rose, Role::Love], None, None),
        );
    }

    #[test]
    fn format() {
        assert_capture(
            "$base:rgb",
            Capture::new(vec![Role::Base], Some(Format::Rgb), None),
        );
        assert_capture(
            "$base:rgb_ns",
            Capture::new(vec![Role::Base], Some(Format::RgbNs), None),
        );
        assert_capture(
            "$base:rgb_function",
            Capture::new(vec![Role::Base], Some(Format::RgbFunction), None),
        );
        assert_capture(
            "$base:rgb_array",
            Capture::new(vec![Role::Base], Some(Format::RgbArray), None),
        );
        assert_capture(
            "$base:rgb_ansi",
            Capture::new(vec![Role::Base], Some(Format::RgbAnsi), None),
        );
        assert_capture(
            "$base:hsl",
            Capture::new(vec![Role::Base], Some(Format::Hsl), None),
        );
        assert_capture(
            "$base:hsl_ns",
            Capture::new(vec![Role::Base], Some(Format::HslNs), None),
        );
        assert_capture(
            "$base:hsl_function",
            Capture::new(vec![Role::Base], Some(Format::HslFunction), None),
        );
        assert_capture(
            "$base:hsl_array",
            Capture::new(vec![Role::Base], Some(Format::HslArray), None),
        );
        assert_capture(
            "$base:hex",
            Capture::new(vec![Role::Base], Some(Format::Hex), None),
        );
        assert_capture(
            "$base:ahex",
            Capture::new(vec![Role::Base], Some(Format::Ahex), None),
        );
        assert_capture(
            "$base:hex_ns",
            Capture::new(vec![Role::Base], Some(Format::HexNs), None),
        );
        assert_capture(
            "$base:ahex_ns",
            Capture::new(vec![Role::Base], Some(Format::AhexNs), None),
        );
    }

    #[test]
    fn opacity() {
        assert_capture("$base/100", Capture::new(vec![Role::Base], None, Some(100)));
        assert_capture("$base/50", Capture::new(vec![Role::Base], None, Some(50)));
        assert_capture("$base/0", Capture::new(vec![Role::Base], None, Some(0)));
        assert_capture(
            "$base:rgb_function/75",
            Capture::new(vec![Role::Base], Some(Format::RgbFunction), Some(75)),
        );
    }

    fn assert_capture(content: &str, correct: Capture) {
        let config = Config::default();
        let mut parser = Parser::new(content, &config);
        match super::parse_capture(&mut parser) {
            Ok(mut capture) => {
                // reset positions for testing purposes
                capture.start = 0;
                capture.end = 0;
                assert_eq!(capture, correct)
            }
            Err(e) => {
                panic!("Unable to parse capture {correct:?}, error: {e:?}")
            }
        }
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
