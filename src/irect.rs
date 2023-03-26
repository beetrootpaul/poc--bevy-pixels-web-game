use bevy::math::{ivec2, IVec2};

#[inline(always)]
pub const fn irect(top_left_x: i32, top_left_y: i32, size_w: i32, size_h: i32) -> IRect {
    IRect {
        top_left: ivec2(top_left_x, top_left_y),
        size: ivec2(size_w, size_h),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IRect {
    pub top_left: IVec2,
    pub size: IVec2,
}

impl IRect {
    pub fn x(self) -> i32 {
        self.top_left.x
    }
    pub fn y(self) -> i32 {
        self.top_left.y
    }
    pub fn w(self) -> i32 {
        self.size.x
    }
    pub fn h(self) -> i32 {
        self.size.y
    }
}
