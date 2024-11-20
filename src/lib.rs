pub mod generate;
pub mod palette;

mod utils;

use clap::ValueEnum;
use colors_transform::{Color, Rgb};
use std::char;
use strum_macros::{Display, EnumString, VariantNames};
use utils::ColorValues;

#[derive(EnumString, VariantNames, Display, Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    /// ebbcbaff
    HexNs,
    /// ffebbcba 
    AhexNs,
    /// 235 188 186
    RgbNs,
    /// 235;188;186
    RgbAnsi,
    /// [235, 188, 186]
    RgbArray,
    /// rgb(235, 188, 186)
    RgbFunction,
    /// 2 55% 83%
    HslNs,
    /// [2, 55%, 83%]
    HslArray,
    /// hsl(2, 55%, 83%)
    HslFunction,
    /// #ebbcbaff
    Hex,
    /// #ffebbcba
    Ahex,
    /// 235, 188, 186
    Rgb,
    /// 2, 55%, 83%
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

impl Format {
    pub fn format_color(&self, color: Rgb, alpha: Option<f32>) -> String {
        let mut chunks = match self.is_hsl() {
            true => color.to_hsl().color_values(),
            false => color.color_values(),
        };

        if let Some(alpha) = alpha {
            dbg!(&alpha);
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
