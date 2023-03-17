use std::time::Duration;

#[cfg(debug_assertions)]
use bevy::diagnostic::Diagnostic;
#[cfg(debug_assertions)]
use bevy::diagnostic::{DiagnosticId, Diagnostics};
use bevy::prelude::*;

pub use xy::Xy;
use FixedFpsSystemSet::{FixedFpsLast, FixedFpsSpawning, FixedFpsUpdateAndDraw};
use crate::game::audio::AudioSystems;

use crate::game::game_state::GameState;
use crate::game::input::KeyboardControlsSystems;
use crate::game::player::PlayerSystems;
use crate::game::sprites::SpritesSystems;
use crate::pico8::Pico8Color;
use crate::pixel_canvas::{PixelCanvas, PixelCanvasPlugin};

mod audio;
mod game_state;
mod input;
mod player;
mod sprites;
mod xy;

pub const GAME_TITLE: &str = "Bevy/pixels web game PoC";

pub const GAME_AREA_WIDTH: u32 = 128;
pub const GAME_AREA_HEIGHT: u32 = 128;

const DESIRED_FPS: u64 = 30;

#[allow(clippy::enum_variant_names)]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum FixedFpsSystemSet {
    FixedFpsSpawning,
    FixedFpsUpdateAndDraw,
    FixedFpsLast,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugin(PixelCanvasPlugin {
            // TODO: better way for number type conversion?
            width: GAME_AREA_WIDTH as usize,
            height: GAME_AREA_HEIGHT as usize,
        });

        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_measurements);
        app.add_startup_system(SpritesSystems::load_sprite_sheet);
        app.add_startup_system(AudioSystems::load_music_files);

        app.add_system(AudioSystems::play_music.run_if(GameState::is_game_running));
        app.add_system(
            KeyboardControlsSystems::handle_keyboard_input
                .in_base_set(CoreSet::PreUpdate)
                .run_if(GameState::is_game_loaded),
        );

        app.insert_resource(Self::fixed_time());
        app.edit_schedule(CoreSchedule::FixedUpdate, |schedule| {
            schedule
                .configure_sets((FixedFpsSpawning, FixedFpsUpdateAndDraw, FixedFpsLast).chain());
            schedule.add_system(
                PlayerSystems::spawn_player
                    .run_if(PlayerSystems::there_is_no_player)
                    .in_set(FixedFpsSpawning)
                    .run_if(GameState::is_game_running),
            );
            schedule.add_system(
                apply_system_buffers
                    .after(FixedFpsSpawning)
                    .before(FixedFpsUpdateAndDraw),
            );
            schedule.add_system(
                Self::clear_screen
                    .before(FixedFpsUpdateAndDraw)
                    .run_if(GameState::is_game_loaded),
            );
            schedule.add_systems(
                (
                    PlayerSystems::move_player.run_if(GameState::is_game_running),
                    PlayerSystems::draw_player.run_if(GameState::is_game_loaded),
                )
                    .chain()
                    .in_set(FixedFpsUpdateAndDraw),
            );
            schedule.add_system(GameState::update_game_state.in_set(FixedFpsLast));
            #[cfg(debug_assertions)]
            schedule.add_system(Self::perform_measurements.after(FixedFpsLast));
        });
    }
}

impl GamePlugin {
    fn fixed_time() -> FixedTime {
        let desired_frame_duration = Duration::from_nanos(1_000_000_000 / DESIRED_FPS);
        let mut fixed_time = FixedTime::new(desired_frame_duration);

        // we need to advance timer by a length of a single frame, otherwise we would see an empty canvas during 1st frame
        fixed_time.tick(desired_frame_duration);

        fixed_time
    }

    fn clear_screen(mut pixel_canvas: ResMut<PixelCanvas>) {
        pixel_canvas.clear(Pico8Color::DarkBlue.into());
    }

    // TODO: how to come up with a good ID value?
    #[cfg(debug_assertions)]
    pub const DIAGNOSTIC_FRAME_DURATION: DiagnosticId =
        DiagnosticId::from_u128(1187582084072456577959028643519383692);
    #[cfg(debug_assertions)]
    pub const DIAGNOSTIC_TIME_ACCRUED: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339571239028643519383692);

    #[cfg(debug_assertions)]
    fn setup_measurements(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(
            Diagnostic::new(
                Self::DIAGNOSTIC_FRAME_DURATION,
                "fixed timestamp: frame duration",
                20,
            )
            .with_smoothing_factor(0.)
            .with_suffix("ms"),
        );
        diagnostics.add(
            Diagnostic::new(
                Self::DIAGNOSTIC_TIME_ACCRUED,
                "fixed timestamp: time accrued",
                20,
            )
            .with_smoothing_factor(0.)
            .with_suffix("ms"),
        );
    }

    #[cfg(debug_assertions)]
    fn perform_measurements(
        mut prev_elapsed_seconds: Local<f64>,
        time: Res<Time>,
        fixed_time: Res<FixedTime>,
        mut diagnostics: ResMut<Diagnostics>,
    ) {
        diagnostics.add_measurement(Self::DIAGNOSTIC_FRAME_DURATION, || {
            (time.raw_elapsed_seconds_f64() - *prev_elapsed_seconds) * 1_000.
        });
        diagnostics.add_measurement(Self::DIAGNOSTIC_TIME_ACCRUED, || {
            // TODO: better way for number type conversion?
            fixed_time.accumulated().as_millis() as f64
        });
        *prev_elapsed_seconds = time.raw_elapsed_seconds_f64();
    }
}
