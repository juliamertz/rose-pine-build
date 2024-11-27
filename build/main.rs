use anyhow::{Context, Result};
use build::{
    config::{Args, Config},
    generate,
};
use clap::Parser;
use std::{fs, path::Path};

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from(&args);

    if !args.template_source.exists() {
        anyhow::bail!(
            "template source can't be found at path {:?}",
            args.template_source
        )
    }

    _ = fs::create_dir_all(&args.out);
    if args.template_source.is_dir() {
        generate_directory(
            &args.template_source,
            &args.template_source,
            &args.out,
            args.recurse,
            &config,
        )?;
    } else {
        let filetype = args.template_source.extension().map_or("".into(), |s| {
            format!(".{}", s.to_str().expect("valid string"))
        });

        let variants = generate::generate_template(&args.template_source, &config)?;
        for (variant, content) in variants {
            let filename = format!("{}{}", variant.key(), filetype);
            let path = args.out.join(filename);
            fs::write(path, content)?;
        }
    }

    Ok(())
}

fn generate_directory(
    base_path: &Path,
    directory_path: &Path,
    out_path: &Path,
    recurse: bool,
    config: &Config,
) -> Result<()> {
    for entry in fs::read_dir(directory_path)? {
        let path = entry?.path();

        if path.is_dir() {
            if !recurse {
                continue;
            }

            generate_directory(
                base_path,
                &directory_path.join(path.file_name().context("expected a dirname")?),
                out_path,
                recurse,
                config,
            )?;
        } else {
            for (variant, content) in generate::generate_template(&path, config)? {
                let path = out_path
                    .join(variant.to_string())
                    .join(path.strip_prefix(base_path)?);

                _ = fs::create_dir_all(
                    path.parent()
                        .context("expected file to have parent directory")?,
                );

                fs::write(path, content)?;
            }
        }
    }

    Ok(())
}
