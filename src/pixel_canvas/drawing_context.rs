use crate::game::Xy;

// each pixel occupies 4 u8 bytes of a frame
pub const PX_LEN: usize = 4;

pub struct DrawingContext<'a> {
    pub frame: &'a mut [u8],
    w: usize, // width
    h: usize, // height
}

impl<'a> DrawingContext<'a> {
    pub fn new(frame: &'a mut [u8], w: usize, h: usize) -> Self {
        Self { frame, w, h }
    }

    // TODO: test it
    pub fn pixel_first_index_for(&self, xy: &Xy) -> Option<usize> {
        // TODO: better way for number type conversion?
        let w = self.w as i32;
        let h = self.h as i32;
        let (x, y) = xy.rounded();
        if x >= 0 && x < w && y >= 0 && y < h {
            // TODO: better way for number type conversion?
            Some(PX_LEN * (y * w + x) as usize)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pixel_canvas::drawing_context::PX_LEN;

    use super::*;

    #[test]
    fn calculate_valid_pixel_index() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let ctx = DrawingContext::new(&mut frame, W, H);

        // TODO: this API with floating points looks suspicious and it seems like something that we should test for fractions as well
        assert_eq!(ctx.pixel_first_index_for(&Xy(0., 0.)), Some(0));
        assert_eq!(ctx.pixel_first_index_for(&Xy(1., 0.)), Some(PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(2., 0.)), Some(2 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(0., 1.)), Some(3 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(1., 1.)), Some(4 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(2., 1.)), Some(5 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(0., 2.)), Some(6 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(1., 2.)), Some(7 * PX_LEN));
        assert_eq!(ctx.pixel_first_index_for(&Xy(2., 2.)), Some(8 * PX_LEN));
    }

    #[test]
    fn calculate_invalid_pixel_index() {
        const W: usize = 3;
        const H: usize = 3;
        let mut frame = [0; PX_LEN * W * H];
        let ctx = DrawingContext::new(&mut frame, W, H);

        // TODO: this API with floating points looks suspicious and it seems like something that we should test for fractions as well
        assert_eq!(ctx.pixel_first_index_for(&Xy(-1., 0.)), None);
        assert_eq!(ctx.pixel_first_index_for(&Xy(0., -1.)), None);
        assert_eq!(ctx.pixel_first_index_for(&Xy(3., 0.)), None);
        assert_eq!(ctx.pixel_first_index_for(&Xy(0., 3.)), None);
    }
}
