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

#[derive(Clone, Copy, Debug, Default, Serialize)]
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
    /// path to directory where generated files will be output
    #[clap(long, short, default_value = "dist")]
    pub out: PathBuf,

    #[cfg(feature = "templating")]
    #[clap(long, short)]
    /// render with tera templating engine
    pub tera: bool,

    #[clap(long, short)]
    /// recursively generate templates in source directory
    pub recurse: bool,

    #[clap(long, short, default_value = "hex")]
    /// default color formatting
    pub format: Format,

    #[clap(long, short, default_value = "parenthesis")]
    /// bracket type for role groups
    pub delimiter: Delimiter,

    #[clap(long, short, default_value = "|")]
    /// charachter to use as seperator in role groups
    pub seperator: char,

    #[clap(long)]
    /// always add alpha values
    pub force_alpha: bool,

    #[clap(long, short, default_value = "$")]
    /// variable prefix
    pub prefix: char,

    /// path to template file or directory
    pub template_source: PathBuf,
}

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Config {
            #[cfg(feature = "templating")]
            tera: value.tera,
            parse: parse::ParseOptions {
                prefix: value.prefix,
                seperator: value.seperator,
                delimiter: value.delimiter,
            },
            generate: generate::Options {
                format: value.format,
                strip_spaces: false,
                force_alpha: value.force_alpha,
            },
        }
    }
}
