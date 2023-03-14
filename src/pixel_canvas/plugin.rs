use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use pixels::{Pixels, SurfaceTexture};

#[cfg(target_arch = "wasm32")]
use pollster::FutureExt;

use crate::pixel_canvas::PixelCanvas;

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

        let pixels = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Pixels::new(
                    // TODO: better way for number type conversion?
                    canvas_config.width as u32,
                    canvas_config.height as u32,
                    surface_texture,
                )
            }
            #[cfg(target_arch = "wasm32")]
            {
                Pixels::new_async(
                    // TODO: better way for number type conversion?
                    canvas_config.width as u32,
                    canvas_config.height as u32,
                    surface_texture,
                )
                .block_on()
            }
        }
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
