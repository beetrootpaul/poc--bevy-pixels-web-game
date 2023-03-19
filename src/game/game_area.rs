use bevy::prelude::{Rect, Resource};

use crate::game::Xy;

const GAME_AREA_WIDTH: u32 = 128;
const GAME_AREA_HEIGHT: u32 = 128;

const LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH: u32 = 64;

const PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT: u32 = 128;

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
    pub fn width(&self) -> u32 {
        GAME_AREA_WIDTH
    }

    pub fn height(&self) -> u32 {
        GAME_AREA_HEIGHT
    }

    pub fn rect(&self) -> Rect {
        match self.variant {
            GameAreaVariant::NoControls => {
                Rect::new(0.0, 0.0, GAME_AREA_WIDTH as f32, GAME_AREA_HEIGHT as f32)
            },
            GameAreaVariant::PortraitControls => {
                Rect::new(0.0, 0.0, GAME_AREA_WIDTH as f32, GAME_AREA_HEIGHT as f32)
            },
            GameAreaVariant::LandscapeControls => Rect::new(
                LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32,
                0.0,
                (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH + GAME_AREA_WIDTH) as f32,
                GAME_AREA_HEIGHT as f32,
            ),
        }
    }

    pub fn outer_width(&self) -> u32 {
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

    pub fn outer_height(&self) -> u32 {
        match self.variant {
            GameAreaVariant::NoControls => GAME_AREA_HEIGHT,
            GameAreaVariant::PortraitControls => {
                GAME_AREA_HEIGHT + PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT
            },
            GameAreaVariant::LandscapeControls => GAME_AREA_HEIGHT,
        }
    }

    pub fn outer_rects(&self) -> Vec<Rect> {
        match self.variant {
            GameAreaVariant::NoControls => vec![],
            GameAreaVariant::PortraitControls => vec![Rect::new(
                0.0,
                GAME_AREA_HEIGHT as f32,
                GAME_AREA_WIDTH as f32,
                (GAME_AREA_HEIGHT + PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT) as f32,
            )],
            GameAreaVariant::LandscapeControls => vec![
                Rect::new(
                    0.0,
                    0.0,
                    LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32,
                    GAME_AREA_HEIGHT as f32,
                ),
                Rect::new(
                    (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH + GAME_AREA_WIDTH) as f32,
                    0.0,
                    (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
                        + GAME_AREA_WIDTH
                        + LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH) as f32,
                    GAME_AREA_HEIGHT as f32,
                ),
            ],
        }
    }

    pub fn game_area_xy_from(&self, xy: Xy) -> Xy {
        match self.variant {
            GameAreaVariant::NoControls => Xy(xy.0, xy.1),
            GameAreaVariant::PortraitControls => Xy(xy.0, xy.1),
            GameAreaVariant::LandscapeControls => Xy(
                (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32) + xy.0,
                xy.1,
            ),
        }
    }
}
