pub mod variant;
pub use colors_transform as transform;
pub use variant::*;

use colors_transform::{Color, Rgb};
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

pub trait ColorValues {
    fn color_values(&self) -> Vec<f32>;
}

impl ColorValues for transform::Rgb {
    fn color_values(&self) -> Vec<f32> {
        vec![self.get_red(), self.get_green(), self.get_blue()]
    }
}

impl ColorValues for transform::Hsl {
    fn color_values(&self) -> Vec<f32> {
        vec![
            self.get_hue().round(),
            self.get_saturation().round(),
            self.get_lightness().round(),
        ]
    }
}

pub struct Metadata {
    pub variant: Variant,
    pub id: String,
    pub key: String,
    pub name: String,
}

impl From<&Variant> for Metadata {
    fn from(value: &Variant) -> Self {
        Metadata {
            id: value.id(),
            key: value.key(),
            name: value.name(),
            variant: *value,
        }
    }
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, EnumIter, VariantNames, EnumString, Hash)]
#[strum(serialize_all = "camelCase")]
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
    pub fn get_color(&self, variant: &Variant) -> Rgb {
        variant.get_color(*self)
    }
}
