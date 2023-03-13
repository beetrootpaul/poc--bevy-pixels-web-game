use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::game::{GamePlugin, GAME_AREA_HEIGHT, GAME_AREA_WIDTH, GAME_TITLE};

mod game;
mod pico8;
mod pixel_canvas;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

// TODO: improve window size management in relation to pixel canvas

fn main() {
    assert_eq!(WINDOW_WIDTH % GAME_AREA_WIDTH, 0);
    assert_eq!(WINDOW_HEIGHT % GAME_AREA_HEIGHT, 0);
    assert_eq!(
        WINDOW_WIDTH % GAME_AREA_WIDTH,
        WINDOW_HEIGHT % GAME_AREA_HEIGHT
    );

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);

    #[cfg(all(
        not(feature = "visualize_schedule_main"),
        not(feature = "visualize_schedule_fixed_update")
    ))]
    app.add_plugin(bevy::log::LogPlugin::default());

    app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default());

    app.add_plugin(bevy::input::InputPlugin::default());

    app.add_plugin(WindowPlugin {
        primary_window: Some(Window {
            title: GAME_TITLE.to_string(),
            // TODO: better way for number type conversion?
            resolution: WindowResolution::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            // TODO: any other props to set?
            ..default()
        }),
        ..default()
    });
    app.add_plugin(bevy::a11y::AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());

    app.add_plugin(bevy::asset::AssetPlugin::default());
    app.add_plugin(bevy::render::texture::ImagePlugin::default());
    app.add_asset::<TextureAtlas>()
        .register_asset_reflect::<TextureAtlas>();

    // https://bevy-cheatbook.github.io/cookbook/print-framerate.html
    #[cfg(debug_assertions)]
    app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

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
            CoreSchedule::Main,
            &bevy_mod_debugdump::schedule_graph::Settings::default(),
        )
    );
    #[cfg(feature = "visualize_schedule_fixed_update")]
    println!(
        "{}",
        bevy_mod_debugdump::schedule_graph_dot(
            &mut app,
            CoreSchedule::FixedUpdate,
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

// TODO: WASM: https://github.com/bevyengine/bevy/tree/main/examples#wasm
// TODO: WASM: https://bfnightly.bracketproductions.com/rustbook/webbuild.html

// TODO: anything from this list?
//       - [ahash](https://crates.io/crates/ahash)
//       - [bitflags](https://crates.io/crates/bitflags)
//       - [bracket-pathfinding](https://crates.io/crates/bracket-pathfinding)
//       - [bracket-random](https://crates.io/crates/bracket-random)
//       - [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook)
//       - [console_log](https://crates.io/crates/console_log)
//       - [egui](https://crates.io/crates/egui)
//       - [egui-wgpu](https://crates.io/crates/egui-wgpu)
//       - [egui-winit](https://crates.io/crates/egui-winit)
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
//       - [pollster](https://crates.io/crates/pollster)
//       - [rand](https://crates.io/crates/rand)
//       - [resources](https://crates.io/crates/resources)
//       - [rotsprite](https://crates.io/crates/rotsprite)
//       - [rust-embed](https://crates.io/crates/rust-embed)
//       - [rustc-hash](https://crates.io/crates/rustc-hash)
//       - [wasm-bindgen](https://crates.io/crates/wasm-bindgen)
//       - [wasm-bindgen-futures](https://crates.io/crates/wasm-bindgen-futures)
//       - [web-sys](https://crates.io/crates/web-sys)
//       - [wgpu](https://crates.io/crates/wgpu)
//       - [winit](https://crates.io/crates/winit)
//       - [winit_input_helper](https://crates.io/crates/winit_input_helper)

// TODO: check license of each dependency

// TODO: https://github.com/jakobhellermann/bevy_mod_debugdump
