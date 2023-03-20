use bevy::math::ivec2;
use bevy::prelude::{IVec2, Resource};

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub maybe_rgba_image: Option<image::RgbaImage>,
}

impl SpriteSheet {
    const COLUMNS: i32 = 16;
    const CELL_W: i32 = 8;
    const CELL_H: i32 = 8;

    pub const PLAYER_UP: i32 = 34;
    pub const PLAYER_RIGHT: i32 = 35;
    pub const PLAYER_DOWN: i32 = 36;
    pub const PLAYER_LEFT: i32 = 37;

    pub fn source_rect_of_cell(cell_index: i32) -> (IVec2, IVec2) {
        let col = cell_index % Self::COLUMNS;
        let row = (cell_index - col) / Self::COLUMNS;
        (
            ivec2(col * Self::CELL_W, row * Self::CELL_H),
            ivec2((col + 1) * Self::CELL_W, (row + 1) * Self::CELL_H),
        )
    }
}
