use bevy::math::vec2;
use bevy::prelude::{Rect, Resource};

// TODO: assets loading from https://github.com/NiklasEi/bevy_game_template

#[derive(Resource, Default)]
pub struct SpriteSheet {
    // TODO: consider making it non-option or panicking on runtime if asset is not loaded yet
    pub maybe_rgba_image: Option<image::RgbaImage>,
}

// TODO: ???
impl SpriteSheet {
    const COLUMNS: usize = 16;
    // TODO: implement
    // pub const ROWS: usize = 4;
    const CELL_W: usize = 8;
    const CELL_H: usize = 8;

    // TODO: implement
    // pub const COIN_FIRST: usize = 0;
    // pub const COIN_LAST: usize = 31;
    pub const PLAYER_UP: usize = 34;
    pub const PLAYER_RIGHT: usize = 35;
    pub const PLAYER_DOWN: usize = 36;
    pub const PLAYER_LEFT: usize = 37;
    // TODO: implement
    // pub const TRAIL_PARTICLE_5PX: usize = 39;
    // pub const TRAIL_PARTICLE_3PX: usize = 40;
    // pub const TRAIL_PARTICLE_1PX: usize = 41;

    pub fn source_rect_of_cell(cell_index: usize) -> Rect {
        // TODO: better way for number type conversion?
        let cell_index = cell_index as i32;
        let cols = Self::COLUMNS as i32;
        let cw = Self::CELL_W as i32;
        let ch = Self::CELL_H as i32;

        let col = cell_index % cols;
        let row = (cell_index - col) / cols;
        Rect::from_corners(
            // TODO: better way for number type conversion?
            vec2((col * cw) as f32, (row * ch) as f32),
            vec2(((col + 1) * cw) as f32, ((row + 1) * ch) as f32),
        )
    }
}
