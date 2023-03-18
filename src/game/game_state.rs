use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Hash, Clone, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
    DebugPause,
    DebugResumeFor1Frame,
}

impl GameState {
    pub fn is_game_loaded(current_state: Res<State<GameState>>) -> bool {
        !matches!(*current_state, State(GameState::Loading))
    }
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
        match *current_state {
            // TODO: add some nice "loading" screen during Loading state
            State(GameState::Loading) => {
                // TODO: make it debug and visible as web console log?
                info!("Loading…");
                // TODO: make it debug and visible as web console log?
                info!("All audio files are loaded :-)");
                next_state.set(GameState::InGame);
            },
            State(GameState::DebugResumeFor1Frame) => {
                next_state.set(GameState::DebugPause);
            },
            _ => {},
        };
    }
}
