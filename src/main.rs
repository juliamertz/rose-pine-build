use std::path::PathBuf;

use clap::Parser;
use rosepine_build::{generate::Generator, palette::Variant, Config, Format};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short)]
    out_dir: Option<PathBuf>,

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

    let out_dir = args.out_dir.unwrap_or("dist".into());
    _ = std::fs::remove_dir_all(&out_dir);
    _ = std::fs::create_dir_all(&out_dir);

    let content = std::fs::read_to_string(&args.template_file).unwrap();
    let generator = Generator::new(config);

    let filetype = args
        .template_file
        .extension()
        .map_or("".to_string(), |t| format!(".{}", t.to_string_lossy()));
    let filename = |v: Variant| out_dir.join(format!("{}{filetype}", v.to_string().to_lowercase()));

    if let Some(variant) = args.variant {
        std::fs::write(
            filename(variant),
            generator
                .generate_variant(variant, &content)
                .expect("to generate variant"),
        )
        .expect("to write");
    } else {
        for (variant, content) in generator.generate_variants(&content).unwrap() {
            std::fs::write(filename(variant), content).expect("to write");
        }
    }
}
