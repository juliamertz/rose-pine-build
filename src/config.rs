use crate::{
    format::Format,
    generate::{self},
    palette::Variant,
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
pub struct Args {
    #[clap(long, short, default_value = "dist")]
    pub out: PathBuf,

    #[clap(long, short)]
    pub format: Option<Format>,

    #[clap(long, short)]
    pub delimiter: Option<Delimiter>,

    #[clap(long, short)]
    pub seperator: Option<char>,

    #[clap(long, short)]
    pub variant: Option<Variant>,

    #[clap(long, short)]
    /// render with tera templating engine
    pub tera: bool,

    #[clap(long)]
    /// always add alpha values
    pub force_alpha: bool,

    #[clap(long, short)]
    pub prefix: Option<char>,

    pub template_file: PathBuf,
}

impl Args {
    pub fn no_options(&self) -> bool {
        self.format.is_none()
            && self.delimiter.is_none()
            && self.seperator.is_none()
            && self.variant.is_none()
            && self.prefix.is_none()
            && !self.force_alpha
    }
}

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Config {
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
