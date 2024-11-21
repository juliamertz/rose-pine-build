use clap::Parser;
use rosepine::{
    generate::{Config, Format, Generator},
    palette::Variant,
};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, default_value = "dist")]
    out_dir: PathBuf,

    #[clap(long, short, default_value = "hex")]
    format: Format,

    #[clap(long, short)]
    variant: Option<Variant>,

    #[clap(long, short, default_value = "$")]
    prefix: char,

    template_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.prefix, args.format);

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
