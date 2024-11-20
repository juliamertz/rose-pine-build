pub mod generate;
pub mod palette;

mod utils;

use colors_transform::{AlphaColor, Color, Rgb};
use std::char;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(EnumString, EnumIter, AsRefStr, Display, Debug, clap::ValueEnum, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    HexNs,
    RgbNs,
    RgbAnsi,
    RgbArray,
    RgbFunction,
    HslNs,
    HslArray,
    HslFunction,
    Hex,
    Rgb,
    Hsl,
}

pub struct Config {
    pub prefix: char,
    pub format: Format,
}

impl Config {
    pub fn new(prefix: char, format: Format) -> Self {
        Self { prefix, format }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: '$',
            format: Format::Hex,
        }
    }
}

fn rgb_values(color: Rgb) -> Vec<f32> {
    vec![color.get_red(), color.get_green(), color.get_blue()]
}

fn hsl_values(color: Rgb) -> Vec<f32> {
    let color = color.to_hsl();
    vec![
        color.get_hue().round(),
        color.get_saturation().round(),
        color.get_lightness().round(),
    ]
}

impl Format {
    pub fn to_color_string(&self, color: Rgb, alpha: bool) -> String {
        let mut chunks = if self.is_hsl() {
            hsl_values(color)
        } else {
            rgb_values(color)
        };
        if alpha {
            chunks.push(color.get_alpha());
        }

        let chunks = self.format_chunks(&chunks);
        match self {
            Self::Hex => format!("#{chunks}"),
            Self::Rgb | Self::Hsl | Self::RgbNs | Self::HslNs | Self::HexNs | Self::RgbAnsi => {
                chunks
            }
            Self::RgbArray | Self::HslArray => format!("[{chunks}]"),
            Self::RgbFunction | Self::HslFunction => {
                let fn_name = match self {
                    Self::RgbFunction => "rgb",
                    Self::HslFunction => "hsl",
                    _ => unreachable!(),
                };
                let fn_name = match alpha {
                    true => &format!("{fn_name}a"),
                    false => fn_name,
                };
                format!("{fn_name}({chunks})")
            }
        }
    }

    fn format_chunks(&self, chunks: &[f32]) -> String {
        let chunks = chunks
            .iter()
            .enumerate()
            .map(|(i, x)| self.format_chunk(*x, i))
            .collect::<Vec<_>>();
        match self {
            Self::Hex | Self::HexNs => chunks.join(""),
            Self::Rgb
            | Self::RgbArray
            | Self::RgbFunction
            | Self::Hsl
            | Self::HslArray
            | Self::HslFunction => chunks.join(", "),
            Self::RgbNs | Self::HslNs => chunks.join(" "),
            Self::RgbAnsi => chunks.join(";"),
        }
    }

    fn format_chunk(&self, chunk: f32, i: usize) -> String {
        if self.is_hsl() && (i > 0) {
            return format!("{chunk}%");
        }

        match self {
            Self::Hex | Self::HexNs => format!("{:02X}", chunk.round() as u8),
            _ => chunk.to_string(),
        }
    }

    fn is_hsl(&self) -> bool {
        matches!(
            self,
            Self::Hsl | Self::HslNs | Self::HslArray | Self::HslFunction
        )
    }
}
