use colors_transform::Color;
use rosepine_build::colors::{Role, Variant};

fn main() {
    let c = Role::Love.get_color(Variant::Moon);
    let r = c.get_red();
    let g = c.get_green();
    let b = c.get_blue();
    // rgb(235, 111, 146)
    println!("rgb({r}, {g}, {b})");
    // let content = String::from(
    //     r#"
    //     $love
    //     #ff0000
    //     $love:rgb_function
    //     $love:hsl_function
    //     rgb(100, 200, 150)
    // "#,
    // );
    //
    // let replaced = replace_templates(&content, Variant::Moon, &Config::default());
    //
    // dbg!(&content);
    // dbg!(&replaced);
}
