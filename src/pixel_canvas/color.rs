#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Solid {
        // each color as a value between 0 and 255
        r: u8,
        g: u8,
        b: u8,
    },
    Transparent,
}

#[cfg(test)]
pub const fn color_solid(r: u8, g: u8, b: u8) -> Color {
    Color::Solid { r, g, b }
}
