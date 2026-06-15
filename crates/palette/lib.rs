pub mod variant;
pub use variant::*;

use serde::Serialize;
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

#[derive(Debug, Clone, Serialize)]
pub struct Color {
    pub rgb: Rgb,
    pub hsl: Hsl,
    pub hex: String,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Rgb> for (u8, u8, u8) {
    fn from(val: Rgb) -> Self {
        (val.r, val.g, val.b)
    }
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex_string(&self) -> Hex {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Hsl {
    pub h: u16,
    pub s: u8,
    pub l: u8,
}

impl Hsl {
    pub fn new(h: u16, s: u8, l: u8) -> Self {
        Self { h, s, l }
    }
}

pub type Hex = String;

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        let hsl: Hsl = rgb.into();
        let hex = rgb.to_hex_string();
        Self { rgb, hsl, hex }
    }
}
impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Self {
        let rgb: Rgb = hsl.into();
        let hex = rgb.to_hex_string();
        Self { rgb, hsl, hex }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Display, EnumIter)]
pub enum Metadata {
    Id,
    Name,
    Description,
    Key,
    Kind,
}

impl Metadata {
    pub fn format(&self, variant: &Variant) -> String {
        match self {
            Self::Id => variant.id(),
            Self::Name => variant.name(),
            Self::Description => env!("CARGO_PKG_DESCRIPTION").to_string(),
            Self::Key => variant.key(),
            Self::Kind => variant.kind().to_string(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Display, PartialEq, Eq, EnumIter, VariantNames, EnumString, Hash, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    Base,
    Surface,
    Overlay,
    Muted,
    Subtle,
    Text,
    Love,
    Gold,
    Rose,
    Pine,
    Foam,
    Iris,
    HighlightLow,
    HighlightMed,
    HighlightHigh,
}

impl Role {
    pub const fn get_rgb(&self, variant: &Variant) -> Rgb {
        variant.get_rgb(*self)
    }

    pub const fn get_hsl(&self, variant: &Variant) -> Hsl {
        variant.get_hsl(*self)
    }

    pub fn get_hex(&self, variant: &Variant) -> Hex {
        self.get_rgb(variant).to_hex_string()
    }

    pub fn get_color(&self, v: &Variant) -> Color {
        Color {
            rgb: self.get_rgb(v),
            hsl: self.get_hsl(v),
            hex: self.get_hex(v),
        }
    }
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    (h, s * 100.0, l * 100.0) // h: 0–360, s/l: 0–100
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

impl From<Rgb> for Hsl {
    fn from(Rgb { r, g, b }: Rgb) -> Self {
        let (h, s, l) = rgb_to_hsl(r, g, b);
        Self {
            h: h as u16,
            s: s as u8,
            l: l as u8,
        }
    }
}

impl From<Hsl> for Rgb {
    fn from(Hsl { h, s, l }: Hsl) -> Self {
        let (r, g, b) = hsl_to_rgb(h as f64, s as f64, l as f64);
        Self { r, g, b }
    }
}
