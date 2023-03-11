#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Color {
    Solid { r: u8, g: u8, b: u8 },
    Transparent,
}
