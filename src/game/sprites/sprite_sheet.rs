use bevy::prelude::Resource;

use crate::irect::{irect, IRect};

pub struct Sprite {
    pub sheet_rect: IRect,
}

#[derive(Resource)]
pub struct SpriteSheet {
    pub main: image::RgbaImage,
    pub touch_controls: image::RgbaImage,
}

impl SpriteSheet {
    pub const PLAYER_UP: Sprite = Sprite {
        sheet_rect: irect(16, 16, 7, 7),
    };
    pub const PLAYER_RIGHT: Sprite = Sprite {
        sheet_rect: irect(24, 16, 7, 7),
    };
    pub const PLAYER_DOWN: Sprite = Sprite {
        sheet_rect: irect(32, 16, 7, 7),
    };
    pub const PLAYER_LEFT: Sprite = Sprite {
        sheet_rect: irect(40, 16, 7, 7),
    };

    pub const TOUCH_BUTTON_UP: Sprite = Sprite {
        sheet_rect: irect(0, 0, 30, 26),
    };
    pub const TOUCH_BUTTON_RIGHT: Sprite = Sprite {
        sheet_rect: irect(0, 26, 26, 30),
    };
    pub const TOUCH_BUTTON_DOWN: Sprite = Sprite {
        sheet_rect: irect(26, 30, 30, 26),
    };
    pub const TOUCH_BUTTON_LEFT: Sprite = Sprite {
        sheet_rect: irect(30, 0, 26, 30),
    };
    pub const TOUCH_BUTTON_UP_PRESSED: Sprite = Sprite {
        sheet_rect: irect(0, 56, 30, 26),
    };
    pub const TOUCH_BUTTON_RIGHT_PRESSED: Sprite = Sprite {
        sheet_rect: irect(0, 82, 26, 30),
    };
    pub const TOUCH_BUTTON_DOWN_PRESSED: Sprite = Sprite {
        sheet_rect: irect(26, 86, 30, 26),
    };
    pub const TOUCH_BUTTON_LEFT_PRESSED: Sprite = Sprite {
        sheet_rect: irect(30, 56, 26, 30),
    };
}
