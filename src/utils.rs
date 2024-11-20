use colors_transform::{Color, Hsl, Rgb};

pub(crate) fn replace_substring(text: &str, start: usize, end: usize, replacement: &str) -> String {
    let (before, after) = text.split_at(start);
    let (_, after_replace) = after.split_at(end - start);

    format!("{}{}{}", before, replacement, after_replace)
}

pub(crate) trait ColorValues {
    fn color_values(&self) -> Vec<f32>;
}

impl ColorValues for Rgb {
    fn color_values(&self) -> Vec<f32> {
        vec![self.get_red(), self.get_green(), self.get_blue()]
    }
}
impl ColorValues for Hsl {
    fn color_values(&self) -> Vec<f32> {
        vec![
            self.get_hue().round(),
            self.get_saturation().round(),
            self.get_lightness().round(),
        ]
    }
}

pub(crate) fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::from(r, g, b)
}
