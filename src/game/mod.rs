use std::time::Duration;

#[cfg(debug_assertions)]
use bevy::diagnostic::Diagnostic;
#[cfg(debug_assertions)]
use bevy::diagnostic::{DiagnosticId, Diagnostics};
use bevy::prelude::*;

pub use xy::Xy;

use crate::game::game_state::GameState;
use crate::game::input::KeyboardControlsSystems;
use crate::game::player::PlayerSystems;
use crate::pico8::Pico8Color;
use crate::pixel_canvas::{PixelCanvas, PixelCanvasPlugin};

mod game_state;
mod input;
mod player;
mod xy;

pub const GAME_TITLE: &str = "Bevy/pixels web game PoC";

pub const GAME_AREA_WIDTH: u32 = 128;
pub const GAME_AREA_HEIGHT: u32 = 128;

const DESIRED_FPS: u64 = 30;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugin(PixelCanvasPlugin {
            width: GAME_AREA_WIDTH,
            height: GAME_AREA_HEIGHT,
        });

        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_measurements);

        app.add_system(
            KeyboardControlsSystems::handle_keyboard_input.in_base_set(CoreSet::PreUpdate),
        );

        app.insert_resource(Self::fixed_time());
        app.add_systems(
            // TODO: how to make parts of these systems run in parallel instead of all of them being sequential?
            (
                PlayerSystems::spawn_player
                    // TODO: how two run_if s work together
                    .run_if(PlayerSystems::there_is_no_player)
                    .run_if(GameState::should_game_update),
                //
                // flush commands in order to have access to spawned player in the very same frame
                apply_system_buffers,
                //
                PlayerSystems::move_player.run_if(GameState::should_game_update),
                //
                Self::clear_screen,
                PlayerSystems::draw_player,
                //
                GameState::update_game_state,
                //
                #[cfg(debug_assertions)]
                Self::perform_measurements,
            )
                .chain()
                .in_schedule(CoreSchedule::FixedUpdate),
        );
    }
}

impl GamePlugin {
    // TODO: how to come up with a good ID value?
    #[cfg(debug_assertions)]
    pub const DIAGNOSTIC_FRAME_DURATION: DiagnosticId =
        DiagnosticId::from_u128(1187582084072456577959028643519383692);
    #[cfg(debug_assertions)]
    pub const DIAGNOSTIC_TIME_ACCRUED: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339571239028643519383692);

    fn fixed_time() -> FixedTime {
        let desired_frame_duration = Duration::from_nanos(1_000_000_000 / DESIRED_FPS);
        let mut fixed_time = FixedTime::new(desired_frame_duration);

        // we need to advance timer by a length of a single frame, otherwise we would see an empty canvas during 1st frame
        fixed_time.tick(desired_frame_duration);

        fixed_time
    }

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

    fn clear_screen(mut pixel_canvas: ResMut<PixelCanvas>) {
        pixel_canvas.clear(Pico8Color::DarkBlue.into());
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
