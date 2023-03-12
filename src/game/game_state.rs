use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Hash, Clone, Default, Debug)]
pub enum GameState {
    #[default]
    InGame,
    DebugPause,
    DebugResumeFor1Frame,
}

impl GameState {
    pub fn is_game_running(current_state: Res<State<GameState>>) -> bool {
        matches!(
            *current_state,
            State(GameState::InGame) | State(GameState::DebugResumeFor1Frame)
        )
    }

    pub fn update_game_state(
        current_state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        if let State(GameState::DebugResumeFor1Frame) = *current_state {
            next_state.set(GameState::DebugPause);
        }
    }
}
