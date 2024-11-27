use crate::{
    config::Config,
    format::Format,
    generate::Options,
    palette::{Role, Variant},
    utils::{Case, Casing},
};
use clap::ValueEnum;
use palette::{Color, Metadata, VariantKind};
use serde::Serialize;
use std::{
    fmt::{Debug, Display},
    num::ParseIntError,
    vec,
};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ParseOptions {
    pub prefix: char,
    pub seperator: char,
    pub delimiter: Delimiter,
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum, Default)]
pub enum Delimiter {
    #[default]
    Parenthesis,
    CurlyBracket,
    AngleBracket,
    SqaureBracket,
}

enum Side<T> {
    Open(T),
    Close(T),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Template {
    Metadata(Metadata, Option<Case>),
    Role(RoleCaptures, Option<Format>, Option<u16>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Capture {
    pub template: Template,
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

struct Lexer {
    index: usize,
    content: Vec<char>,
    config: ParseOptions,
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

impl Capture {
    pub fn format(&self, variant: &Variant, options: &Options) -> String {
        match self.template {
            Template::Role(ref role, format, alpha) => {
                let format = match format {
                    Some(ref format) => format,
                    None => &options.format,
                };

                let alpha = if options.force_alpha {
                    Some(alpha.unwrap_or(100))
                } else {
                    alpha
                };

                format.format_color(role.get_color(variant), alpha)
            }
            Template::Metadata(key, case) => {
                let value = key.format(variant);
                match case {
                    Some(case) => value.to_case(case),
                    None => value,
                }
            }
        }
    }
}

impl Delimiter {
    pub fn open(&self) -> char {
        Side::Open(*self).into()
    }
    pub fn close(&self) -> char {
        Side::Close(*self).into()
    }
}

impl From<Side<Delimiter>> for char {
    fn from(value: Side<Delimiter>) -> Self {
        match value {
            Side::Open(delimiter) => match delimiter {
                Delimiter::Parenthesis => '(',
                Delimiter::CurlyBracket => '{',
                Delimiter::AngleBracket => '<',
                Delimiter::SqaureBracket => '[',
            },
            Side::Close(delimiter) => match delimiter {
                Delimiter::Parenthesis => ')',
                Delimiter::CurlyBracket => '}',
                Delimiter::AngleBracket => '>',
                Delimiter::SqaureBracket => ']',
            },
        }
    }
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "index: {}, config: {:?}", self.index, self.config)?;
        writeln!(f, "{}⬇", " ".repeat(self.index))?;
        writeln!(f, "{}", self.content.iter().collect::<String>())
    }
}

impl Lexer {
    fn new(content: &str, config: &Config) -> Self {
        Self {
            index: 0,
            content: content.chars().collect(),
            config: config.parse,
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

fn parse_capture(lexer: &mut Lexer) -> Result<Capture, ParseError> {
    let mut roles = RoleCaptures::new();

    let start = lexer.index;
    if lexer.current() != Some(&lexer.config.prefix) {
        return Err(ParseError::PrefixExpected);
    }
    lexer.advance();

    if let Ok(key) = parse_enum_variant::<Metadata>(lexer, Case::Snake) {
        let format = if lexer.current() == Some(&':') {
            lexer.advance();
            Some(parse_enum_variant::<Case>(lexer, Case::Snake)?)
        } else {
            None
        };

        return Ok(Capture {
            template: Template::Metadata(key, format),
            start,
            end: lexer.index,
        });
    }

    // Grouped roles
    if lexer.current() == Some(&lexer.config.delimiter.open()) {
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

        if lexer.current() != Some(&lexer.config.delimiter.close()) {
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
            match c.is_ascii_digit() && buf.len() < 3 {
                true => {
                    buf.push(*c);
                    if lexer.advance().is_none() {
                        break;
                    };
                }
                false => break,
            };
        }

        let parsed: u16 = buf
            .into_iter()
            .collect::<String>()
            .parse()
            .map_err(ParseError::InvalidOpacity)?;

        Some(parsed)
    } else {
        None
    };

    Ok(Capture {
        template: Template::Role(roles, format, opacity),
        start,
        end: lexer.index,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoleCaptures(Vec<Role>);

impl RoleCaptures {
    fn new() -> Self {
        Self(Vec::with_capacity(3))
    }

    fn push(&mut self, val: Role) {
        if self.0.len() < 3 {
            self.0.push(val)
        }
    }

    fn get_color(&self, variant: &Variant) -> Color {
        match self.0.as_slice() {
            [role] => role,
            [dark, light] => match variant.kind() {
                VariantKind::Light => light,
                VariantKind::Dark => dark,
            },
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
mod test;
