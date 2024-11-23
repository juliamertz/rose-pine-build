use clap::{
    builder::styling::{AnsiColor, Styles},
    Parser,
};
use rosepine::{config::Config, format::Format, generate, palette::Variant, parse::Delimiter};
use std::{fs, path::PathBuf};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Magenta.on_default())
        .usage(AnsiColor::Blue.on_default())
        .literal(AnsiColor::White.on_default())
        .placeholder(AnsiColor::Yellow.on_default())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, styles=styles())]
pub struct Args {
    #[clap(long, short, default_value = "dist")]
    pub out: PathBuf,

    #[clap(long)]
    pub write_config: bool,

    #[clap(long, short)]
    pub format: Option<Format>,

    #[clap(long, short)]
    pub delimiter: Option<Delimiter>,

    #[clap(long, short)]
    pub seperator: Option<char>,

    #[clap(long, short)]
    pub variant: Option<Variant>,

    #[clap(long, short)]
    pub prefix: Option<char>,

    pub template_file: PathBuf,
}

static CONFIG_PATH: &str = ".rose_pine.ron";

fn main() {
    let args = Args::parse();

    let config = Config::read(CONFIG_PATH).unwrap_or_default();
    if args.write_config {
        config.write(CONFIG_PATH).expect("To write config");
    }

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
