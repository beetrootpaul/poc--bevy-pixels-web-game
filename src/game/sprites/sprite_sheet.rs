use bevy::math::ivec2;
use bevy::prelude::{IVec2, Resource};

pub struct Sprite {
    pub sheet_rect: (IVec2, IVec2),
}

#[derive(Resource)]
pub struct SpriteSheet {
    pub main: image::RgbaImage,
    pub touch_controls: image::RgbaImage,
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

    pub const TOUCH_BUTTON_UP: Sprite = Sprite {
        sheet_rect: (ivec2(0, 0), ivec2(30, 26)),
    };
    pub const TOUCH_BUTTON_RIGHT: Sprite = Sprite {
        sheet_rect: (ivec2(0, 26), ivec2(26, 56)),
    };
    pub const TOUCH_BUTTON_DOWN: Sprite = Sprite {
        sheet_rect: (ivec2(26, 30), ivec2(56, 56)),
    };
    pub const TOUCH_BUTTON_LEFT: Sprite = Sprite {
        sheet_rect: (ivec2(30, 0), ivec2(56, 30)),
    };
    pub const TOUCH_BUTTON_UP_PRESSED: Sprite = Sprite {
        sheet_rect: (ivec2(0, 56), ivec2(30, 82)),
    };
    pub const TOUCH_BUTTON_RIGHT_PRESSED: Sprite = Sprite {
        sheet_rect: (ivec2(0, 82), ivec2(26, 112)),
    };
    pub const TOUCH_BUTTON_DOWN_PRESSED: Sprite = Sprite {
        sheet_rect: (ivec2(26, 86), ivec2(56, 112)),
    };
    pub const TOUCH_BUTTON_LEFT_PRESSED: Sprite = Sprite {
        sheet_rect: (ivec2(30, 56), ivec2(56, 86)),
    };
}
