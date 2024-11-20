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

pub(crate) fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::from(r, g, b)
}

pub(crate) trait Substitutable {
    fn gsub(&mut self, replacement: Self, start: usize, end: usize);
}

impl Substitutable for String {
    fn gsub(&mut self, replacement: Self, start: usize, end: usize) {
        let (before, after) = self.split_at(start);
        let (_, after_replace) = after.split_at(end - start);
        *self = format!("{}{}{}", before, replacement, after_replace)
    }
}

// impl Substitutable for Vec<u8> {
//     fn gsub(&mut self, replacement: Self, start: usize, end: usize) {
//         // let mut result = self.clone();
//         for (i, c) in replacement.iter().enumerate() {
//             if i > end - 1 {
//                 self.insert(start + i, *c);
//             } else {
//                 self[start + i] = *c;
//             }
//         }
//     }
// }

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
