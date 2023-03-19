use bevy::prelude::*;

#[cfg(debug_assertions)]
use crate::game::game_state::GameState;
use crate::game::player::PlayerMovement;

pub struct KeyboardControlsSystems;

impl KeyboardControlsSystems {
    pub fn handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_movement_query: Query<&mut PlayerMovement>,
        #[cfg(debug_assertions)] current_state: Res<State<GameState>>,
        #[cfg(debug_assertions)] mut next_state: ResMut<NextState<GameState>>,
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
                },
                State(GameState::DebugPause) => {
                    next_state.set(GameState::InGame);
                },
                _ => {},
            };
        }
        #[cfg(debug_assertions)]
        if let State(GameState::DebugPause) = *current_state {
            if keyboard_input.just_pressed(KeyCode::Period) {
                next_state.set(GameState::DebugResumeFor1Frame);
            }
        }
    }
}
