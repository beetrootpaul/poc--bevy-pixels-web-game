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
        // TODO: consider grouping audio related stuff into a SystemParam
        audio_files: Res<AudioFiles>,
        asset_server: Res<AssetServer>,
    ) {
        match *current_state {
            // TODO: add some nice "loading" screen during Loading state
            State(GameState::Loading) => {
                // TODO: make it debug and visible as web console log?
                info!("Loading…");
                // TODO: encapsulate closer to AudioFiles maybe?
                let audio_files_load_state: LoadState = asset_server
                    .get_group_load_state(audio_files.all_handles().iter().map(|h| h.id()));
                if audio_files_load_state == LoadState::Loaded {
                    // TODO: make it debug and visible as web console log?
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
