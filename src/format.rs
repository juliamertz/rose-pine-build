use crate::palette::transform::Rgb;
use palette::{transform::Color, ColorValues};
use strum_macros::{Display, EnumIter, EnumString, VariantNames};
use clap::ValueEnum;

#[derive(
    EnumString, EnumIter, VariantNames, Display, Debug, ValueEnum, Clone, Copy, PartialEq, Eq,
)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    /// #ebbcba | #ebbcbaff
    Hex,
    /// ebbcbaff
    HexNs,
    /// #ebbcba | #ffebbcba
    Ahex,
    /// ffebbcba
    AhexNs,
    /// 235, 188, 186
    Rgb,
    /// 235 188 186
    RgbNs,
    /// rgb(235, 188, 186)
    RgbFunction,
    /// 235;188;186
    RgbArray,
    /// 2, 55%, 83%
    RgbAnsi,
    /// [235, 188, 186]
    Hsl,
    /// 2 55% 83%
    HslNs,
    /// hsl(2, 55%, 83%)
    HslFunction,
    /// [2, 55%, 83%]
    HslArray,
}

impl Format {
    pub fn is_hsl(&self) -> bool {
        matches!(
            self,
            Self::Hsl | Self::HslNs | Self::HslArray | Self::HslFunction
        )
    }

    pub fn is_rgb(&self) -> bool {
        matches!(
            self,
            Self::Rgb | Self::RgbNs | Self::RgbArray | Self::RgbFunction | Self::RgbAnsi
        )
    }

    pub fn is_hex(&self) -> bool {
        matches!(self, Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs)
    }

    pub fn format_color(&self, color: Rgb, alpha: Option<impl Into<f32> + Copy>) -> String {
        let mut chunks = match self.is_hsl() {
            true => color.to_hsl().color_values(),
            false => color.color_values(),
        };

        if let Some(alpha) = alpha {
            match *self {
                Self::Ahex | Self::AhexNs => chunks.insert(0, (alpha.into() / 100.0) * 255.0),
                Self::Hex | Self::HexNs => chunks.push((alpha.into() / 100.0) * 255.0),
                _ => chunks.push(alpha.into() / 100.0),
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
            Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs => chunks.join("").to_lowercase(),
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
}
