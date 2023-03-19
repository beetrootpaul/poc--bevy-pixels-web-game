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

pub enum GameAreaVariant {
    Portrait,
    Landscape,
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
            GameAreaVariant::Portrait => {
                Rect::new(0.0, 0.0, GAME_AREA_WIDTH as f32, GAME_AREA_HEIGHT as f32)
            },
            GameAreaVariant::Landscape => Rect::new(
                LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32,
                0.0,
                (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH + GAME_AREA_WIDTH) as f32,
                GAME_AREA_HEIGHT as f32,
            ),
        }
    }

    pub fn outer_width(&self) -> u32 {
        match self.variant {
            GameAreaVariant::Portrait => GAME_AREA_WIDTH,
            GameAreaVariant::Landscape => {
                LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
                    + GAME_AREA_WIDTH
                    + LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
            },
        }
    }

    pub fn outer_height(&self) -> u32 {
        match self.variant {
            GameAreaVariant::Portrait => {
                GAME_AREA_HEIGHT + PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT
            },
            GameAreaVariant::Landscape => GAME_AREA_HEIGHT,
        }
    }

    pub fn outer_rects(&self) -> Vec<Rect> {
        match self.variant {
            GameAreaVariant::Portrait => vec![Rect::new(
                0.0,
                GAME_AREA_HEIGHT as f32,
                GAME_AREA_WIDTH as f32,
                (GAME_AREA_HEIGHT + PORTRAIT_TOUCH_CONTROLS_BOTTOM_AREA_HEIGHT) as f32,
            )],
            GameAreaVariant::Landscape => vec![
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
            GameAreaVariant::Portrait => Xy(xy.0, xy.1),
            GameAreaVariant::Landscape => Xy(
                (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32) + xy.0,
                xy.1,
            ),
        }
    }
}
