use bevy::prelude::Component;

#[inline(always)]
pub const fn xy(x: f32, y: f32) -> Xy {
    Xy(x as f32, y as f32)
}

// TODO: should we use Vec2 instead?
#[derive(Component)]
pub struct Xy(pub f32, pub f32);

impl Xy {
    pub fn rounded(&self) -> (i32, i32) {
        // TODO: better way for number type conversion?
        (self.0.round() as i32, self.1.round() as i32)
    }
}
