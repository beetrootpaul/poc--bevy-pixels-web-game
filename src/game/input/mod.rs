pub use crate::game::input::keyboard_controls::KeyboardControlsSystems;
pub use crate::game::input::touch_controls::TouchControlsSystems;
use bevy::prelude::Resource;

mod keyboard_controls;
mod touch_controls;

#[derive(Resource)]
pub struct InputConfig {
    pub is_touch_available: bool,
}
