use palette::{Color, Hsl};

const STEP_SIZE: u8 = 10;

pub fn lighten(color: Color, steps: u8) -> Color {
    Color::from(Hsl {
        l: color.hsl.l.saturating_add(steps * STEP_SIZE).min(100),
        ..color.hsl
    })
}

pub fn darken(color: Color, steps: u8) -> Color {
    Color::from(Hsl {
        l: color.hsl.l.saturating_sub(steps * STEP_SIZE),
        ..color.hsl
    })
}
