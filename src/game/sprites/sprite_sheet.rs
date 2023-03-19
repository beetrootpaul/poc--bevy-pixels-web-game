use bevy::math::vec2;
use bevy::prelude::{Rect, Resource};

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub maybe_rgba_image: Option<image::RgbaImage>,
}

impl SpriteSheet {
    const COLUMNS: usize = 16;
    const CELL_W: usize = 8;
    const CELL_H: usize = 8;

    pub const PLAYER_UP: usize = 34;
    pub const PLAYER_RIGHT: usize = 35;
    pub const PLAYER_DOWN: usize = 36;
    pub const PLAYER_LEFT: usize = 37;

    pub fn source_rect_of_cell(cell_index: usize) -> Rect {
        let cell_index = cell_index as i32;
        let cols = Self::COLUMNS as i32;
        let cw = Self::CELL_W as i32;
        let ch = Self::CELL_H as i32;

        let col = cell_index % cols;
        let row = (cell_index - col) / cols;
        Rect::from_corners(
            vec2((col * cw) as f32, (row * ch) as f32),
            vec2(((col + 1) * cw) as f32, ((row + 1) * ch) as f32),
        )
    }
}
