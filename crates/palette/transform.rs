use crate::{Color, Hsl};

const MAX_LIGHTNESS: u8 = 100;
const MAX_STEPS: u8 = 10;
const ABS_STEP: u8 = 10;

pub fn lighten(color: Color, steps: u8, step_size: u8) -> Color {
    Color::from(Hsl {
        l: color
            .hsl
            .l
            .saturating_add(steps * step_size)
            .min(MAX_LIGHTNESS),
        ..color.hsl
    })
}

pub fn absolute_lighten(color: Color, steps: u8) -> Color {
    lighten(color, steps, ABS_STEP)
}

pub fn relative_lighten(color: Color, steps: u8) -> Color {
    let step_size = (MAX_LIGHTNESS - color.hsl.l) / MAX_STEPS;
    lighten(color, steps, step_size)
}

pub fn darken(color: Color, steps: u8, step_size: u8) -> Color {
    Color::from(Hsl {
        l: color.hsl.l.saturating_sub(steps * step_size),
        ..color.hsl
    })
}

pub fn absolute_darken(color: Color, steps: u8) -> Color {
    darken(color, steps, ABS_STEP)
}

pub fn relative_darken(color: Color, steps: u8) -> Color {
    let step_size = color.hsl.l / MAX_STEPS;
    darken(color, steps, step_size)
}

