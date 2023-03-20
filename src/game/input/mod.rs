pub use crate::game::input::keyboard_controls::KeyboardControlsSystems;
use bevy::prelude::Resource;

mod keyboard_controls;

#[derive(Resource)]
pub struct InputConfig {
    pub is_touch_available: bool,
}
