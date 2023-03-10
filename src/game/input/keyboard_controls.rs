use bevy::prelude::*;

#[cfg(debug_assertions)]
use crate::game::game_state::GameState;
use crate::game::player::PlayerMovement;

pub struct KeyboardControlsSystems;

impl KeyboardControlsSystems {
    // TODO: handle a case of multiple arrows pressed at once
    pub fn handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_movement_query: Query<&mut PlayerMovement>,
        #[cfg(debug_assertions)] current_state: Res<State<GameState>>,
        #[cfg(debug_assertions)] mut next_state: ResMut<NextState<GameState>>,
        // TODO: ???
        // sprites_boundaries_config: Option<ResMut<SpritesBoundariesConfig>>,
        // hit_circles_visualization_config: Option<ResMut<HitCirclesVisualizationConfig>>,
    ) {
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

        // d = enter [d]ebug pause
        #[cfg(debug_assertions)]
        if keyboard_input.just_pressed(KeyCode::D) {
            match *current_state {
                State(GameState::InGame) => {
                    next_state.set(GameState::DebugPause);
                }
                State(GameState::DebugPause) => {
                    next_state.set(GameState::InGame);
                }
                _ => {}
            };
        }
        #[cfg(debug_assertions)]
        if let State(GameState::DebugPause) = *current_state {
            if keyboard_input.just_pressed(KeyCode::Period) {
                next_state.set(GameState::DebugResumeFor1Frame);
            }
        }
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
