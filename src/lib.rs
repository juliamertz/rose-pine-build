pub mod colors;

use std::char;

use palette::{
    rgb::Rgb, Alpha, FromColor, GetHue, Hsl, Hsla, IntoColor, Lch, LinSrgba, Srgb, Srgba,
};
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(EnumString, AsRefStr, Display, Debug)]
#[strum(serialize_all = "kebab-case")]
pub enum Format {
    Hex,
    HexNs,
    Rgb,
    RgbNs,
    RgbAnsi,
    RgbArray,
    RgbFunction,
    Hsl,
    HslNs,
    HslArray,
    HslFunction,
}

pub struct Config {
    pub prefix: char,
}

// FIX: HSL values are wrong
// rgb(200, 10, 50)
// hsl(347.3684, 0%, 1%)

// fn hsl_values(color: Srgba) -> (f32, f32, f32) {
//     let h: Hsla = color.into_color();
//     (h.hue.into_positive_degrees(), h.saturation, h.lightness)
// }

impl Format {
    pub fn to_string(&self, color: Srgb<u8>) -> String {
        let (red, green, blue) = (color.red, color.green, color.blue);
        // let (hue, saturation, lightness) = hsl_values(color);

        match self {
            Self::Hex => format!("#{:02X}{:02X}{:02X}", red, green, blue),
            Self::HexNs => format!("{:02X}{:02X}{:02X}", red, green, blue),
            Self::Rgb => format!("{}, {}, {}", red, green, blue),
            Self::RgbNs => format!("{} {} {}", red, green, blue),
            Self::RgbAnsi => format!("{};{};{}", red, green, blue),
            Self::RgbArray => format!("[{}, {}, {}]", red, green, blue),
            Self::RgbFunction => format!("rgb({}, {}, {})", red, green, blue),
            // Self::Hsl => format!("{}, {}%, {}%", hue, saturation, lightness),
            // Self::HslNs => format!("{} {}% {}%", hue, saturation, lightness),
            // Self::HslArray => format!("[{}, {}%, {}%]", hue, saturation, lightness),
            // Self::HslFunction => format!("hsl({}, {}%, {}%)", hue, saturation, lightness),
            _ => unimplemented!(),
        }
    }

    pub fn to_alpha_string(&self, color: Alpha<Rgb<Srgb,u8>, f32>) -> String {
        let (red, green, blue) = (color.red, color.green, color.blue);
        let alpha: u8 = (color.alpha * 255.0).round() as u8;
        // let (hue, saturation, lightness) = hsl_values(color);

        match self {
            Self::Hex => format!("#{:02X}{:02X}{:02X}{:02X}", red, green, blue, alpha),
            Self::HexNs => format!("{:02X}{:02X}{:02X}{:02X}", red, green, blue, alpha),
            Self::Rgb => format!("{}, {}, {}, {}", red, green, blue, alpha),
            Self::RgbNs => format!("{} {} {} {}", red, green, blue, alpha),
            Self::RgbAnsi => format!("{};{};{};{}", red, green, blue, alpha),
            Self::RgbArray => format!("[{}, {}, {}, {}]", red, green, blue, alpha),
            Self::RgbFunction => format!("rgb({}, {}, {}, {})", red, green, blue, alpha),
            // Self::Hsl => format!("{}, {}%, {}%", hue, saturation, lightness),
            // Self::HslNs => format!("{} {}% {}%", hue, saturation, lightness),
            // Self::HslArray => format!("[{}, {}%, {}%]", hue, saturation, lightness),
            // Self::HslFunction => format!("hsl({}, {}%, {}%)", hue, saturation, lightness),
            _ => unimplemented!(),
        }
    }
}
