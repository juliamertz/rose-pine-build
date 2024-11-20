use std::{io::Write, path::PathBuf};

use clap::Parser;
use rosepine_build::{colors::Variant, generate, Config};
use strum::IntoEnumIterator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short)]
    write: bool,

    #[clap(long, short)]
    out_dir: Option<PathBuf>,

    template_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let out_dir = args.out_dir.unwrap_or("dist".into());
    _ = std::fs::create_dir_all(&out_dir);

    let content = std::fs::read_to_string(args.template_file).unwrap();

    for variant in Variant::iter() {
        let result = generate::replace_templates(&content, variant, &Config::default());
        if args.write {
            std::fs::write(out_dir.join(format!("{variant}.toml")), result).unwrap();
        } else {
            std::io::stdout().write_all(result.as_bytes()).unwrap();
        }
    }
}
