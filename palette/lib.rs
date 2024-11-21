mod variant;

pub use colors_transform as transform;
pub use variant::*;

use colors_transform::Rgb;
use std::collections::HashMap;
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

pub struct Metadata {
    pub variant: Variant,
    pub id: String,
    pub key: String,
    pub name: String,
    pub colors: HashMap<Role, Rgb>,
}

impl From<&Variant> for Metadata {
    fn from(value: &Variant) -> Self {
        Metadata {
            id: value.id(),
            key: value.key(),
            name: value.name(),
            colors: value.colors(),
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
