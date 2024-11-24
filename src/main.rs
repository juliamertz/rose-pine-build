use clap::{
    builder::styling::{AnsiColor, Styles},
    Parser,
};
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

    if let Some(variant) = args.variant {
        fs::write(
            filename(variant),
            generate::generate_variant(&variant, &config, &content),
        )
        .expect("to write");
    } else {
        for (variant, content) in generate::generate_variants(&config, &content) {
            fs::write(filename(variant), content).expect("to write");
        }
    }
}
