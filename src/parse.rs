use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    num::ParseIntError,
    vec,
};
use strum::IntoEnumIterator;

use crate::{
    config::Config,
    format::Format,
    palette::{transform::Rgb, Role, Variant},
    utils::{Case, Casing},
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum, Default)]
pub enum Delimiter {
    #[default]
    Parenthesis,
    CurlyBracket,
    AngleBracket,
    SqaureBracket,
}

impl Delimiter {
    pub fn left(&self) -> char {
        Side::Left(*self).into()
    }
    pub fn right(&self) -> char {
        Side::Right(*self).into()
    }
}

pub enum Side<T> {
    Left(T),
    Right(T),
}

impl From<Side<Delimiter>> for char {
    fn from(value: Side<Delimiter>) -> Self {
        match value {
            Side::Left(delimiter) => match delimiter {
                Delimiter::Parenthesis => '(',
                Delimiter::CurlyBracket => '{',
                Delimiter::AngleBracket => '<',
                Delimiter::SqaureBracket => '[',
            },
            Side::Right(delimiter) => match delimiter {
                Delimiter::Parenthesis => ')',
                Delimiter::CurlyBracket => '}',
                Delimiter::AngleBracket => '>',
                Delimiter::SqaureBracket => ']',
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParseOptions {
    pub prefix: char,
    pub seperator: char,
    pub delimiter: Delimiter,
}

impl ParseOptions {
    pub fn new(prefix: char, seperator: char, delimiter: Delimiter) -> Self {
        Self {
            prefix,
            seperator,
            delimiter,
        }
    }
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            delimiter: Delimiter::Parenthesis,
            prefix: '$',
            seperator: '|',
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Capture {
    pub role: RoleVariants,
    pub format: Option<Format>,
    pub opacity: Option<u16>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum ParseError {
    VariantNotFound,
    PrefixExpected,
    OpenDelimExpected,
    CloseDelimExpected,
    InvalidOpacity(ParseIntError),
}

pub(crate) struct Lexer {
    index: usize,
    content: Vec<char>,
    config: ParseOptions,
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "index: {}, config: {:?}",
            self.index,
            self.config
        )?;
        writeln!(f, "{}⬇", " ".repeat(self.index))?;
        writeln!(f, "{}", self.content.iter().collect::<String>())
    }
}

impl Lexer {
    pub(crate) fn new(content: &str, config: &Config) -> Self {
         Self {
            index: 0,
            content: content.chars().collect(),
            config: config.parse.clone(),
        }
    }


    fn current(&self) -> Option<&char> {
        self.content.get(self.index)
    }

    fn advance_n(&mut self, n: usize) {
        self.index += n
    }

    fn advance(&mut self) -> Option<usize> {
        if self.index >= self.content.len() - 1 {
            return None;
        }

        self.index += 1;
        Some(self.index)
    }

    /// Advances while the current char is whitespace
    fn skip_whitespace(&mut self) {
        while let Some(' ') = self.current() {
            self.advance();
        }
    }

    /// Returns char relative to the parsers current position
    fn lookahead(&self) -> Option<&char> {
        self.content.get(self.index + 1)
    }

    /// Returns char relative to the parsers current position
    fn lookahead_n(&self, n: usize) -> Option<&char> {
        self.content.get(self.index + n)
    }

    /// Checks if pattern is present starting at the current position of the parser
    fn scan_ahead(&self, pattern: &str) -> bool {
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

    /// Looks ahead for an enum variant match
    /// Returns an option of the enum and the matched length
    fn scan_enum_variant<T>(&mut self, case: Case) -> Option<(T, usize)>
    where
        T: IntoEnumIterator + Display + Copy,
    {
        T::iter()
            .rev()
            .find(|v| self.scan_ahead(&v.to_case(case)))
            .map(|v| (v, v.to_case(case).len()))
    }
}

pub fn parse_template(content: &str, config: &Config) -> Vec<Result<Capture, ParseError>> {
    let mut lexer = Lexer::new(content, config);
    let mut captures = vec![];

    while lexer.lookahead().is_some() {
        if lexer.current() == Some(&config.parse.prefix) {
            captures.push(parse_capture(&mut lexer));
        }

        lexer.advance();
    }

    captures
}

fn parse_enum_variant<T>(lexer: &mut Lexer, case: Case) -> Result<T, ParseError>
where
    T: IntoEnumIterator + Display + Copy,
{
    match lexer.scan_enum_variant(case) {
        Some((variant, length)) => {
            lexer.advance_n(length);
            Ok(variant)
        }
        None => Err(ParseError::VariantNotFound),
    }
}

pub(crate) fn parse_capture(lexer: &mut Lexer) -> Result<Capture, ParseError> {
    let mut roles = RoleVariants::new();

    let start = lexer.index;
    if lexer.current() != Some(&lexer.config.prefix) {
        return Err(ParseError::PrefixExpected);
    }
    lexer.advance();

    // Grouped roles
    if lexer.current() == Some(&lexer.config.delimiter.left()) {
        lexer.advance();
        lexer.skip_whitespace();
        roles.push(parse_enum_variant(lexer, Case::Snake)?);
        lexer.skip_whitespace();

        if lexer.current() == Some(&lexer.config.seperator) {
            lexer.advance();
            lexer.skip_whitespace();
            roles.push(parse_enum_variant(lexer, Case::Snake)?);
            lexer.skip_whitespace();

            if lexer.current() == Some(&lexer.config.seperator) {
                lexer.advance();
                lexer.skip_whitespace();
                roles.push(parse_enum_variant(lexer, Case::Snake)?);
            }
            lexer.skip_whitespace();
        }

        if lexer.current() != Some(&lexer.config.delimiter.right()) {
            return Err(ParseError::CloseDelimExpected);
        }
        lexer.advance();
    }
    // Role name without group
    else {
        roles.push(parse_enum_variant(lexer, Case::Snake)?);
    }

    let format = if lexer.current() == Some(&':') {
        lexer.advance();
        Some(parse_enum_variant::<Format>(lexer, Case::Snake)?)
    } else {
        None
    };

    let opacity = if lexer.current() == Some(&'/') {
        lexer.advance();
        let mut buf: Vec<char> = vec![];

        while let Some(c) = lexer.current() {
            if c.is_ascii_digit() && buf.len() < 3 {
                buf.push(*c);
                if lexer.advance().is_none() {
                    break;
                };
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
        end: lexer.index,
    })
}

impl Capture {
    pub fn format_role(&self, variant: Variant, config: &Config) -> String {
        let format = match self.format {
            Some(ref format) => format,
            None => &config.generate.format,
        };

        format.format_color(self.role.get_color(&variant), self.opacity)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoleVariants {
    pub roles: Vec<Role>,
}

impl RoleVariants {
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
