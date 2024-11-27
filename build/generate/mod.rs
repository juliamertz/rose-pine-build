use crate::{config::Config, format::Format};
use anyhow::Result;
use palette::Variant;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

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
type Templates = Vec<(PathBuf, Template)>;

pub fn generate_template(path: &Path, config: &Config) -> Result<Template> {
    let template = fs::read_to_string(path)?;

    #[cfg(feature = "templating")]
    if config.tera {
        return templating::generate_variants(template);
    }

    Ok(replace::generate_variants(config, &template))
}

pub fn generate_templates(paths: Vec<PathBuf>, config: &Config) -> Result<Templates> {
    let mut buf = vec![];

    for path in paths {
        let variants = generate_template(&path, config)?;
        buf.push((path, variants));
    }

    Ok(buf)
}
