extern crate core;

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowResolution;

use crate::game::{GamePlugin, GAME_TITLE};

mod game;
mod irect;
mod pico8;
mod pixel_canvas;

#[cfg(target_arch = "wasm32")]
const HTML_CANVAS_SELECTOR: &str = "#bevy_pixels_web_game_poc__canvas";

fn main() {
    // https://bevy-cheatbook.github.io/platforms/wasm/panic-console.html#panic-messages
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = bevy::app::App::new();

    app.add_plugins(bevy::MinimalPlugins);

    app.add_plugin(bevy::log::LogPlugin::default());

    #[cfg(debug_assertions)]
    app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

    app.add_plugin(bevy::window::WindowPlugin {
        primary_window: Some(bevy::window::Window {
            title: GAME_TITLE.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            resolution: WindowResolution::new(1024.0, 512.0),
            #[cfg(target_arch = "wasm32")]
            canvas: Some(HTML_CANVAS_SELECTOR.to_string()),
            // Crucial for `pixels` to render with a preserved aspect ratio instead of stretching to fill canvas whole:
            #[cfg(target_arch = "wasm32")]
            fit_canvas_to_parent: true,
            #[cfg(target_arch = "wasm32")]
            prevent_default_event_handling: false,
            ..bevy::utils::default()
        }),
        ..bevy::utils::default()
    });
    app.add_plugin(bevy::a11y::AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());

    app.add_plugin(bevy::input::InputPlugin::default());
    app.add_plugin(bevy::gilrs::GilrsPlugin::default());

    app.add_plugin(bevy::asset::AssetPlugin::default());
    app.add_plugin(bevy::audio::AudioPlugin::default());

    app.add_plugin(GamePlugin);

    #[cfg(debug_assertions)]
    app.add_system(bevy::window::close_on_esc);

    #[cfg(feature = "visualize_schedule_main")]
    println!(
        "{}",
        bevy_mod_debugdump::schedule_graph_dot(
            &mut app,
            bevy::app::CoreSchedule::Main,
            &bevy_mod_debugdump::schedule_graph::Settings::default(),
        )
    );
    #[cfg(feature = "visualize_schedule_fixed_update")]
    println!(
        "{}",
        bevy_mod_debugdump::schedule_graph_dot(
            &mut app,
            bevy::app::CoreSchedule::FixedUpdate,
            &bevy_mod_debugdump::schedule_graph::Settings::default(),
        )
    );

    #[cfg(all(
        not(feature = "visualize_schedule_main"),
        not(feature = "visualize_schedule_fixed_update")
    ))]
    app.run();
}
