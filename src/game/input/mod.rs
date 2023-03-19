use bevy::prelude::Resource;
pub use crate::game::input::keyboard_controls::KeyboardControlsSystems;

mod keyboard_controls;

#[derive(Resource)]
pub struct InputConfig {
    pub is_touch_available: bool
}