use crate::pixel_canvas::color::Color;

pub struct Draw;

// each pixel occupies 4 u8 bytes of a frame
const PX_LEN: usize = 4;

impl Draw {
    pub fn clear(frame: &mut [u8], color: Color) {
        if let Color::Solid { r, g, b } = color {
            frame.copy_from_slice(&[r, g, b, 0xff].repeat(frame.len() / PX_LEN));
        }
    }

    pub fn set_pixel(frame: &mut [u8], pixel_index: usize, color: Color) {
        if let Color::Solid { r, g, b } = color {
            frame[(PX_LEN * pixel_index)..(PX_LEN * pixel_index + PX_LEN)]
                .copy_from_slice(&[r, g, b, 0xff]);
        }
    }
}
