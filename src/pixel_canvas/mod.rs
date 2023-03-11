use bevy::prelude::Resource;
use pixels::Pixels;

pub use color::Color;
pub use pico8_color::Pico8Color;
pub use plugin::PixelCanvasPlugin;

use crate::game::Xy;

mod color;
mod pico8_color;
mod plugin;

#[derive(Resource)]
pub struct PixelCanvas {
    pixels: Pixels,
    width: u32,
    height: u32,
}

// each pixel occupies 4 u8 bytes of a frame
const PX_LEN: usize = 4;

impl PixelCanvas {
    pub fn clear(&mut self, color: Color) {
        if let Color::Solid { r, g, b } = color {
            let frame = self.pixels.get_frame_mut();
            frame.copy_from_slice(&[r, g, b, 0xff].repeat(frame.len() / PX_LEN));
        }
    }

    // TODO: it doesn't feel right to pass simple XY as a reference :thinking:
    pub fn set_pixel(&mut self, xy: &Xy, color: Color) {
        if let Color::Solid { r, g, b } = color {
            if let Some(pixel_index) = self.pixel_index_at(xy) {
                let frame = self.pixels.get_frame_mut();
                frame[(PX_LEN * pixel_index)..(PX_LEN * pixel_index + PX_LEN)]
                    .copy_from_slice(&[r, g, b, 0xff]);
            }
        }
    }

    fn pixel_index_at(&self, xy: &Xy) -> Option<usize> {
        // TODO: better way for number type conversion?
        let w = self.width as i32;
        let h = self.height as i32;
        let (x, y) = xy.rounded();
        if x >= 0 && x < w && y >= 0 && y < h {
            // TODO: better way for number type conversion?
            Some((y * w + x) as usize)
        } else {
            None
        }
    }
}
