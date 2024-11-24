use clap::Parser;
use rosepine::{
    config::{Args, Config},
    generate,
    palette::Variant,
};
use std::fs;

fn main() {
    let args = Args::parse();
    let config = Config::from(&args);

    let content = fs::read_to_string(&args.template_file).unwrap();

    let filetype = args
        .template_file
        .extension()
        .map(|t| format!(".{}", t.to_string_lossy()))
        .unwrap_or_default();
    let filename = |v: Variant| {
        args.out
            .join(format!("{}{filetype}", v.to_string().to_lowercase()))
    };

    _ = fs::create_dir_all(&args.out);

    // TODO: refactor this mess
    let is_tera = filetype == ".tera";

    if let Some(variant) = args.variant {
        let result = if is_tera {
            generate::render_template(&variant, &content).unwrap()
        } else {
            generate::generate_variant(&variant, &config, &content)
        };

        fs::write(filename(variant), result).expect("to write");
    } else {
        let variants = if is_tera {
            generate::render_templates(&content).unwrap()
        } else {
            generate::generate_variants(&config, &content)
        };

        for (variant, content) in variants {
            fs::write(filename(variant), content).expect("to write");
        }
    }
}
