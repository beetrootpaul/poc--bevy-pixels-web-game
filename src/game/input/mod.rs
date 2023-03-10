use bevy::app::App;
use bevy::prelude::Plugin;

use crate::game::input::keyboard_controls::KeyboardControlsPlugin;

mod keyboard_controls;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KeyboardControlsPlugin);
    }
}
