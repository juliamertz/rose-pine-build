use crate::{
    format::Format,
    generate::{self},
    palette::Variant,
    parse::{self, Delimiter},
};

use clap::Parser;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub parse: parse::ParseOptions,
    pub generate: generate::Options,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, short, default_value = "dist")]
    pub out: PathBuf,

    #[clap(long)]
    pub write_config: bool,

    #[clap(long, short)]
    pub format: Option<Format>,

    #[clap(long, short)]
    pub delimiter: Option<Delimiter>,

    #[clap(long, short)]
    pub seperator: Option<char>,

    #[clap(long, short)]
    pub variant: Option<Variant>,

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
    }
}

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Config {
            parse: parse::ParseOptions {
                prefix: value.prefix.unwrap_or_default(),
                seperator: value.seperator.unwrap_or_default(),
                delimiter: value.delimiter.unwrap_or_default(),
            },
            generate: generate::Options {
                format: value.format.unwrap_or_default(),
                strip_spaces: false,
            },
        }
    }
}

impl Config {
    pub fn read(path: impl AsRef<Path>) -> Result<Config> {
        if path.as_ref().exists() {
            let data = fs::read_to_string(path).expect("to read config");
            let parsed: Config =
                ron::from_str(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            return Ok(parsed);
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "Unable to read config file from default paths",
        ))
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        let config = PrettyConfig::new().escape_strings(false);
        let contents = ron::ser::to_string_pretty(self, config).unwrap();
        fs::write(path, contents)
    }
}
