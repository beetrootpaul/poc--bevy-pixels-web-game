use bevy::prelude::{Rect, Resource};
use image::RgbaImage;
use pixels::Pixels;

pub use color::Color;
pub use plugin::PixelCanvasPlugin;

use crate::game::Xy;
use crate::pixel_canvas::draw_on_frame::DrawOnFrame;
use crate::pixel_canvas::drawing_context::DrawingContext;

mod color;
mod draw_on_frame;
mod drawing_context;
mod plugin;

#[derive(Resource)]
pub struct PixelCanvas {
    pixels: Pixels,
    width: usize,
    height: usize,
}

impl PixelCanvas {
    fn drawing_context(&mut self) -> DrawingContext {
        DrawingContext::new(self.pixels.get_frame_mut(), self.width, self.height)
    }

    #[allow(dead_code)]
    pub fn clear(&mut self, color: Color) {
        DrawOnFrame::clear(&mut self.drawing_context(), color);
    }

    // TODO: it doesn't feel right to pass simple XY as a reference :thinking:
    #[allow(dead_code)]
    pub fn set_pixel(&mut self, xy: &Xy, color: Color) {
        DrawOnFrame::set_pixel(&mut self.drawing_context(), xy, color);
    }

    // TODO: TMP implementation
    #[allow(dead_code)]
    pub fn draw_sprite(&mut self, target_xy: &Xy, rgba_image: &RgbaImage, source_rect: Rect) {
        DrawOnFrame::draw_sprite(&mut self.drawing_context(), target_xy, rgba_image, source_rect);
    }
}
