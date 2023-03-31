use bevy::math::ivec2;
use bevy::prelude::{IVec2, Resource};

use crate::irect::{irect, IRect};

const GAME_AREA_WIDTH: i32 = 128;
const GAME_AREA_HEIGHT: i32 = 128;

const LANDSCAPE_TOUCH_CONTROLS_LEFT_SIDE_AREA_WIDTH: i32 = 96;
const LANDSCAPE_TOUCH_CONTROLS_RIGHT_AREA_WIDTH: i32 = 32;

const PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT: i32 = 128;

#[derive(Resource)]
pub struct GameArea {
    pub variant: GameAreaVariant,
}

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Debug)]
pub enum GameAreaVariant {
    NoControls,
    PortraitControls,
    LandscapeControls,
}

impl GameArea {
    pub fn width(&self) -> i32 {
        GAME_AREA_WIDTH
    }

    pub fn height(&self) -> i32 {
        GAME_AREA_HEIGHT
    }

    pub fn rect(&self) -> IRect {
        match self.variant {
            GameAreaVariant::NoControls => irect(GAME_AREA_WIDTH, GAME_AREA_HEIGHT),
            GameAreaVariant::PortraitControls => irect(GAME_AREA_WIDTH, GAME_AREA_HEIGHT),
            GameAreaVariant::LandscapeControls => irect(GAME_AREA_WIDTH, GAME_AREA_HEIGHT)
                .at(LANDSCAPE_TOUCH_CONTROLS_LEFT_SIDE_AREA_WIDTH, 0),
        }
    }

    pub fn outer_width(&self) -> i32 {
        match self.variant {
            GameAreaVariant::NoControls => GAME_AREA_WIDTH,
            GameAreaVariant::PortraitControls => GAME_AREA_WIDTH,
            GameAreaVariant::LandscapeControls => {
                LANDSCAPE_TOUCH_CONTROLS_LEFT_SIDE_AREA_WIDTH
                    + GAME_AREA_WIDTH
                    + LANDSCAPE_TOUCH_CONTROLS_RIGHT_AREA_WIDTH
            },
        }
    }

    pub fn outer_height(&self) -> i32 {
        match self.variant {
            GameAreaVariant::NoControls => GAME_AREA_HEIGHT,
            GameAreaVariant::PortraitControls => {
                GAME_AREA_HEIGHT + PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT
            },
            GameAreaVariant::LandscapeControls => GAME_AREA_HEIGHT,
        }
    }

    pub fn outer_rects(&self) -> Vec<IRect> {
        match self.variant {
            GameAreaVariant::NoControls => vec![],
            GameAreaVariant::PortraitControls => {
                vec![
                    irect(GAME_AREA_WIDTH, PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT)
                        .at(0, GAME_AREA_HEIGHT),
                ]
            },
            GameAreaVariant::LandscapeControls => vec![
                irect(
                    LANDSCAPE_TOUCH_CONTROLS_LEFT_SIDE_AREA_WIDTH,
                    GAME_AREA_HEIGHT,
                ),
                irect(LANDSCAPE_TOUCH_CONTROLS_RIGHT_AREA_WIDTH, GAME_AREA_HEIGHT).at(
                    LANDSCAPE_TOUCH_CONTROLS_LEFT_SIDE_AREA_WIDTH + GAME_AREA_WIDTH,
                    0,
                ),
            ],
        }
    }

    pub fn touch_controls_area(&self) -> Option<IRect> {
        self.outer_rects().first().cloned()
    }

    pub fn game_area_xy_from(&self, xy: IVec2) -> IVec2 {
        match self.variant {
            GameAreaVariant::NoControls => xy,
            GameAreaVariant::PortraitControls => xy,
            GameAreaVariant::LandscapeControls => {
                xy + ivec2(LANDSCAPE_TOUCH_CONTROLS_LEFT_SIDE_AREA_WIDTH, 0)
            },
        }
    }

    pub fn game_area_rect_from(&self, rect: IRect) -> IRect {
        IRect {
            left_top: self.game_area_xy_from(rect.left_top),
            size: rect.size,
        }
    }
}
