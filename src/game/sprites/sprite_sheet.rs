use bevy::math::ivec2;
use bevy::prelude::{IVec2, Resource};

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub maybe_rgba_image: Option<image::RgbaImage>,
}

pub struct Sprite {
    pub sheet_rect: (IVec2, IVec2),
}

impl SpriteSheet {
    pub const PLAYER_UP: Sprite = Sprite {
        sheet_rect: (ivec2(16, 16), ivec2(23, 23)),
    };
    pub const PLAYER_RIGHT: Sprite = Sprite {
        sheet_rect: (ivec2(24, 16), ivec2(31, 23)),
    };
    pub const PLAYER_DOWN: Sprite = Sprite {
        sheet_rect: (ivec2(32, 16), ivec2(39, 23)),
    };
    pub const PLAYER_LEFT: Sprite = Sprite {
        sheet_rect: (ivec2(40, 16), ivec2(47, 23)),
    };
}
