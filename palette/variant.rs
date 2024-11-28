use crate::{Color, Hsl, Metadata, Rgb, Role};
use heck::ToSnakeCase;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Variant {
    Main,
    Moon,
    Dawn,
}

#[derive(Debug, Copy, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum VariantKind {
    Light,
    Dark,
}

type Colors = (Rgb, Hsl);

pub struct Palette {
    base: Colors,
    surface: Colors,
    overlay: Colors,
    muted: Colors,
    subtle: Colors,
    text: Colors,
    love: Colors,
    gold: Colors,
    rose: Colors,
    pine: Colors,
    foam: Colors,
    iris: Colors,
    highlight_low: Colors,
    highlight_med: Colors,
    highlight_high: Colors,
}

impl Palette {
    pub const MAIN: Palette = Palette {
        base: (rgb(25, 23, 36), hsl(249, 22, 12)),
        surface: (rgb(31, 29, 46), hsl(247, 23, 15)),
        overlay: (rgb(38, 35, 58), hsl(248, 25, 18)),
        muted: (rgb(110, 106, 134), hsl(249, 12, 47)),
        subtle: (rgb(144, 140, 170), hsl(248, 15, 61)),
        text: (rgb(224, 222, 244), hsl(245, 50, 91)),
        love: (rgb(235, 111, 146), hsl(343, 76, 68)),
        gold: (rgb(246, 193, 119), hsl(35, 88, 72)),
        rose: (rgb(235, 188, 186), hsl(2, 55, 83)),
        pine: (rgb(49, 116, 143), hsl(197, 49, 38)),
        foam: (rgb(156, 207, 216), hsl(189, 43, 73)),
        iris: (rgb(196, 167, 231), hsl(267, 57, 78)),
        highlight_low: (rgb(33, 32, 46), hsl(244, 18, 15)),
        highlight_med: (rgb(64, 61, 82), hsl(249, 15, 28)),
        highlight_high: (rgb(82, 79, 103), hsl(248, 13, 36)),
    };
    pub const MOON: Palette = Palette {
        base: (rgb(35, 33, 54), hsl(246, 24, 17)),
        surface: (rgb(42, 39, 63), hsl(248, 24, 20)),
        overlay: (rgb(57, 53, 82), hsl(248, 21, 26)),
        muted: (rgb(110,106,134), hsl(249,12,47)),
        subtle: (rgb(144,140,170), hsl(248,15,61)),
        text: (rgb(224,222,244), hsl(245,50,91)),
        love: (rgb(235,111,146), hsl(343,76,68)),
        gold: (rgb(246,193,119), hsl(35,88,72)),
        rose: (rgb(234,154,151), hsl(2,66,75)),
        pine: (rgb(62,143,176), hsl(197,48,47)),
        foam: (rgb(156,207,216), hsl(189,43,73)),
        iris: (rgb(196,167,231), hsl(267,57,78)),
        highlight_low: (rgb(42, 40, 62), hsl(245, 22, 20)),
        highlight_med: (rgb(68, 65, 90), hsl(247, 16, 30)),
        highlight_high: (rgb(86, 82, 110), hsl(249, 15, 38)),
    };
    pub const DAWN: Palette = Palette {
        base: (rgb(250, 244, 237), hsl(32, 57, 95)),
        surface: (rgb(255, 250, 243), hsl(35, 100, 98)),
        overlay: (rgb(242, 233, 222), hsl(33, 43, 91)),
        muted: (rgb(152, 147, 165), hsl(257, 9, 61)),
        subtle: (rgb(121, 117, 147), hsl(248, 12, 52)),
        text: (rgb(87, 82, 121), hsl(248, 19, 40)),
        love: (rgb(180, 99, 122), hsl(343, 35, 55)),
        gold: (rgb(234, 157, 52), hsl(35, 81, 56)),
        rose: (rgb(215, 130, 126), hsl(3, 53, 67)),
        pine: (rgb(40, 105, 131), hsl(197, 53, 34)),
        foam: (rgb(86, 148, 159), hsl(189, 30, 48)),
        iris: (rgb(144, 122, 169), hsl(268, 21, 57)),
        highlight_low: (rgb(244, 237, 232), hsl(25, 35, 93)),
        highlight_med: (rgb(223, 218, 217), hsl(10, 9, 86)),
        highlight_high: (rgb(206, 202, 205), hsl(315, 4, 80)),
    };

    pub const fn get_role(&self, role: &Role) -> Colors {
        match role {
            Role::Base => self.base,
            Role::Surface => self.surface,
            Role::Overlay => self.overlay,
            Role::Muted => self.muted,
            Role::Subtle => self.subtle,
            Role::Text => self.text,
            Role::Love => self.love,
            Role::Gold => self.gold,
            Role::Rose => self.rose,
            Role::Pine => self.pine,
            Role::Foam => self.foam,
            Role::Iris => self.iris,
            Role::HighlightLow => self.highlight_low,
            Role::HighlightMed => self.highlight_med,
            Role::HighlightHigh => self.highlight_high,
        }
    }

    pub const fn get_rgb(&self, role: &Role) -> Rgb {
        self.get_role(role).0
    }
    pub const fn get_hsl(&self, role: &Role) -> Hsl {
        self.get_role(role).1
    }
}

impl Variant {
    pub const fn get_palette(&self) -> Palette {
        match self {
            Variant::Main => Palette::MAIN,
            Variant::Moon => Palette::MOON,
            Variant::Dawn => Palette::DAWN,
        }
    }

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

    pub fn kind(&self) -> VariantKind {
        match self {
            Self::Dawn => VariantKind::Light,
            _ => VariantKind::Dark,
        }
    }

    pub fn metadata(&self) -> HashMap<String, String> {
        Metadata::iter()
            .map(|r| (r.to_string().to_snake_case(), r.format(self)))
            .collect()
    }

    pub fn colors(&self) -> HashMap<String, Color> {
        Role::iter()
            .map(|r| (r.to_string().to_snake_case(), r.get_color(self)))
            .collect()
    }

    pub const fn get_rgb(&self, role: Role) -> Rgb {
        self.get_palette().get_rgb(&role)
    }

    pub const fn get_hsl(&self, role: Role) -> Hsl {
        self.get_palette().get_hsl(&role)
    }
}

const fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb { r, g, b }
}
const fn hsl(h: u16, s: u8, l: u8) -> Hsl {
    Hsl { h, s, l }
}
