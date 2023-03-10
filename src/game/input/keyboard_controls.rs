use bevy::prelude::*;

use crate::game::player::PlayerMovement;

pub struct KeyboardControlsSystems;

impl KeyboardControlsSystems {
    // TODO: handle a case of multiple arrows pressed at once
    pub fn handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_movement_query: Query<&mut PlayerMovement>,
        // TODO: ???
        // mut commands: Commands,
        // current_state: Res<CurrentState<GameState>>,
        // sprites_boundaries_config: Option<ResMut<SpritesBoundariesConfig>>,
        // hit_circles_visualization_config: Option<ResMut<HitCirclesVisualizationConfig>>,
    ) {
        // println!("handle_keyboard_input");
        if keyboard_input.just_pressed(KeyCode::Left) {
            for mut player_movement in player_movement_query.iter_mut() {
                *player_movement = PlayerMovement::Left;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            for mut player_movement in player_movement_query.iter_mut() {
                *player_movement = PlayerMovement::Right;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            for mut player_movement in player_movement_query.iter_mut() {
                *player_movement = PlayerMovement::Up;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            for mut player_movement in player_movement_query.iter_mut() {
                *player_movement = PlayerMovement::Down;
            }
        }

        // TODO: ???
        // d = enter [d]ebug pause
        // #[cfg(debug_assertions)]
        // if keyboard_input.just_pressed(KeyCode::D) {
        // match *current_state {
        //     CurrentState(GameState::InGame) => {
        //         commands.insert_resource(NextState(GameState::DebugPause));
        //     }
        //     CurrentState(GameState::DebugPause) => {
        //         commands.insert_resource(NextState(GameState::InGame));
        //     }
        //     _ => {}
        // };
        // }
        // TODO: ???
        // #[cfg(debug_assertions)]
        // if let CurrentState(GameState::DebugPause) = *current_state {
        //     if keyboard_input.just_pressed(KeyCode::Period) {
        //         commands.insert_resource(NextState(GameState::DebugResumeFor1Frame));
        //     }
        // }
        // TODO: ???
        // s = draw [s]prite boundaries
        // #[cfg(debug_assertions)]
        // if keyboard_input.just_pressed(KeyCode::S) {
        //     if let Some(mut config) = sprites_boundaries_config {
        //         config.is_plugin_enabled = !config.is_plugin_enabled;
        //     }
        // }
        // TODO: ???
        // c = draw hit [c]ircle visualization
        // #[cfg(debug_assertions)]
        // if keyboard_input.just_pressed(KeyCode::C) {
        // if let Some(mut config) = hit_circles_visualization_config {
        //     config.is_plugin_enabled = !config.is_plugin_enabled;
        // }
        // }
    }
}
