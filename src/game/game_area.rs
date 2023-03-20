use std::ops::Add;

use bevy::math::ivec2;
use bevy::prelude::{IVec2, Resource};

const GAME_AREA_WIDTH: i32 = 128;
const GAME_AREA_HEIGHT: i32 = 128;

const LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH: i32 = 64;

const PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT: i32 = 128;

#[derive(Resource)]
pub struct GameArea {
    pub variant: GameAreaVariant,
}

#[allow(clippy::enum_variant_names)]
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

    pub fn rect(&self) -> (IVec2, IVec2) {
        match self.variant {
            GameAreaVariant::NoControls => (ivec2(0, 0), ivec2(GAME_AREA_WIDTH, GAME_AREA_HEIGHT)),
            GameAreaVariant::PortraitControls => {
                (ivec2(0, 0), ivec2(GAME_AREA_WIDTH, GAME_AREA_HEIGHT))
            },
            GameAreaVariant::LandscapeControls => (
                ivec2(LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH, 0),
                ivec2(
                    LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH + GAME_AREA_WIDTH,
                    GAME_AREA_HEIGHT,
                ),
            ),
        }
    }

    pub fn outer_width(&self) -> i32 {
        match self.variant {
            GameAreaVariant::NoControls => GAME_AREA_WIDTH,
            GameAreaVariant::PortraitControls => GAME_AREA_WIDTH,
            GameAreaVariant::LandscapeControls => {
                LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
                    + GAME_AREA_WIDTH
                    + LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
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

    pub fn outer_rects(&self) -> Vec<(IVec2, IVec2)> {
        match self.variant {
            GameAreaVariant::NoControls => vec![],
            GameAreaVariant::PortraitControls => vec![(
                ivec2(0, GAME_AREA_HEIGHT),
                ivec2(
                    GAME_AREA_WIDTH,
                    GAME_AREA_HEIGHT + PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT,
                ),
            )],
            GameAreaVariant::LandscapeControls => vec![
                (
                    ivec2(0, 0),
                    ivec2(LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH, GAME_AREA_HEIGHT),
                ),
                (
                    ivec2(
                        LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH + GAME_AREA_WIDTH,
                        0,
                    ),
                    ivec2(
                        LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
                            + GAME_AREA_WIDTH
                            + LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH,
                        GAME_AREA_HEIGHT,
                    ),
                ),
            ],
        }
    }

    pub fn game_area_xy_from(&self, xy: IVec2) -> IVec2 {
        match self.variant {
            GameAreaVariant::NoControls => xy,
            GameAreaVariant::PortraitControls => xy,
            GameAreaVariant::LandscapeControls => {
                xy.add(ivec2(LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH, 0))
            },
        }
    }
}
