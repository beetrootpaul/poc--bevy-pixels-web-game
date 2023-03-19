use bevy::math::{vec2, Vec2};
use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug)]
pub struct Xy(pub f32, pub f32);

impl Xy {
    #[inline(always)]
    pub fn rounded(&self) -> (i32, i32) {
        (self.0.round() as i32, self.1.round() as i32)
    }
}

impl From<Xy> for Vec2 {
    fn from(xy: Xy) -> Self {
        vec2(xy.0, xy.1)
    }
}
impl From<Vec2> for Xy {
    fn from(v: Vec2) -> Self {
        Xy(v.x, v.y)
    }
}