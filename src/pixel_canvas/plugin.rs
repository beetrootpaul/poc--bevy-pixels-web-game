use crate::pixel_canvas::PixelCanvas;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use pixels::{Pixels, SurfaceTexture};

pub struct PixelCanvasPlugin {
    pub width: usize,
    pub height: usize,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[system_set(base)]
enum PixelCanvasSystemSet {
    RenderPixelCanvas,
}
#[derive(Resource)]
struct PixelCanvasConfig {
    width: usize,
    height: usize,
}

// TODO: check if there is a new release of bevy_pixels, adjusted for Bevy 0.10
impl Plugin for PixelCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelCanvasConfig {
            width: self.width,
            height: self.height,
        });

        app.add_startup_system(Self::setup);

        app.configure_set(PixelCanvasSystemSet::RenderPixelCanvas.after(CoreSet::PostUpdate));
        app.add_system(Self::render.in_base_set(PixelCanvasSystemSet::RenderPixelCanvas));
    }
}

impl PixelCanvasPlugin {
    fn setup(
        primary_window_query: Query<Entity, With<PrimaryWindow>>,
        // TODO:: what does NonSend do?
        winit_windows: NonSend<WinitWindows>,
        canvas_config: Res<PixelCanvasConfig>,
        mut commands: Commands,
    ) {
        let primary_window = primary_window_query
            .get_single()
            .expect("should query single primary window");

        let winit_window = winit_windows
            .get_window(primary_window)
            .expect("should get winit window for a given primary window");

        let surface_texture = SurfaceTexture::new(
            winit_window.inner_size().width,
            winit_window.inner_size().height,
            winit_window,
        );

        // TODO: WASM: Pixels::new_async(canvas_width, canvas_height, surface_texture).block_on()
        let pixels = Pixels::new(
            // TODO: better way for number type conversion?
            canvas_config.width as u32,
            canvas_config.height as u32,
            surface_texture,
        )
        .expect("should create pixels");

        commands.insert_resource(PixelCanvas {
            pixels,
            width: canvas_config.width,
            height: canvas_config.height,
        })
    }

    fn render(resource: Res<PixelCanvas>) {
        resource.pixels.render().expect("should render pixels");
    }
}

// TODO: bevy_pixels for Bevy 0.9 . Anything else to be taken from here?
/*
pub mod prelude {
    pub use crate::{PixelsPlugin, PixelsResource, PixelsStage};
}

pub use pixels;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::*,
    window::{WindowBackendScaleFactorChanged, WindowId, WindowResized},
    winit::WinitWindows,
};
use pixels::{Pixels, SurfaceTexture};
#[cfg(target_arch = "wasm32")]
use pollster::FutureExt as _;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PixelsStage {
    Draw,
    Render,
    PostRender,
}

#[derive(Resource)]
pub struct PixelsResource {
    pub pixels: Pixels,
    pub window_id: WindowId,
}

// Internal configuration resource for use in `setup` system. Users should set values on
// `PixelsPlugin` instead of inserting this resource directly. Ideally we just read the plugin
// configuration directly within `setup` system, but this is not currently possible.
#[derive(Resource)]
pub struct PixelsOptions {
    width: u32,
    height: u32,
}

pub struct PixelsPlugin {
    /// Width of the pixel buffer
    pub width: u32,
    /// Height of the pixel buffer
    pub height: u32,
}

impl Default for PixelsPlugin {
    fn default() -> Self {
        PixelsPlugin {
            width: 180,
            height: 120,
        }
    }
}

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelsOptions {
            width: self.width,
            height: self.height,
        })
        .add_stage_after(
            CoreStage::PostUpdate,
            PixelsStage::Draw,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PixelsStage::Draw,
            PixelsStage::Render,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PixelsStage::Render,
            PixelsStage::PostRender,
            SystemStage::parallel(),
        )
        .add_startup_system_to_stage(StartupStage::PreStartup, Self::setup)
        .add_system(Self::window_resize)
        .add_system(Self::window_change)
        .add_system_to_stage(PixelsStage::Render, Self::render);
    }
}

impl PixelsPlugin {
    pub const RENDER_TIME: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339577959028643519383692);

    pub fn setup(
        mut commands: Commands,
        mut diagnostics: ResMut<Diagnostics>,
        options: Res<PixelsOptions>,
        windows: Res<Windows>,
        winit_windows: NonSend<WinitWindows>,
    ) {
        diagnostics.add(Diagnostic::new(Self::RENDER_TIME, "render_time", 20).with_suffix("s"));

        let window_id = windows
            .get_primary()
            .expect("primary window not found")
            .id();

        let winit_window = winit_windows
            .get_window(window_id)
            .expect("failed to get primary winit window");

        let window_size = winit_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, winit_window);

        let pixels = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Pixels::new(options.width, options.height, surface_texture)
            }
            #[cfg(target_arch = "wasm32")]
            {
                // TODO: Find a way to asynchronously load pixels on web
                Pixels::new_async(options.width, options.height, surface_texture).block_on()
            }
        }
        .expect("failed to create pixels");

        commands.insert_resource(PixelsResource { pixels, window_id });
    }

    pub fn window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        mut resource: ResMut<PixelsResource>,
        windows: Res<Windows>,
    ) {
        for event in window_resized_events.iter() {
            if event.id == resource.window_id {
                Self::resize_surface_to_window(&mut resource, &windows);
            }
        }
    }

    pub fn window_change(
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut resource: ResMut<PixelsResource>,
        windows: Res<Windows>,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            if event.id == resource.window_id {
                Self::resize_surface_to_window(&mut resource, &windows);
            }
        }
    }

    fn resize_surface_to_window(resource: &mut ResMut<PixelsResource>, windows: &Res<Windows>) {
        let window = windows.get(resource.window_id).unwrap();

        let _ = resource
            .pixels
            .resize_surface(window.physical_width(), window.physical_height());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn render(resource: Res<PixelsResource>, mut diagnostics: ResMut<Diagnostics>) {
        let start = Instant::now();

        resource.pixels.render().expect("failed to render pixels");

        let end = Instant::now();
        let render_time = end.duration_since(start);
        diagnostics.add_measurement(Self::RENDER_TIME, || render_time.as_secs_f64());
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render(resource: Res<PixelsResource>) {
        resource.pixels.render().expect("failed to render pixels");
    }
}

*/
