#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Color {
    Solid {
        // each color as a value between 0 and 255
        r: u8,
        g: u8,
        b: u8,
    },
    Transparent,
}
