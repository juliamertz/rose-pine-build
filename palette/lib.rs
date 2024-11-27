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

pub trait ColorValues {
    fn color_values(&self) -> Vec<f32>;
}

// colors_transform library uses f32 idk why
impl ColorValues for Rgb {
    fn color_values(&self) -> Vec<f32> {
        vec![self.r as f32, self.g as f32, self.b as f32]
    }
}
impl ColorValues for Hsl {
    fn color_values(&self) -> Vec<f32> {
        vec![self.h as f32, self.s as f32, self.l as f32]
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
    pub fn get_rgb(&self, variant: &Variant) -> Rgb {
        variant.get_rgb(*self)
    }
    pub fn get_hsl(&self, variant: &Variant) -> Hsl {
        variant.get_hsl(*self)
    }

    pub fn get_color(&self, v: &Variant) -> Color {
        let rgb = self.get_rgb(v);
        Color {
            rgb,
            hsl: self.get_hsl(v),
            hex: format!("{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b),
        }
    }
}
