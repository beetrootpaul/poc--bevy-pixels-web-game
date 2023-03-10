use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowResolution};

use crate::pixel_art::{PixelCanvas, PixelCanvasPlugin, PixelCanvasSystemSet};

mod pixel_art;

fn main() {
    let mut app = App::new();

    // TODO: ???
    // app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
    //     1.0 / 60.0,
    // )));

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            // TODO: extract game title as constant
            title: "Bevy/pixels web game PoC".to_string(),
            // TODO: extract window size as constants
            resolution: WindowResolution::new(512., 512.),
            // TODO: any other props to set?
            ..default()
        }),
        ..default()
    };
    #[cfg(not(feature = "print_system_sets_diagram"))]
        let default_plugins: PluginGroupBuilder = DefaultPlugins.set(window_plugin);
    #[cfg(feature = "print_system_sets_diagram")]
        let default_plugins: PluginGroupBuilder = DefaultPlugins
        .set(window_plugin)
        .disable::<bevy::log::LogPlugin>();
    app.add_plugins(default_plugins);

    // TODO: extract canvas size as constants
    app.add_plugin(PixelCanvasPlugin {
        canvas_width: 16,
        canvas_height: 16,
    });

    // TODO: ImagePlugin::default_nearest()
    //       comment: Prevent blurring of scaled up pixel art sprites

    // TODO: app.insert_resource(Msaa { samples: 1 });
    //       comment: Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet

    #[cfg(debug_assertions)]
    app.add_system(close_on_esc);

    // TODO: TMP
    app.add_systems(
        (draw_background, draw_pixel)
            .chain()
            .in_base_set(PixelCanvasSystemSet::DrawPixelCanvas),
    );

    #[cfg(feature = "print_system_sets_diagram")]
    bevy_mod_debugdump::print_main_schedule(&mut app);

    #[cfg(not(feature = "print_system_sets_diagram"))]
    app.run();
}

// TODO: TMP
fn draw_background(mut pixels_resource: ResMut<PixelCanvas>) {
    // TODO: encapsulate frame access
    let frame = pixels_resource.pixels.get_frame_mut();
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

// TODO: TMP
fn draw_pixel(mut pixels_resource: ResMut<PixelCanvas>) {
    // TODO: encapsulate frame access
    let frame = pixels_resource.pixels.get_frame_mut();
    let pixel_index = 17;
    frame[4 * pixel_index] = 0xff;
    frame[4 * pixel_index + 1] = 0x00;
    frame[4 * pixel_index + 2] = 0x55;
    frame[4 * pixel_index + 3] = 0xff;
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
