use crate::{config::Config, format::Format};
use anyhow::Result;
use palette::Variant;
use serde::Serialize;
use std::{fs, path::Path};

pub mod replace;
#[cfg(feature = "templating")]
pub mod templating;

#[derive(Clone, Copy, Debug, Serialize, Default)]
pub struct Options {
    pub format: Format,
    pub strip_spaces: bool,
    pub force_alpha: bool,
}

type Template = Vec<(Variant, String)>;

pub fn generate_template(path: &Path, config: &Config) -> Result<Template> {
    let template = fs::read_to_string(path)?;

    #[cfg(feature = "templating")]
    if config.tera {
        return templating::generate_variants(template);
    }

    Ok(replace::generate_variants(config, &template))
}
