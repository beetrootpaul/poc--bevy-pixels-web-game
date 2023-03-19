use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
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

impl Plugin for PixelCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelCanvasConfig {
            width: self.width,
            height: self.height,
        });

        app.add_startup_system(Self::setup);

        app.add_system(Self::window_resize);

        app.configure_set(PixelCanvasSystemSet::RenderPixelCanvas.after(CoreSet::PostUpdate));
        app.add_system(Self::render.in_base_set(PixelCanvasSystemSet::RenderPixelCanvas));
    }
}

impl PixelCanvasPlugin {
    fn setup(
        primary_window_query: Query<Entity, With<PrimaryWindow>>,
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
                    canvas_config.width as u32,
                    canvas_config.height as u32,
                    surface_texture,
                )
            }
            #[cfg(target_arch = "wasm32")]
            {
                Pixels::new_async(
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

    fn render(pixel_canvas: Res<PixelCanvas>) {
        pixel_canvas.pixels.render().expect("should render pixels");
    }

    pub fn window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        primary_window_query: Query<Entity, With<PrimaryWindow>>,
        winit_windows: NonSend<WinitWindows>,
        mut pixel_canvas: ResMut<PixelCanvas>,
    ) {
        for event in window_resized_events.iter() {
            let primary_window = primary_window_query
                .get_single()
                .expect("should query single primary window");
            let winit_window = winit_windows
                .get_window(primary_window)
                .expect("should get winit window for a given primary window");
            if event.window == primary_window {
                pixel_canvas
                    .pixels
                    .resize_surface(
                        winit_window.inner_size().width,
                        winit_window.inner_size().height,
                    )
                    .expect("should resize pixels surface");
            }
        }
    }
}
