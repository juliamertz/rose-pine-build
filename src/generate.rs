use clap::ValueEnum;
use colors_transform::{Color, Rgb};
use std::char;
use std::collections::HashMap;
use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

use crate::{
    palette::{Role, Variant},
    parse::{self, ParseError},
    utils::{ColorValues, Reversed, Substitutable},
};

/// HashMap containing output strings for each variant
pub type Outputs = HashMap<Variant, String>;

#[derive(Clone, Debug)]
pub struct Generator {
    config: Config,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub prefix: char,
    pub format: Format,
}

#[derive(
    EnumString, EnumIter, VariantNames, Display, Debug, ValueEnum, Clone, Copy, PartialEq, Eq,
)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    /// #ebbcba | #ebbcbaff
    Hex,
    /// #ebbcba | #ffebbcba
    Ahex,
    /// 235, 188, 186
    Rgb,
    /// 2, 55%, 83%
    Hsl,
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
                Self::Ahex | Self::AhexNs => chunks.insert(0, alpha.into() * 255.0),
                Self::Hex | Self::HexNs => chunks.push(alpha.into() * 255.0),
                _ => chunks.push(alpha.into()),
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

impl Generator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_variants(&self, text: &str) -> Result<Outputs, ParseError> {
        let mut outputs = HashMap::new();
        for v in Variant::iter() {
            outputs.insert(v, self.generate_variant(v, text)?);
        }
        Ok(outputs)
    }

    pub fn generate_variant(&self, variant: Variant, text: &str) -> Result<String, ParseError> {
        let captures = parse::parse_template(text, &self.config);
        // let captures = self.pattern.find_iter(text).collect::<Vec<_>>();
        todo!();
        let mut buffer = text.to_owned();

        // for capture in captures.reversed() {
        //     let template = parse::parse_capture(capture.as_str(), variant, &self.config)?;
        //     buffer.substitute(
        //         template.format_role(variant, &self.config),
        //         capture.start(),
        //         capture.end(),
        //     );
        // }

        Ok(buffer)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::sync::OnceLock;
//
//     use super::*;
//
//     static GENERATOR: OnceLock<Generator> = OnceLock::new();
//     pub fn generate_variant(variant: Variant, text: &str) -> Result<String, ParseError> {
//         GENERATOR
//             .get_or_init(|| Generator::new(Config::default()))
//             .generate_variant(variant, text)
//     }
//
//     #[test]
//     fn format_rgb() -> Result<(), ParseError> {
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:rgb")?,
//             "235, 111, 146"
//         );
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:rgb_function")?,
//             "rgb(235, 111, 146)"
//         );
//         assert_eq!(
//             generate_variant(Variant::Moon, "$pine:rgb_function/80")?,
//             "rgba(62, 143, 176, 0.8)"
//         );
//         Ok(())
//     }
//
//     #[test]
//     fn format_parse_order() -> Result<(), ParseError> {
//         assert_eq!(
//             generate_variant(
//                 Variant::Moon,
//                 "$love:rgb_function,$love:rgb,$love:hex_ns,$love:hex",
//             )?,
//             "rgb(235, 111, 146),235, 111, 146,eb6f92,#eb6f92"
//         );
//         Ok(())
//     }
//
//     #[test]
//     fn format_hsl() -> Result<(), ParseError> {
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:hsl_function")?,
//             "hsl(343, 76%, 68%)"
//         );
//         Ok(())
//     }
//
//     #[test]
//     fn opacity() -> Result<(), ParseError> {
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:rgb_function/50")?,
//             "rgba(235, 111, 146, 0.5)"
//         );
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:hsl_function/50")?,
//             "hsla(343, 76%, 68%, 0.5%)"
//         );
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:hex/100")?,
//             "#eb6f92ff"
//         );
//         assert_eq!(generate_variant(Variant::Moon, "$love:hex/0")?, "#eb6f9200");
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:ahex_ns/50")?,
//             "80eb6f92"
//         );
//         assert_eq!(
//             generate_variant(Variant::Moon, "$love:ahex_ns/100")?,
//             "ffeb6f92"
//         );
//         Ok(())
//     }
//
//     #[test]
//     fn role_variation() -> Result<(), ParseError> {
//         assert_eq!(generate_variant(Variant::Main, "$(pine|foam)")?, "#31748f");
//         assert_eq!(
//             generate_variant(Variant::Main, "$(rose|love):hex")?,
//             "#ebbcba"
//         );
//         assert_eq!(
//             generate_variant(Variant::Dawn, "$(rose|love):hex")?,
//             "#b4637a"
//         );
//
//         Ok(())
//     }
// }
