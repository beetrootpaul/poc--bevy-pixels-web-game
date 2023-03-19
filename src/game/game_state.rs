use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::game::audio::AudioFiles;

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
        audio_files: Res<AudioFiles>,
        asset_server: Res<AssetServer>,
    ) {
        match *current_state {
            State(GameState::Loading) => {
                info!("Loading…");
                let audio_files_load_state: LoadState = asset_server
                    .get_group_load_state(audio_files.all_handles().iter().map(|h| h.id()));
                if audio_files_load_state == LoadState::Loaded {
                    info!("All audio files are loaded :-)");
                    next_state.set(GameState::InGame);
                }
            },
            State(GameState::DebugResumeFor1Frame) => {
                next_state.set(GameState::DebugPause);
            },
            _ => {},
        };
    }
}
