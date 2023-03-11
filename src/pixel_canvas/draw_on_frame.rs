use crate::game::Xy;
use crate::pixel_canvas::drawing_context::{DrawingContext, PX_LEN};
use crate::pixel_canvas::Color;

pub struct DrawOnFrame;

impl DrawOnFrame {
    pub fn clear(ctx: &mut DrawingContext, color: Color) {
        if let Color::Solid { r, g, b } = color {
            ctx.frame
                .copy_from_slice(&[r, g, b, 0xff].repeat(ctx.frame.len() / PX_LEN));
        }
    }

    // TODO: test it
    // TODO: it doesn't feel right to pass simple XY as a reference :thinking:
    pub fn set_pixel(ctx: &mut DrawingContext, xy: &Xy, color: Color) {
        if let Color::Solid { r, g, b } = color {
            if let Some(pixel_index) = ctx.pixel_first_index_for(xy) {
                ctx.frame[pixel_index..(pixel_index + PX_LEN)].copy_from_slice(&[r, g, b, 0xff]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
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

        // TODO: this API with floating points looks suspicious and it seems like something that we should test for fractions as well
        DrawOnFrame::set_pixel(&mut ctx, &Xy(0., 0.), Color::Solid { r: 1, g: 2, b: 3 });
        DrawOnFrame::set_pixel(&mut ctx, &Xy(2., 2.), Color::Solid { r: 2, g: 3, b: 4 });

        for (x, y) in iproduct!(0..W, 0..H) {
            match (x, y) {
                (0, 0) => assert_color(&ctx, x, y, rgb(1, 2, 3)),
                (2, 2) => assert_color(&ctx, x, y, rgb(2, 3, 4)),
                _ => assert_color(&ctx, x, y, rgb(9, 8, 7)),
            }
        }
    }

    fn assert_color(ctx: &DrawingContext, x: usize, y: usize, expected_color_slice: [u8; 4]) {
        assert_eq!(
            get_pixel(ctx, x, y),
            expected_color_slice,
            "Color mismatch at ({},{})",
            x,
            y
        );
    }

    fn get_pixel(ctx: &DrawingContext, x: usize, y: usize) -> [u8; PX_LEN] {
        // TODO: better way for number type conversion?
        let idx = ctx
            .pixel_first_index_for(&Xy(x as f32, y as f32))
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
