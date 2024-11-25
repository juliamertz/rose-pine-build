use crate::{config::Config, format::Format};
use anyhow::Result;
use palette::Variant;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod replace;
pub mod templating;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Options {
    pub format: Format,
    pub strip_spaces: bool,
    pub force_alpha: bool,
}

type Template = Vec<(Variant, String)>;
type Templates = Vec<(PathBuf, Template)>;

pub fn generate_template(path: &Path, config: &Config, use_tera: bool) -> Result<Template> {
    let template = fs::read_to_string(path)?;

    if use_tera {
        templating::generate_variants(template)
    } else {
        Ok(replace::generate_variants(config, &template))
    }
}

pub fn generate_templates(
    paths: Vec<PathBuf>,
    config: &Config,
    use_tera: bool,
) -> Result<Templates> {
    let mut buf = vec![];

    for path in paths {
        let variants = generate_template(&path, config, use_tera)?;
        buf.push((path, variants));
    }

    Ok(buf)
}
