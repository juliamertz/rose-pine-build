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
        // TODO:
        for variant in Variant::iter() {
            let out_dir = args.out.join(variant.key());
            _ = fs::create_dir_all(&out_dir);
        }

        for file in fs::read_dir(&args.template_source)? {
            let file = file?;
            let variants = generate::generate_template(&file.path(), &config, args.tera)?;
            for (variant, content) in variants {
                let out_dir = args.out.join(variant.key());
                let path = out_dir.join(file.file_name());
                fs::write(path, content)?;
            }
        }
    } else {
        let filetype = args.template_source.extension().map_or("".into(), |s| {
            format!(".{}", s.to_str().expect("valid string"))
        });

        let variants = generate::generate_template(&args.template_source, &config, args.tera)?;
        for (variant, content) in variants {
            let filename = format!("{}{}", variant.id(), filetype);
            let path = args.out.join(filename);
            fs::write(path, content)?;
        }
    }

    // let variants = generate::generate_templates(vec![args.template_source], &config, args.tera)?;

    // for (file_path, variants) in variants {
    //     // fs::write(filename(variant), content).expect("to write");
    // }

    Ok(())
}
