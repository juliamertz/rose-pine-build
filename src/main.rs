use clap::Parser;
use std::fs;

use rosepine::{
    config::{Args, Config},
    generate::Generator,
    palette::Variant,
};

fn main() {
    let args = Args::parse();
    let config = Config::from(&args);

    let content = fs::read_to_string(&args.template_file).unwrap();
    let generator = Generator::new(config);

    let filetype = args
        .template_file
        .extension()
        .map(|t| format!(".{}", t.to_string_lossy()))
        .unwrap_or_default();
    let filename = |v: Variant| {
        args.out_dir
            .join(format!("{}{filetype}", v.to_string().to_lowercase()))
    };

    _ = fs::create_dir_all(&args.out_dir);

    if let Some(variant) = args.variant {
        fs::write(
            filename(variant),
            generator
                .generate_variant(variant, &content)
                .expect("to generate variant"),
        )
        .expect("to write");
    } else {
        for (variant, content) in generator.generate_variants(&content).unwrap() {
            fs::write(filename(variant), content).expect("to write");
        }
    }
}
