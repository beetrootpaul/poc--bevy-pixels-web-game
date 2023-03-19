use bevy::prelude::Rect;

use crate::game::Xy;

const GAME_AREA_WIDTH: u32 = 128;
const GAME_AREA_HEIGHT: u32 = 128;

const LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH: u32 = 64;

pub struct GameArea;

// TODO: implement portrait mode and test manually and set dynamically based on viewport size
impl GameArea {
    pub fn width() -> u32 {
        GAME_AREA_WIDTH
    }

    pub fn height() -> u32 {
        GAME_AREA_HEIGHT
    }

    pub fn rect() -> Rect {
        Rect::new(
            LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32,
            0.0,
            (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH + GAME_AREA_WIDTH) as f32,
            GAME_AREA_HEIGHT as f32,
        )
    }

    pub fn outer_width() -> u32 {
        LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
            + GAME_AREA_WIDTH
            + LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH
    }

    pub fn outer_height() -> u32 {
        GAME_AREA_HEIGHT
    }

    pub fn outer_rects() -> Vec<Rect> {
        vec![
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
        ]
    }

    pub fn game_area_xy_from(xy: Xy) -> Xy {
        Xy(
            (LANDSCAPE_TOUCH_CONTROLS_SIDE_AREA_WIDTH as f32) + xy.0,
            xy.1,
        )
    }
}
