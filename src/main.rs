use anyhow::Result;
use clap::Parser;
use rosepine::{
    config::{Args, Config},
    generate,
    palette::Variant,
};
use std::fs;
use strum::IntoEnumIterator;

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
        let out_dir = |v: Variant| args.out.join(v.key());
        for variant in Variant::iter() {
            _ = fs::create_dir_all(out_dir(variant));
        }

        for file in fs::read_dir(&args.template_source)? {
            let file = file?;
            let variants = generate::generate_template(&file.path(), &config)?;
            for (variant, content) in variants {
                let path = out_dir(variant).join(file.file_name());
                fs::write(path, content)?;
            }
        }
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
