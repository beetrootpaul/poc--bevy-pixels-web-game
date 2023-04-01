use std::time::Duration;

#[cfg(debug_assertions)]
use bevy::diagnostic::{DiagnosticId, Diagnostics};
#[cfg(debug_assertions)]
use bevy::diagnostic::Diagnostic;
use bevy::prelude::*;

use FixedFpsSystemSet::{FixedFpsLast, FixedFpsSpawning, FixedFpsUpdateAndDraw};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::game::audio::AudioSystems;
use crate::game::coin::CoinSystems;
use crate::game::collision::CollisionSystems;
pub use crate::game::game_area::{GameArea, GameAreaVariant};
use crate::game::game_state::GameState;
use crate::game::input::{GamepadControlsSystems, KeyboardControlsSystems, TouchControlsSystems};
pub use crate::game::input::InputConfig;
use crate::game::player::PlayerSystems;
use crate::game::sprites::SpritesSystems;
use crate::game::text::TextSystems;
use crate::game::top_bar::TopBarSystems;
use crate::game::trail::TrailSystems;
use crate::pico8::{Pico8Color, Pico8FontSystems};
use crate::pixel_canvas::{PixelCanvas, PixelCanvasPlugin};

mod audio;
mod coin;
mod collision;
mod game_area;
mod game_state;
mod input;
mod player;
mod position;
mod sprites;
mod text;
mod top_bar;
mod trail;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn __is_touch_available__() -> bool;
}

pub const GAME_TITLE: &str = "Bevy/pixels web game PoC";

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
        app.insert_resource(InputConfig {
            #[cfg(not(target_arch = "wasm32"))]
            is_touch_available: false,
            #[cfg(target_arch = "wasm32")]
            is_touch_available: __is_touch_available__(),
        });

        let game_area = GameArea {
            variant: GameAreaVariant::NoControls,
        };
        let game_area_outer_w = game_area.outer_width();
        let game_area_outer_h = game_area.outer_height();
        app.insert_resource(game_area);

        app.add_plugin(PixelCanvasPlugin {
            width: game_area_outer_w,
            height: game_area_outer_h,
        });

        app.add_state::<GameState>();
        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_measurements);
        app.add_startup_system(SpritesSystems::load_sprite_sheet);
        app.add_startup_system(Pico8FontSystems::load_font_sheet);
        app.add_startup_system(AudioSystems::load_music_files);

        app.add_system(
            TouchControlsSystems::spawn_touch_controls
                .in_base_set(CoreSet::Update)
                .run_if(GameState::is_game_loaded),
        );
        app.add_system(
            TouchControlsSystems::handle_touch_input
                .in_base_set(CoreSet::Update)
                .run_if(GameState::is_game_loaded),
        );
        app.add_system(
            GamepadControlsSystems::handle_gamepad_input
                .in_base_set(CoreSet::Update)
                .run_if(GameState::is_game_loaded),
        );
        app.add_system(
            KeyboardControlsSystems::handle_keyboard_input
                .in_base_set(CoreSet::Update)
                .run_if(GameState::is_game_loaded),
        );
        app.add_system(AudioSystems::play_music.run_if(GameState::is_game_running));

        app.insert_resource(Self::fixed_time());
        app.edit_schedule(CoreSchedule::FixedUpdate, |schedule| {
            schedule
                .configure_sets((FixedFpsSpawning, FixedFpsUpdateAndDraw, FixedFpsLast).chain());
            schedule.add_system(
                TopBarSystems::spawn_top_bar
                    .run_if(TopBarSystems::there_is_no_top_bar)
                    .in_set(FixedFpsSpawning)
                    .run_if(GameState::is_game_running),
            );
            schedule.add_system(
                PlayerSystems::spawn_player
                    .run_if(PlayerSystems::there_is_no_player)
                    .in_set(FixedFpsSpawning)
                    .run_if(GameState::is_game_running),
            );
            schedule.add_system(
                CoinSystems::spawn_coin
                    .run_if(CoinSystems::there_is_no_coin)
                    .in_set(FixedFpsSpawning)
                    .run_if(GameState::is_game_running),
            );
            schedule.add_system(
                apply_system_buffers
                    .after(FixedFpsSpawning)
                    .before(FixedFpsUpdateAndDraw),
            );
            schedule.add_system(
                Self::clear_canvas
                    .before(FixedFpsUpdateAndDraw)
                    .run_if(GameState::is_game_loaded),
            );
            schedule.add_systems(
                (
                    PlayerSystems::move_player.run_if(GameState::is_game_running),
                    TrailSystems::update_trails.run_if(GameState::is_game_running),
                    TrailSystems::update_particles.run_if(GameState::is_game_running),
                    TrailSystems::draw_particles,
                    CoinSystems::draw_coin,
                    PlayerSystems::draw_player,
                    CollisionSystems::draw_hit_circles,
                    TextSystems::draw_texts,
                )
                    .chain()
                    .in_set(FixedFpsUpdateAndDraw),
            );
            schedule.add_system(
                TouchControlsSystems::draw_touch_controls
                    .run_if(GameState::is_game_loaded)
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

    fn clear_canvas(mut pixel_canvas: ResMut<PixelCanvas>, game_area: Res<GameArea>) {
        pixel_canvas.draw_rect_filled(game_area.rect(), Pico8Color::DarkBlue.into());
        for outer_rect in game_area.outer_rects().iter() {
            pixel_canvas.draw_rect_filled(*outer_rect, Pico8Color::Black.into());
        }
    }

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
            f64::from(u32::try_from(fixed_time.accumulated().as_millis()).unwrap())
        });
        *prev_elapsed_seconds = time.raw_elapsed_seconds_f64();
    }
}
