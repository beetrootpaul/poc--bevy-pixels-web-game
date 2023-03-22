use std::ops::{Div, Mul, Sub};

use bevy::math::{dvec2, DVec2, IVec2};
use bevy::prelude::Resource;
use image::RgbaImage;
use pixels::Pixels;

pub use color::Color;
pub use plugin::PixelCanvasPlugin;

use crate::pixel_canvas::draw_on_frame::DrawOnFrame;
use crate::pixel_canvas::drawing_context::DrawingContext;

mod color;
mod draw_on_frame;
mod drawing_context;
mod plugin;

#[derive(Resource)]
pub struct PixelCanvas {
    pixels: Pixels,
    logical_width: usize,
    logical_height: usize,
    real_position_inside_window: IVec2,
    scale_logical_to_real: i32,
    viewport_scale_factor: f64,
}

impl PixelCanvas {
    pub fn real_viewport_xy_to_logical_canvas_xy(&self, viewport_xy: IVec2) -> IVec2 {
        let scaled_viewport_xy: DVec2 = dvec2(
            f64::try_from(viewport_xy.x).unwrap(),
            f64::try_from(viewport_xy.y).unwrap(),
        )
        .mul(self.viewport_scale_factor);
        let real_canvas_xy: DVec2 = scaled_viewport_xy.sub(dvec2(
            f64::try_from(self.real_position_inside_window.x).unwrap(),
            f64::try_from(self.real_position_inside_window.y).unwrap(),
        ));
        let logical_canvas_xy: DVec2 =
            real_canvas_xy.div(f64::try_from(self.scale_logical_to_real).unwrap());
        logical_canvas_xy.as_ivec2()
    }

    fn drawing_context(&mut self) -> DrawingContext {
        DrawingContext::new(
            self.pixels.get_frame_mut(),
            self.logical_width,
            self.logical_height,
        )
    }

    #[allow(dead_code)]
    pub fn clear(&mut self, color: Color) {
        DrawOnFrame::clear(&mut self.drawing_context(), color);
    }

    #[allow(dead_code)]
    pub fn set_pixel(&mut self, xy: IVec2, color: Color) {
        DrawOnFrame::set_pixel(&mut self.drawing_context(), xy, color);
    }

    #[allow(dead_code)]
    pub fn draw_filled_rect(&mut self, rect: (IVec2, IVec2), color: Color) {
        DrawOnFrame::draw_filled_rect(&mut self.drawing_context(), rect, color);
    }

    #[allow(dead_code)]
    pub fn draw_sprite(
        &mut self,
        target_xy: IVec2,
        rgba_image: &RgbaImage,
        source_rect: (IVec2, IVec2),
    ) {
        DrawOnFrame::draw_sprite(
            &mut self.drawing_context(),
            target_xy,
            rgba_image,
            source_rect,
        );
    }
}
