use crate::{
    format::Format,
    generate::{self},
    palette::Variant,
    parse::{self, Delimiter},
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub parse: parse::ParseOptions,
    pub generate: generate::GenerateOptions,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, short, default_value = "dist")]
    pub out_dir: PathBuf,

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

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Config {
            parse: parse::ParseOptions {
                prefix: value.prefix.unwrap_or_default(),
                seperator: value.seperator.unwrap_or_default(),
                delimiter: value.delimiter.unwrap_or_default(),
            },
            generate: generate::GenerateOptions {
                format: value.format.unwrap_or_default(),
                strip_spaces: false,
            },
        }
    }
}

static CONFIG_PATHS: [&str; 2] = [".rose-pine.toml", "RosePine.toml"];

fn read_config() -> Result<Config> {
    let cwd = std::env::current_dir().expect("valid working directory");
    for path in CONFIG_PATHS {
        let path = cwd.join(path);
        if path.exists() {
            let data = fs::read_to_string(path).expect("to read config");
            let parsed: Config =
                ron::from_str(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            return Ok(parsed);
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "Unable to read config file from default paths",
    ))
}
