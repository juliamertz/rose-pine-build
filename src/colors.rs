use colors_transform::Rgb;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display, EnumIter)]
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
    pub fn get_color(&self, variant: Variant) -> Rgb {
        variant.get_color(*self)
    }
}

#[derive(Debug, Clone, Copy, EnumString, Display, EnumIter)]
pub enum Variant {
    Main,
    Moon,
    Dawn,
}

impl Variant {
    pub fn get_color(&self, role: Role) -> Rgb {
        match self {
            Variant::Main => match role {
                Role::Base => rgb(25.0, 23.0, 36.0),
                Role::Surface => rgb(31.0, 29.0, 46.0),
                Role::Overlay => rgb(38.0, 35.0, 58.0),
                Role::Muted => rgb(110.0, 106.0, 134.0),
                Role::Subtle => rgb(144.0, 140.0, 170.0),
                Role::Text => rgb(224.0, 222.0, 244.0),
                Role::Love => rgb(235.0, 111.0, 146.0),
                Role::Gold => rgb(246.0, 193.0, 119.0),
                Role::Rose => rgb(235.0, 188.0, 186.0),
                Role::Pine => rgb(49.0, 116.0, 143.0),
                Role::Foam => rgb(156.0, 207.0, 216.0),
                Role::Iris => rgb(196.0, 167.0, 231.0),
                Role::HighlightLow => rgb(33.0, 32.0, 46.0),
                Role::HighlightMed => rgb(64.0, 61.0, 82.0),
                Role::HighlightHigh => rgb(82.0, 79.0, 103.0),
            },
            Variant::Moon => match role {
                Role::Base => rgb(35.0, 33.0, 54.0),
                Role::Surface => rgb(42.0, 39.0, 63.0),
                Role::Overlay => rgb(57.0, 53.0, 82.0),
                Role::Muted => rgb(110.0, 106.0, 134.0),
                Role::Subtle => rgb(144.0, 140.0, 170.0),
                Role::Text => rgb(224.0, 222.0, 244.0),
                Role::Love => rgb(235.0, 111.0, 146.0),
                Role::Gold => rgb(246.0, 193.0, 119.0),
                Role::Rose => rgb(234.0, 154.0, 151.0),
                Role::Pine => rgb(62.0, 143.0, 176.0),
                Role::Foam => rgb(156.0, 207.0, 216.0),
                Role::Iris => rgb(196.0, 167.0, 231.0),
                Role::HighlightLow => rgb(42.0, 40.0, 62.0),
                Role::HighlightMed => rgb(68.0, 65.0, 90.0),
                Role::HighlightHigh => rgb(86.0, 82.0, 110.0),
            },
            Variant::Dawn => match role {
                Role::Base => rgb(250.0, 244.0, 237.0),
                Role::Surface => rgb(255.0, 250.0, 243.0),
                Role::Overlay => rgb(242.0, 233.0, 222.0),
                Role::Muted => rgb(152.0, 147.0, 165.0),
                Role::Subtle => rgb(121.0, 117.0, 147.0),
                Role::Text => rgb(87.0, 82.0, 121.0),
                Role::Love => rgb(180.0, 99.0, 122.0),
                Role::Gold => rgb(234.0, 157.0, 52.0),
                Role::Rose => rgb(215.0, 130.0, 126.0),
                Role::Pine => rgb(40.0, 105.0, 131.0),
                Role::Foam => rgb(86.0, 148.0, 159.0),
                Role::Iris => rgb(144.0, 122.0, 169.0),
                Role::HighlightLow => rgb(244.0, 237.0, 232.0),
                Role::HighlightMed => rgb(223.0, 218.0, 217.0),
                Role::HighlightHigh => rgb(206.0, 202.0, 205.0),
            },
        }
    }
}

fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::from(r, g, b)
}
