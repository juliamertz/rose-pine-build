mod generate;

use generate::replace_templates;
use palette::{Alpha, Srgb, Srgba, WithAlpha};

use rosepine_build::{
    colors::{Role, Variant},
    Config, Format,
};
use strum::IntoEnumIterator;

fn main() {
    let my_color: Srgb<u8> = Srgb::new(200, 10, 50);
    let format = Format::RgbFunction;

    dbg!(format.to_string(my_color));

    for role in Role::iter() {
        let color = role.get_color(Variant::Moon);
        let a = color.with_alpha(1.0);
        dbg!(format.to_alpha_string(a));
    }

    let config = &Config { prefix: '$' };
    let _out = replace_templates("".into(), config);
}
