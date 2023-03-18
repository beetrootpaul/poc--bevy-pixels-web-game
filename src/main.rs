use bevy::window::WindowResolution;

use crate::game::{GamePlugin, GAME_AREA_HEIGHT, GAME_AREA_WIDTH, GAME_TITLE};

mod game;
mod pico8;
mod pixel_canvas;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

#[cfg(target_arch = "wasm32")]
const HTML_CANVAS_SELECTOR: &str = "#bevy_pixels_web_game_poc__canvas";

// TODO: improve window size management in relation to pixel canvas

fn main() {
    assert_eq!(WINDOW_WIDTH % GAME_AREA_WIDTH, 0);
    assert_eq!(WINDOW_HEIGHT % GAME_AREA_HEIGHT, 0);
    assert_eq!(
        WINDOW_WIDTH % GAME_AREA_WIDTH,
        WINDOW_HEIGHT % GAME_AREA_HEIGHT
    );

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
            // TODO: better way for number type conversion?
            resolution: WindowResolution::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            #[cfg(target_arch = "wasm32")]
            canvas: Some(HTML_CANVAS_SELECTOR.to_string()),
            // TODO: any other props to set?
            ..bevy::utils::default()
        }),
        ..bevy::utils::default()
    });
    app.add_plugin(bevy::a11y::AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());

    app.add_plugin(bevy::input::InputPlugin::default());

    app.add_plugin(bevy::asset::AssetPlugin::default());
    app.add_plugin(bevy::audio::AudioPlugin::default());

    app.add_plugin(GamePlugin);

    // TODO: ImagePlugin::default_nearest()
    //       comment: Prevent blurring of scaled up pixel art sprites

    // TODO: app.insert_resource(Msaa { samples: 1 });
    //       comment: Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet

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

// TODO: anything left in https://github.com/bevyengine/bevy/tree/main/examples worth applying on this app?

// TODO: TESTS: https://chadnauseam.com/coding/gamedev/automated-testing-in-bevy
// TODO: TESTS: https://github.com/bevyengine/bevy/blob/latest/tests/how_to_test_systems.rs
// TODO: TESTS: https://bevy-cheatbook.github.io/programming/system-tests.html

// TODO: anything from this list?
//       - [ahash](https://crates.io/crates/ahash)
//       - [bitflags](https://crates.io/crates/bitflags)
//       - [console_log](https://crates.io/crates/console_log)
//       - [env_logger](https://crates.io/crates/env_logger)
//       - [glam](https://crates.io/crates/glam)
//       - [hecs](https://crates.io/crates/hecs)
//       - [image](https://crates.io/crates/image)
//       - [indexmap](https://crates.io/crates/indexmap)
//       - [instant](https://crates.io/crates/instant)
//       - [kira](https://crates.io/crates/kira)
//       - [lazy_static](https://crates.io/crates/lazy_static)
//       - [log](https://crates.io/crates/log)
//       - [num](https://crates.io/crates/num)
//       - [rand](https://crates.io/crates/rand)
//       - [resources](https://crates.io/crates/resources)
//       - [rust-embed](https://crates.io/crates/rust-embed)
//       - [rustc-hash](https://crates.io/crates/rustc-hash)
//       - [wasm-bindgen-futures](https://crates.io/crates/wasm-bindgen-futures)
//       - [web-sys](https://crates.io/crates/web-sys)

// TODO: check license of each dependency
