use std::time::Duration;

use bevy::prelude::*;

pub use xy::Xy;

use crate::game::input::KeyboardControlsSystems;
use crate::game::player::PlayerSystems;
use crate::pixel_canvas::{PixelCanvas, PixelCanvasPlugin};

mod input;
mod player;
mod xy;

pub const GAME_TITLE: &str = "Bevy/pixels web game PoC";

// TODO: make it square
pub const GAME_AREA_WIDTH: u32 = 40;
pub const GAME_AREA_HEIGHT: u32 = 16;

const DESIRED_FPS: u64 = 30;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // TODO: extract canvas size as constants
        app.add_plugin(PixelCanvasPlugin {
            width: GAME_AREA_WIDTH,
            height: GAME_AREA_HEIGHT,
        });

        app.add_system(
            KeyboardControlsSystems::handle_keyboard_input.in_base_set(CoreSet::PreUpdate),
        );

        app.insert_resource(fixed_time());
        app.add_systems(
            (
                PlayerSystems::spawn_player.run_if(PlayerSystems::there_is_no_player),
                //
                // flush commands in order to have access to spawned player in the very same frame
                apply_system_buffers,
                //
                PlayerSystems::move_player,
                //
                clear_screen,
                PlayerSystems::draw_player,
                //
                log_fixed_timestep_measurements,
            )
                .chain()
                .in_schedule(CoreSchedule::FixedUpdate),
        );
    }
}

fn fixed_time() -> FixedTime {
    let desired_frame_duration = Duration::from_nanos(1_000_000_000 / DESIRED_FPS);
    let mut fixed_time = FixedTime::new(desired_frame_duration);

    // we need to advance timer by a length of a single frame, otherwise we would see an empty canvas during 1st frame
    fixed_time.tick(desired_frame_duration);

    fixed_time
}

// TODO: move somewhere else?
fn clear_screen(mut pixel_canvas: ResMut<PixelCanvas>) {
    debug!("> clear_screen");

    // TODO: encapsulate frame access
    let frame = pixel_canvas.pixels.get_frame_mut();
    // TODO: use desired PICO-8 color
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

// TODO: rework it to diagnostics
fn log_fixed_timestep_measurements(
    mut prev_elapsed_seconds: Local<f32>,
    time: Res<Time>,
    fixed_time: Res<FixedTime>,
) {
    info!(
        "time since last fixed_update: {:.0}ms",
        (time.raw_elapsed_seconds() - *prev_elapsed_seconds) * 1_000.
    );
    info!(
        "time accrued toward next fixed_update: {}μs",
        fixed_time.accumulated().as_micros()
    );
    *prev_elapsed_seconds = time.raw_elapsed_seconds();
}
