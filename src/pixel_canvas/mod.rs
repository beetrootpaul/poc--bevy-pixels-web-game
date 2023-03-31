use bevy::math::{dvec2, ivec2, DVec2, IVec2};
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use image::RgbaImage;
use pixels::Pixels;

pub use color::Color;
pub use plugin::PixelCanvasPlugin;

use crate::font::{FontGlyph, FontSheet};
use crate::irect::IRect;
use crate::pico8::Pico8Color;
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
        ) * self.viewport_scale_factor;
        let real_canvas_xy: DVec2 = scaled_viewport_xy
            - dvec2(
                f64::try_from(self.real_position_inside_window.x).unwrap(),
                f64::try_from(self.real_position_inside_window.y).unwrap(),
            );
        let logical_canvas_xy: DVec2 =
            real_canvas_xy / f64::try_from(self.scale_logical_to_real).unwrap();
        logical_canvas_xy.as_ivec2()
    }

    fn drawing_context(&mut self) -> DrawingContext {
        DrawingContext::new(
            self.pixels.frame_mut(),
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
    pub fn draw_rect_filled(&mut self, rect: IRect, color: Color) {
        DrawOnFrame::draw_rect_filled(&mut self.drawing_context(), rect, color);
    }

    #[allow(dead_code)]
    pub fn draw_ellipse(&mut self, bounding_rect: IRect, color: Color) {
        DrawOnFrame::draw_ellipse(&mut self.drawing_context(), bounding_rect, color, false);
    }

    #[allow(dead_code)]
    pub fn draw_ellipse_filled(&mut self, bounding_rect: IRect, color: Color) {
        DrawOnFrame::draw_ellipse(&mut self.drawing_context(), bounding_rect, color, true);
    }

    #[allow(dead_code)]
    pub fn draw_sprite(&mut self, target_xy: IVec2, rgba_image: &RgbaImage, source_rect: IRect) {
        DrawOnFrame::draw_sprite(
            &mut self.drawing_context(),
            target_xy,
            rgba_image,
            source_rect,
            HashMap::new(),
        );
    }

    #[allow(dead_code)]
    pub fn draw_text<FS: FontSheet>(&mut self, target_xy: IVec2, font_sheet: &FS, text: &str) {
        let font_sheet_image = font_sheet.rgba_image();
        let mut xy = target_xy;
        for char in text.chars() {
            let glyph_rect = font_sheet.rect_of(FontGlyph::of(char));
            DrawOnFrame::draw_sprite(
                &mut self.drawing_context(),
                xy,
                font_sheet_image,
                glyph_rect,
                HashMap::from([
                    (font_sheet.font_color(), Pico8Color::White.into()),
                    (font_sheet.transparent_color(), Color::Transparent),
                ]),
            );
            xy += ivec2(glyph_rect.w() + 1, 0);
        }
    }
}
