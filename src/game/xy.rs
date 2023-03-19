use bevy::prelude::Component;

#[derive(Component)]
pub struct Xy(pub f32, pub f32);

impl Xy {
    #[inline(always)]
    pub fn rounded(&self) -> (i32, i32) {
        (self.0.round() as i32, self.1.round() as i32)
    }
}
