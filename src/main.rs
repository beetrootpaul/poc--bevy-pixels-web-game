use std::time::Duration;

use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowResolution};
use bevy::winit::WinitPlugin;

fn main() {
    let mut app = App::new();

    // TODO: ???
    // app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
    //     1.0 / 60.0,
    // )));

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // TODO: extract game title as constant
            title: "Bevy/pixels web game PoC".to_string(),
            // TODO: extract window size as constants
            resolution: WindowResolution::new(512., 512.).with_scale_factor_override(1.),
            // TODO: any other props to set?
            ..default()
        }),
        ..default()
    }));

    // TODO: ImagePlugin::default_nearest()
    #[cfg(debug_assertions)]
    app.add_system(close_on_esc);

    app.run();
}

// TODO: anything left in https://github.com/bevyengine/bevy/tree/main/examples worth applying on this app?

// TODO: TESTS: https://chadnauseam.com/coding/gamedev/automated-testing-in-bevy

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
