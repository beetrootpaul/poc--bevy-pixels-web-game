use bevy::prelude::Resource;
use pixels::Pixels;

pub use color::Color;
pub use pico8_color::Pico8Color;
pub use plugin::PixelCanvasPlugin;

use crate::game::Xy;
use crate::pixel_canvas::draw::Draw;

mod color;
mod draw;
mod pico8_color;
mod plugin;

#[derive(Resource)]
pub struct PixelCanvas {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl PixelCanvas {
    pub fn clear(&mut self, color: Color) -> &mut Self {
        Draw::clear(self.pixels.get_frame_mut(), color);
        self
    }

    // TODO: it doesn't feel right to pass simple XY as a reference :thinking:
    pub fn set_pixel(&mut self, xy: &Xy, color: Color) -> &mut Self {
        if let Some(pixel_index) = self.pixel_index_at(xy) {
            Draw::set_pixel(self.pixels.get_frame_mut(), pixel_index, color);
        }
        self
    }

    // TODO: should be here or inside Draw?
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
