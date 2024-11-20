use std::{io::Write, path::PathBuf};

use clap::Parser;
use rosepine_build::{generate, palette::Variant, Config, Format};
use strum::IntoEnumIterator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short)]
    write: bool,

    #[clap(long, short)]
    out_dir: Option<PathBuf>,

    #[clap(long, short, default_value = "hex")]
    format: Format,

    #[clap(long, short, default_value = "$")]
    prefix: char,

    template_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.prefix, args.format);

    let out_dir = args.out_dir.unwrap_or("dist".into());
    _ = std::fs::create_dir_all(&out_dir);

    let content = std::fs::read_to_string(&args.template_file).unwrap();

    for variant in Variant::iter() {
        let result = generate::replace_templates(&content, variant, &config);
        if args.write {
            let filetype = args
                .template_file
                .extension()
                .map_or("".to_string(), |t| format!(".{}", t.to_string_lossy()));
            let filename = out_dir.join(format!("{variant}{filetype}"));

            std::fs::write(filename, result).unwrap();
        } else {
            std::io::stdout().write_all(result.as_bytes()).unwrap();
        }
    }
}
