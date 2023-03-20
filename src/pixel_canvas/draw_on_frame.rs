use bevy::prelude::IVec2;
use image::{EncodableLayout, RgbaImage};

use crate::pixel_canvas::drawing_context::{DrawingContext, PX_LEN};
use crate::pixel_canvas::Color;

pub struct DrawOnFrame;

impl DrawOnFrame {
    #[allow(dead_code)]
    pub fn clear(ctx: &mut DrawingContext, color: Color) {
        if let Color::Solid { r, g, b } = color {
            ctx.frame
                .copy_from_slice(&[r, g, b, 0xff].repeat(ctx.frame.len() / PX_LEN));
        }
    }

    #[allow(dead_code)]
    pub fn set_pixel(ctx: &mut DrawingContext, xy: IVec2, color: Color) {
        if let Color::Solid { r, g, b } = color {
            if let Some(pixel_index) = ctx.pixel_first_index_for(xy) {
                ctx.frame[pixel_index..(pixel_index + PX_LEN)].copy_from_slice(&[r, g, b, 0xff]);
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_filled_rect(ctx: &mut DrawingContext, rect: (IVec2, IVec2), color: Color) {
        if let Color::Solid { r, g, b } = color {
            if let Some(pixel_index) = ctx.pixel_first_index_for(rect.0) {
                let rect_w = usize::try_from(rect.1.x - rect.0.x).unwrap();
                let rect_h = usize::try_from(rect.1.y - rect.0.y).unwrap();
                for rect_row in 0..rect_h {
                    let target_i = pixel_index + (rect_row * ctx.w) * PX_LEN;
                    ctx.frame[target_i..(target_i + rect_w * PX_LEN)]
                        .copy_from_slice(&[r, g, b, 0xff].repeat(rect_w));
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_sprite(
        ctx: &mut DrawingContext,
        target_xy: IVec2,
        rgba_image: &RgbaImage,
        source_rect: (IVec2, IVec2),
    ) {
        if let Some(pixel_index) = ctx.pixel_first_index_for(target_xy) {
            let sprite_w = usize::try_from(source_rect.1.x - source_rect.0.x).unwrap();
            let sprite_h = usize::try_from(source_rect.1.y - source_rect.0.y).unwrap();
            let sprite_bytes: &[u8] = rgba_image.as_bytes();
            for sprite_row in 0..sprite_h {
                for sprite_column in 0..sprite_w {
                    let target_i_r = pixel_index + (sprite_row * ctx.w + sprite_column) * PX_LEN;
                    let target_i_g = target_i_r + 1;
                    let target_i_b = target_i_g + 1;
                    let target_i_a = target_i_b + 1;
                    let source_i_r = ((usize::try_from(source_rect.0.y).unwrap() + sprite_row)
                        * usize::try_from(rgba_image.width()).unwrap()
                        + (usize::try_from(source_rect.0.x).unwrap() + sprite_column))
                        * PX_LEN;
                    let source_i_g = source_i_r + 1;
                    let source_i_b = source_i_g + 1;
                    let source_i_a = source_i_b + 1;
                    if sprite_bytes[source_i_a] > 0x88 {
                        ctx.frame[target_i_r] = sprite_bytes[source_i_r];
                        ctx.frame[target_i_g] = sprite_bytes[source_i_g];
                        ctx.frame[target_i_b] = sprite_bytes[source_i_b];
                        ctx.frame[target_i_a] = sprite_bytes[source_i_a];
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::ivec2;
    use itertools::iproduct;

    use crate::pixel_canvas::drawing_context::DrawingContext;

    use super::*;

    #[test]
    fn clear_frame() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);

        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 1, g: 2, b: 3 });

        for (x, y) in iproduct!(0..W, 0..H) {
            assert_color(&ctx, x, y, rgb(1, 2, 3));
        }
    }

    #[test]
    fn set_pixel_in_frame() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::set_pixel(&mut ctx, ivec2(0, 0), Color::Solid { r: 1, g: 2, b: 3 });
        DrawOnFrame::set_pixel(&mut ctx, ivec2(2, 2), Color::Solid { r: 2, g: 3, b: 4 });

        for (x, y) in iproduct!(0..W, 0..H) {
            match (x, y) {
                (0, 0) => assert_color(&ctx, x, y, rgb(1, 2, 3)),
                (2, 2) => assert_color(&ctx, x, y, rgb(2, 3, 4)),
                _ => assert_color(&ctx, x, y, rgb(9, 8, 7)),
            }
        }
    }

    #[test]
    fn draw_filled_rect_in_frame() {
        const W: usize = 5;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_filled_rect(
            &mut ctx,
            (ivec2(1, 2), ivec2(4, 5)),
            Color::Solid { r: 1, g: 2, b: 3 },
        );

        for (x, y) in iproduct!(0..W, 0..H) {
            match (x, y) {
                (1..=3, 2..=4) => assert_color(&ctx, x, y, rgb(1, 2, 3)),
                _ => assert_color(&ctx, x, y, rgb(9, 8, 7)),
            }
        }
    }

    fn assert_color(ctx: &DrawingContext, x: usize, y: usize, expected_color_slice: [u8; 4]) {
        assert_eq!(
            get_pixel(
                ctx,
                ivec2(i32::try_from(x).unwrap(), i32::try_from(y).unwrap())
            ),
            expected_color_slice,
            "Color mismatch at ({},{})",
            x,
            y
        );
    }

    fn get_pixel(ctx: &DrawingContext, xy: IVec2) -> [u8; PX_LEN] {
        let idx = ctx
            .pixel_first_index_for(xy)
            .expect("should convert XY to pixel index");
        [
            ctx.frame[idx],
            ctx.frame[idx + 1],
            ctx.frame[idx + 2],
            ctx.frame[idx + 3],
        ]
    }

    fn rgb(r: u8, g: u8, b: u8) -> [u8; PX_LEN] {
        [r, g, b, 255]
    }
}
