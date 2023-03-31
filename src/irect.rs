use bevy::math::{ivec2, IVec2};

#[inline(always)]
pub const fn irect(w: i32, h: i32) -> IRect {
    IRect {
        left_top: IVec2::ZERO,
        size: ivec2(w, h),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IRect {
    pub left_top: IVec2,
    pub size: IVec2,
}

impl IRect {
    #[allow(dead_code)]
    pub fn xy0(self) -> IVec2 {
        self.left_top
    }
    #[allow(dead_code)]
    pub fn x0(self) -> i32 {
        self.left_top.x
    }
    #[allow(dead_code)]
    pub fn y0(self) -> i32 {
        self.left_top.y
    }
    #[allow(dead_code)]
    pub fn xy1(self) -> IVec2 {
        self.left_top + self.size
    }
    #[allow(dead_code)]
    pub fn x1(self) -> i32 {
        self.left_top.x + self.size.x
    }
    #[allow(dead_code)]
    pub fn y1(self) -> i32 {
        self.left_top.y + self.size.y
    }
    #[allow(dead_code)]
    pub fn center(self) -> IVec2 {
        self.left_top + self.size / 2
    }
    #[allow(dead_code)]
    pub fn w(self) -> i32 {
        self.size.x
    }
    #[allow(dead_code)]
    pub fn h(self) -> i32 {
        self.size.y
    }
    #[allow(dead_code)]
    pub const fn at(self, left: i32, top: i32) -> IRect {
        IRect {
            left_top: ivec2(left, top),
            size: self.size,
        }
    }
}
