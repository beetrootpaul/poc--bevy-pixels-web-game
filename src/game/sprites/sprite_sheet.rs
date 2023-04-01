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
        sheet_rect: irect(7, 7).at(16, 16),
    };
    pub const PLAYER_RIGHT: Sprite = Sprite {
        sheet_rect: irect(7, 7).at(24, 16),
    };
    pub const PLAYER_DOWN: Sprite = Sprite {
        sheet_rect: irect(7, 7).at(32, 16),
    };
    pub const PLAYER_LEFT: Sprite = Sprite {
        sheet_rect: irect(7, 7).at(40, 16),
    };

    pub const COIN_FRAME_1: Sprite = Sprite {
        sheet_rect: irect(6, 6).at(1, 1),
    };

    pub const TOUCH_BUTTON_UP: Sprite = Sprite {
        sheet_rect: irect(30, 26).at(0, 0),
    };
    pub const TOUCH_BUTTON_RIGHT: Sprite = Sprite {
        sheet_rect: irect(26, 30).at(0, 26),
    };
    pub const TOUCH_BUTTON_DOWN: Sprite = Sprite {
        sheet_rect: irect(30, 26).at(26, 30),
    };
    pub const TOUCH_BUTTON_LEFT: Sprite = Sprite {
        sheet_rect: irect(26, 30).at(30, 0),
    };
    pub const TOUCH_BUTTON_UP_PRESSED: Sprite = Sprite {
        sheet_rect: irect(30, 26).at(0, 56),
    };
    pub const TOUCH_BUTTON_RIGHT_PRESSED: Sprite = Sprite {
        sheet_rect: irect(26, 30).at(0, 82),
    };
    pub const TOUCH_BUTTON_DOWN_PRESSED: Sprite = Sprite {
        sheet_rect: irect(30, 26).at(26, 86),
    };
    pub const TOUCH_BUTTON_LEFT_PRESSED: Sprite = Sprite {
        sheet_rect: irect(26, 30).at(30, 56),
    };
}
