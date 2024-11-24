use crate::{Color, Hsl, Metadata, Rgb, Role};
use heck::ToSnekCase;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

#[derive(
    Debug, Clone, Copy, Display, PartialEq, Eq, EnumIter, VariantNames, EnumString, Hash, Serialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum Variant {
    Main,
    Moon,
    Dawn,
}

impl Variant {
    pub fn id(&self) -> String {
        match self {
            Self::Main => "rose-pine",
            Self::Moon => "rose-pine-moon",
            Self::Dawn => "rose-pine-dawn",
        }
        .into()
    }

    pub fn key(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn name(&self) -> String {
        match self {
            Self::Main => "Rosé Pine",
            Self::Moon => "Rosé Pine Moon",
            Self::Dawn => "Rosé Pine Dawn",
        }
        .into()
    }

    pub fn is_light(&self) -> bool {
        self.eq(&Variant::Dawn)
    }

    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    pub fn colors(&self) -> HashMap<String, Color> {
        Role::iter()
            .map(|r| (r.to_string().to_snek_case(), r.get_color(self)))
            .collect()
    }

    pub fn metadata(&self) -> Metadata {
        self.into()
    }

    pub fn get_rgb(&self, role: Role) -> Rgb {
        match self {
            Variant::Main => match role {
                Role::Base => rgb(25, 23, 36),
                Role::Surface => rgb(31, 29, 46),
                Role::Overlay => rgb(38, 35, 58),
                Role::Muted => rgb(110, 106, 134),
                Role::Subtle => rgb(144, 140, 170),
                Role::Text => rgb(224, 222, 244),
                Role::Love => rgb(235, 111, 146),
                Role::Gold => rgb(246, 193, 119),
                Role::Rose => rgb(235, 188, 186),
                Role::Pine => rgb(49, 116, 143),
                Role::Foam => rgb(156, 207, 216),
                Role::Iris => rgb(196, 167, 231),
                Role::HighlightLow => rgb(33, 32, 46),
                Role::HighlightMed => rgb(64, 61, 82),
                Role::HighlightHigh => rgb(82, 79, 103),
            },
            Variant::Moon => match role {
                Role::Base => rgb(35, 33, 54),
                Role::Surface => rgb(42, 39, 63),
                Role::Overlay => rgb(57, 53, 82),
                Role::Muted => rgb(110, 106, 134),
                Role::Subtle => rgb(144, 140, 170),
                Role::Text => rgb(224, 222, 244),
                Role::Love => rgb(235, 111, 146),
                Role::Gold => rgb(246, 193, 119),
                Role::Rose => rgb(234, 154, 151),
                Role::Pine => rgb(62, 143, 176),
                Role::Foam => rgb(156, 207, 216),
                Role::Iris => rgb(196, 167, 231),
                Role::HighlightLow => rgb(42, 40, 62),
                Role::HighlightMed => rgb(68, 65, 90),
                Role::HighlightHigh => rgb(86, 82, 110),
            },
            Variant::Dawn => match role {
                Role::Base => rgb(250, 244, 237),
                Role::Surface => rgb(255, 250, 243),
                Role::Overlay => rgb(242, 233, 222),
                Role::Muted => rgb(152, 147, 165),
                Role::Subtle => rgb(121, 117, 147),
                Role::Text => rgb(87, 82, 121),
                Role::Love => rgb(180, 99, 122),
                Role::Gold => rgb(234, 157, 52),
                Role::Rose => rgb(215, 130, 126),
                Role::Pine => rgb(40, 105, 131),
                Role::Foam => rgb(86, 148, 159),
                Role::Iris => rgb(144, 122, 169),
                Role::HighlightLow => rgb(244, 237, 232),
                Role::HighlightMed => rgb(223, 218, 217),
                Role::HighlightHigh => rgb(206, 202, 205),
            },
        }
    }

    pub fn get_hsl(&self, role: Role) -> Hsl {
        use colors_transform::{Color, Rgb}; // TODO: Drop this lib?

        let (r, g, b) = self.get_rgb(role).into();
        let hsl = Rgb::from(r as f32, g as f32, b as f32).to_hsl();
        Hsl::new(
            hsl.get_hue().round() as u16,
            hsl.get_saturation().round() as u8,
            hsl.get_lightness().round() as u8,
        )
    }
}

fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::new(r, g, b)
}
