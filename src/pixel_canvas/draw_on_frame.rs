use bevy::math::ivec2;
use bevy::prelude::IVec2;
use image::{EncodableLayout, RgbaImage};

use crate::irect::IRect;
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
    pub fn draw_horizontal_line(ctx: &mut DrawingContext, xy: IVec2, w: i32, color: Color) {
        assert!(w >= 0);
        if let Color::Solid { r, g, b } = color {
            let ctx_w = i32::try_from(ctx.w).unwrap();
            let ctx_h = i32::try_from(ctx.h).unwrap();
            if xy.x >= ctx_w || xy.y < 0 || xy.y >= ctx_h {
                return;
            }
            let visible_x = xy.x.max(0);
            let visible_w = (xy.x + w).min(ctx_w) - visible_x;

            if let Some(pixel_index) = ctx.pixel_first_index_for(ivec2(visible_x, xy.y)) {
                let visible_w = usize::try_from(visible_w).unwrap();
                ctx.frame[pixel_index..(pixel_index + visible_w * PX_LEN)]
                    .copy_from_slice(&[r, g, b, 0xff].repeat(visible_w));
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_rect_filled(ctx: &mut DrawingContext, rect: IRect, color: Color) {
        for y in rect.y0()..rect.y1() {
            Self::draw_horizontal_line(ctx, ivec2(rect.x0(), y), rect.w(), color);
        }
    }

    // Based on https://github.com/aseprite/aseprite/blob/25fbe786f8353a2ddb57de3bcc5db00066cc9ca6/src/doc/algo.cpp#L216-L315
    #[allow(dead_code)]
    pub fn draw_ellipse(ctx: &mut DrawingContext, bounding_rect: IRect, color: Color, fill: bool) {
        let mut x0: i32 = bounding_rect.x0();
        let mut x1: i32 = bounding_rect.x1() - 1;
        let mut y0: i32 = bounding_rect.y0();
        let mut y1: i32 = bounding_rect.y1() - 1;
        let h = y1 - y0 + 1;

        // diameter
        let mut a: i32 = (x1 - x0).abs();
        let b: i32 = (y1 - y0).abs();

        let mut b1: i32 = b & 1;

        let mut dx: f32 = 4.0 * (1.0 - (a as f32)) * (b as f32) * (b as f32); // error increment
        let mut dy: f32 = 4.0 * ((b1 as f32) + 1.0) * (a as f32) * (a as f32); // error increment

        let mut err: f32 = dx + dy + (b1 * a * a) as f32; // error of 1.step
        let mut e2: f32;

        y0 += (b + 1) / 2;
        y1 = y0 - b1; // starting pixel
        a = 8 * a * a;
        b1 = 8 * b * b;

        loop {
            if fill {
                Self::draw_horizontal_line(ctx, ivec2(x0, y0), x1 - x0 + 1, color); //  I. & II. Quadrant
                Self::draw_horizontal_line(ctx, ivec2(x0, y1), x1 - x0 + 1, color);
            //  III. & IV. Quadrant
            } else {
                Self::set_pixel(ctx, ivec2(x1, y0), color); //   I. Quadrant
                Self::set_pixel(ctx, ivec2(x0, y0), color); //  II. Quadrant
                Self::set_pixel(ctx, ivec2(x0, y1), color); // III. Quadrant
                Self::set_pixel(ctx, ivec2(x1, y1), color); //  IV. Quadrant
            }

            e2 = 2.0 * err;
            if e2 <= dy {
                y0 += 1;
                y1 -= 1;
                dy += a as f32;
                err += dy;
            } // y step
            if e2 >= dx || 2.0 * err > dy {
                x0 += 1;
                x1 -= 1;
                dx += b1 as f32;
                err += dx;
            } // x step

            if x0 > x1 {
                break;
            }
        }

        while y0 - y1 < h {
            // too early stop of flat ellipses a=1
            Self::set_pixel(ctx, ivec2(x0 - 1, y0), color); // -> finish tip of ellipse
            Self::set_pixel(ctx, ivec2(x1 + 1, y0), color);
            y0 += 1;
            Self::set_pixel(ctx, ivec2(x0 - 1, y1), color);
            Self::set_pixel(ctx, ivec2(x1 + 1, y1), color);
            y1 -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn draw_sprite(
        ctx: &mut DrawingContext,
        target_xy: IVec2,
        rgba_image: &RgbaImage,
        source_rect: IRect,
    ) {
        if let Some(pixel_index) = ctx.pixel_first_index_for(target_xy) {
            let sprite_w = usize::try_from(source_rect.w()).unwrap();
            let sprite_h = usize::try_from(source_rect.h()).unwrap();
            let sprite_bytes: &[u8] = rgba_image.as_bytes();
            for sprite_row in 0..sprite_h {
                for sprite_column in 0..sprite_w {
                    let target_i_r = pixel_index + (sprite_row * ctx.w + sprite_column) * PX_LEN;
                    let target_i_g = target_i_r + 1;
                    let target_i_b = target_i_g + 1;
                    let target_i_a = target_i_b + 1;
                    let source_i_r = ((usize::try_from(source_rect.y0()).unwrap() + sprite_row)
                        * usize::try_from(rgba_image.width()).unwrap()
                        + (usize::try_from(source_rect.x0()).unwrap() + sprite_column))
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
    use std::collections::HashMap;

    use bevy::math::ivec2;
    use itertools::Itertools;

    use crate::irect::irect;
    use crate::pixel_canvas::drawing_context::DrawingContext;

    use super::*;

    #[test]
    fn test_clear_frame() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);

        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 1, g: 2, b: 3 });

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(1, 2, 3), "#")]),
            "
                ###
                ###
                ###
            ",
        );
    }

    #[test]
    fn test_set_pixel() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::set_pixel(&mut ctx, ivec2(0, 0), Color::Solid { r: 1, g: 2, b: 3 });
        DrawOnFrame::set_pixel(&mut ctx, ivec2(2, 2), Color::Solid { r: 2, g: 3, b: 4 });

        assert_frame_pixels(
            &ctx,
            HashMap::from([
                (rgb(9, 8, 7), "-"),
                (rgb(1, 2, 3), "#"),
                (rgb(2, 3, 4), "@"),
            ]),
            "
                #--
                ---
                --@
            ",
        );
    }

    #[test]
    fn test_draw_rect_filled() {
        const W: usize = 5;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_rect_filled(
            &mut ctx,
            irect(1, 2, 3, 3),
            Color::Solid { r: 1, g: 2, b: 3 },
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                -----
                -----
                -###-
                -###-
                -###-
            ",
        );
    }

    #[test]
    fn test_draw_rects_clipped() {
        const W: usize = 5;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        // clipped from the left
        DrawOnFrame::draw_rect_filled(
            &mut ctx,
            irect(-1, 1, 3, 3),
            Color::Solid { r: 1, g: 1, b: 1 },
        );
        // clipped from the right
        DrawOnFrame::draw_rect_filled(
            &mut ctx,
            irect(3, 1, 3, 3),
            Color::Solid { r: 2, g: 2, b: 2 },
        );
        // clipped from the top
        DrawOnFrame::draw_rect_filled(
            &mut ctx,
            irect(1, -1, 3, 3),
            Color::Solid { r: 3, g: 3, b: 3 },
        );
        // clipped from the bottom
        DrawOnFrame::draw_rect_filled(
            &mut ctx,
            irect(1, 3, 3, 3),
            Color::Solid { r: 4, g: 4, b: 4 },
        );
        // drawn last, but clipped entirely
        DrawOnFrame::draw_rect_filled(
            &mut ctx,
            irect(-3, -1, 3, 3),
            Color::Solid { r: 5, g: 5, b: 5 },
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([
                (rgb(9, 8, 7), "-"),
                (rgb(1, 1, 1), "#"),
                (rgb(2, 2, 2), "@"),
                (rgb(3, 3, 3), "%"),
                (rgb(4, 4, 4), "*"),
                (rgb(5, 5, 5), "!"),
            ]),
            "
                -%%%-
                #%%%@
                ##-@@
                #***@
                -***-
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_1x1() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(1, 1),
                size: ivec2(1, 1),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            false,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ---
                -#-
                ---
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_2x2() {
        const W: usize = 4;
        const H: usize = 4;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(1, 1),
                size: ivec2(2, 2),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            false,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ----
                -##-
                -##-
                ----
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_4x3() {
        const W: usize = 6;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(1, 1),
                size: ivec2(4, 3),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            false,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ------
                --##--
                -#--#-
                --##--
                ------
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_12x5() {
        const W: usize = 12;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(0, 0),
                size: ivec2(12, 5),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            false,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ---######---
                -##------##-
                #----------#
                -##------##-
                ---######---
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_filled_1x1() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(1, 1),
                size: ivec2(1, 1),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            true,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ---
                -#-
                ---
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_filled_2x2() {
        const W: usize = 4;
        const H: usize = 4;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(1, 1),
                size: ivec2(2, 2),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            true,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ----
                -##-
                -##-
                ----
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_filled_4x3() {
        const W: usize = 6;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        DrawOnFrame::draw_ellipse(
            &mut ctx,
            IRect {
                top_left: ivec2(1, 1),
                size: ivec2(4, 3),
            },
            Color::Solid { r: 1, g: 2, b: 3 },
            true,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([(rgb(9, 8, 7), "-"), (rgb(1, 2, 3), "#")]),
            "
                ------
                --##--
                -####-
                --##--
                ------
            ",
        );
    }

    #[test]
    fn test_draw_ellipses_clipped() {
        const W: usize = 5;
        const H: usize = 5;
        let mut frame = [0; PX_LEN * W * H];
        let mut ctx = DrawingContext::new(&mut frame, W, H);
        DrawOnFrame::clear(&mut ctx, Color::Solid { r: 9, g: 8, b: 7 });

        // clipped from the left
        DrawOnFrame::draw_ellipse(
            &mut ctx,
            irect(-1, 1, 3, 3),
            Color::Solid { r: 1, g: 1, b: 1 },
            true,
        );
        // clipped from the right
        DrawOnFrame::draw_ellipse(
            &mut ctx,
            irect(3, 1, 3, 3),
            Color::Solid { r: 2, g: 2, b: 2 },
            true,
        );
        // clipped from the top
        DrawOnFrame::draw_ellipse(
            &mut ctx,
            irect(1, -1, 3, 3),
            Color::Solid { r: 3, g: 3, b: 3 },
            true,
        );
        // clipped from the bottom
        DrawOnFrame::draw_ellipse(
            &mut ctx,
            irect(1, 3, 3, 3),
            Color::Solid { r: 4, g: 4, b: 4 },
            true,
        );
        // drawn last, but clipped entirely
        DrawOnFrame::draw_ellipse(
            &mut ctx,
            irect(-2, -2, 3, 3),
            Color::Solid { r: 5, g: 5, b: 5 },
            true,
        );

        assert_frame_pixels(
            &ctx,
            HashMap::from([
                (rgb(9, 8, 7), "-"),
                (rgb(1, 1, 1), "#"),
                (rgb(2, 2, 2), "@"),
                (rgb(3, 3, 3), "%"),
                (rgb(4, 4, 4), "*"),
                (rgb(5, 5, 5), "!"),
            ]),
            "
                -%%%-
                #-%-@
                ##-@@
                #-*-@
                -***-
            ",
        );
    }

    fn assert_frame_pixels(
        ctx: &DrawingContext,
        color_symbols: HashMap<[u8; PX_LEN], &str>,
        expected_frame_pixels: &str,
    ) {
        let expected_frame_pixels = expected_frame_pixels
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .join("\n");

        let mut actual_frame_pixels = "".to_string();
        for y in 0..ctx.h {
            actual_frame_pixels += "\n";
            for x in 0..ctx.w {
                let pixel: [u8; PX_LEN] = get_pixel(
                    ctx,
                    ivec2(i32::try_from(x).unwrap(), i32::try_from(y).unwrap()),
                );
                match color_symbols.get(&pixel) {
                    Some(symbol) => actual_frame_pixels += symbol,
                    None => actual_frame_pixels += "?",
                }
            }
        }
        let actual_frame_pixels = actual_frame_pixels.as_str().trim();

        assert_eq!(actual_frame_pixels, expected_frame_pixels);
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
