use colors_transform::{Color, Hsl, Rgb};

pub(crate) trait Reversed {
    fn reversed(self) -> Self;
}
impl<T> Reversed for Vec<T> {
    fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
}

pub(crate) trait Substitutable {
    fn substitute(&mut self, replacement: Self, start: usize, end: usize);
}

impl Substitutable for String {
    fn substitute(&mut self, replacement: Self, start: usize, end: usize) {
        let (before, after) = self.split_at(start);
        let (_, after_replace) = after.split_at(end - start);
        *self = format!("{}{}{}", before, replacement, after_replace)
    }
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
