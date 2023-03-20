use bevy::prelude::IVec2;

// each pixel occupies 4 u8 bytes of a frame
pub const PX_LEN: usize = 4;

pub struct DrawingContext<'a> {
    pub frame: &'a mut [u8],
    pub w: usize, // width
    pub h: usize, // height
}

impl<'a> DrawingContext<'a> {
    pub fn new(frame: &'a mut [u8], w: usize, h: usize) -> Self {
        Self { frame, w, h }
    }

    pub fn pixel_first_index_for(&self, xy: IVec2) -> Option<usize> {
        let w = self.w as i32;
        let h = self.h as i32;
        if xy.x >= 0 && xy.x < w && xy.y >= 0 && xy.y < h {
            Some(PX_LEN * (xy.y * w + xy.x) as usize)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::ivec2;

    use crate::pixel_canvas::drawing_context::PX_LEN;

    use super::*;

    #[test]
    fn calculate_valid_pixel_index() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let ctx = DrawingContext::new(&mut frame, W, H);

        assert_eq!(ctx.pixel_first_index_for(ivec2(0, 0)), Some(0));
        assert_eq!(ctx.pixel_first_index_for(ivec2(1, 0)), Some(PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(2, 0)), Some(2 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(0, 1)), Some(3 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(1, 1)), Some(4 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(2, 1)), Some(5 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(0, 2)), Some(6 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(1, 2)), Some(7 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(ivec2(2, 2)), Some(8 * PX_LEN));
    }

    #[test]
    fn calculate_invalid_pixel_index() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let ctx = DrawingContext::new(&mut frame, W, H);

        assert_eq!(ctx.pixel_first_index_for(ivec2(-1, 0)), None);
        assert_eq!(ctx.pixel_first_index_for(ivec2(0, -1)), None);
        assert_eq!(ctx.pixel_first_index_for(ivec2(3, 0)), None);
        assert_eq!(ctx.pixel_first_index_for(ivec2(0, 3)), None);
    }
}
