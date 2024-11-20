pub mod generate;
pub mod palette;

mod utils;

use clap::ValueEnum;
use colors_transform::{Color, Rgb};
use std::char;
use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display, Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
// TODO: Better example values
pub enum Format {
    /// Hex value with opacity last: 000000FF
    HexNs,
    /// Hex value with opacity first: FF000000
    AhexNs,
    /// rgb values delimited by spaces: 100 100 100
    RgbNs,
    /// semicolon delimited rgb values: 100;100;100
    RgbAnsi,
    /// Array of rgb values: []
    RgbArray,
    /// CSS rgb function: rgb()
    RgbFunction,
    /// hsl values delimited by spaces: 100 100 100
    HslNs,
    /// Array of hsl values: []
    HslArray,
    /// CSS Hsl function: hsl()
    HslFunction,
    /// Hex string with opacity last: #000000FF
    Hex,
    /// Hex string with opacity first: #FF000000
    Ahex,
    /// Comma seperated rgb values: 100, 100, 100
    Rgb,
    /// Comma seperated hsl values: 100, 100, 100
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
    pub fn format_color(&self, color: Rgb, alpha: Option<f32>) -> String {
        let mut chunks = if self.is_hsl() {
            hsl_values(color)
        } else {
            rgb_values(color)
        };

        if let Some(alpha) = alpha {
            match *self {
                Self::Ahex | Self::AhexNs => chunks.insert(0, alpha * 255.0),
                Self::Hex | Self::HexNs => chunks.push(alpha * 255.0),
                _ => chunks.push(alpha),
            }
        }

        let chunks = self.format_chunks(&chunks);
        match self {
            Self::Hex | Self::Ahex => format!("#{chunks}"),
            Self::Rgb
            | Self::Hsl
            | Self::RgbNs
            | Self::HslNs
            | Self::HexNs
            | Self::AhexNs
            | Self::RgbAnsi => chunks,
            Self::RgbArray | Self::HslArray => format!("[{chunks}]"),
            Self::RgbFunction | Self::HslFunction => {
                let fn_name = match self {
                    Self::RgbFunction => "rgb",
                    Self::HslFunction => "hsl",
                    _ => unreachable!(),
                };
                let fn_name = match alpha.is_some() {
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
            Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs => chunks.join(""),
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
            Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs => {
                format!("{:02X}", chunk.round() as u8)
            }
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
