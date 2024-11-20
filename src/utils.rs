pub(crate) fn replace_substring(text: &str, start: usize, end: usize, replacement: &str) -> String {
    let (before, after) = text.split_at(start);
    let (_, after_replace) = after.split_at(end - start);

    format!("{}{}{}", before, replacement, after_replace)
}

use colors_transform::Rgb;
pub(crate) fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::from(r, g, b)
}
