use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct SpriteSheet {
    // TODO: consider making it non-option or panicking on runtime if asset is not loaded yet
    pub maybe_rgba_image: Option<image::RgbaImage>,
}

// TODO: ???
impl SpriteSheet {
    // TODO: implement
    // pub const COLUMNS: usize = 16;
    // pub const ROWS: usize = 4;
    // pub const DEFAULT_SPRITE_W: f32 = 8.;
    // pub const DEFAULT_SPRITE_H: f32 = 8.;

    // TODO: implement
    // pub const COIN_FIRST: usize = 0;
    // pub const COIN_LAST: usize = 31;
    // pub const PLAYER_UP: usize = 34;
    // pub const PLAYER_RIGHT: usize = 35;
    // pub const PLAYER_DOWN: usize = 36;
    // pub const PLAYER_LEFT: usize = 37;
    // pub const TRAIL_PARTICLE_5PX: usize = 39;
    // pub const TRAIL_PARTICLE_3PX: usize = 40;
    // pub const TRAIL_PARTICLE_1PX: usize = 41;
}
