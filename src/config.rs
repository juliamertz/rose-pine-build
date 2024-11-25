use crate::{
    format::Format,
    generate::{self},
    parse::{self, Delimiter},
};

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Config {
    /// use tera templating engine
    #[cfg(feature = "templating")]
    pub tera: bool,

    pub parse: parse::ParseOptions,
    pub generate: generate::Options,
}

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Magenta.on_default())
        .usage(AnsiColor::Blue.on_default())
        .literal(AnsiColor::White.on_default())
        .placeholder(AnsiColor::Yellow.on_default())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, styles = styles())]
/// Theme generator for Rosé Pine
pub struct Args {
    #[clap(long, short, default_value = "dist")]
    pub out: PathBuf,

    #[cfg(feature = "templating")]
    #[clap(long, short)]
    /// render with tera templating engine
    pub tera: bool,

    #[clap(long, short)]
    /// default color formatting
    pub format: Option<Format>,

    #[clap(long, short)]
    /// bracket type for role groups
    pub delimiter: Option<Delimiter>,

    #[clap(long, short)]
    /// charachter to use as seperator in role groups
    pub seperator: Option<char>,

    #[clap(long)]
    /// always add alpha values
    pub force_alpha: bool,

    #[clap(long, short)]
    /// variable prefix
    pub prefix: Option<char>,

    /// path to template file
    pub template_source: PathBuf,
}

impl Args {
    pub fn no_options(&self) -> bool {
        self.format.is_none()
            && self.delimiter.is_none()
            && self.seperator.is_none()
            && self.prefix.is_none()
            && !self.force_alpha
    }
}

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Config {
            #[cfg(feature = "templating")]
            tera: value.tera,
            parse: parse::ParseOptions {
                prefix: value.prefix.unwrap_or('$'),
                seperator: value.seperator.unwrap_or('|'),
                delimiter: value.delimiter.unwrap_or(Delimiter::Parenthesis),
            },
            generate: generate::Options {
                format: value.format.unwrap_or(Format::Hex),
                strip_spaces: false,
                force_alpha: value.force_alpha,
            },
        }
    }
}
