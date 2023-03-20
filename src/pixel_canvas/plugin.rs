use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use pixels::{Pixels, SurfaceTexture};

#[cfg(target_arch = "wasm32")]
use pollster::FutureExt;

use crate::game::{GameArea, GameAreaVariant, InputConfig};
use crate::pixel_canvas::PixelCanvas;

pub struct PixelCanvasPlugin {
    pub width: i32,
    pub height: i32,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[system_set(base)]
enum PixelCanvasSystemSet {
    RenderPixelCanvas,
}
#[derive(Resource)]
struct PixelCanvasConfig {
    width: i32,
    height: i32,
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
        winit_windows: NonSend<bevy::winit::WinitWindows>,
        canvas_config: Res<PixelCanvasConfig>,
        mut commands: Commands,
    ) {
        let primary_window = primary_window_query
            .get_single()
            .expect("should query single primary window");
        commands.insert_resource(Self::new_pixel_canvas(
            &winit_windows,
            primary_window,
            canvas_config.width,
            canvas_config.height,
        ));
    }

    fn render(pixel_canvas: Res<PixelCanvas>) {
        pixel_canvas.pixels.render().expect("should render pixels");
    }

    pub fn window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        primary_window_query: Query<Entity, With<PrimaryWindow>>,
        input_config: Res<InputConfig>,
        winit_windows: NonSend<bevy::winit::WinitWindows>,
        mut game_area: ResMut<GameArea>,
        mut commands: Commands,
    ) {
        for event in window_resized_events.iter() {
            let primary_window = primary_window_query
                .get_single()
                .expect("should query single primary window");
            if event.window == primary_window {
                if input_config.is_touch_available {
                    let winit_window = winit_windows
                        .get_window(primary_window)
                        .expect("should get winit window for a given primary window");
                    let w = winit_window.inner_size().width;
                    let h = winit_window.inner_size().height;
                    game_area.variant = if h > w {
                        GameAreaVariant::PortraitControls
                    } else {
                        GameAreaVariant::LandscapeControls
                    };
                } else {
                    game_area.variant = GameAreaVariant::NoControls;
                }

                commands.insert_resource(Self::new_pixel_canvas(
                    &winit_windows,
                    primary_window,
                    game_area.outer_width(),
                    game_area.outer_height(),
                ));
            }
        }
    }

    fn new_pixel_canvas(
        winit_windows: &NonSend<bevy::winit::WinitWindows>,
        primary_window: Entity,
        width: i32,
        height: i32,
    ) -> PixelCanvas {
        let winit_window = winit_windows
            .get_window(primary_window)
            .expect("should get winit window for a given primary window");

        let surface_texture = SurfaceTexture::new(
            winit_window.inner_size().width,
            winit_window.inner_size().height,
            winit_window,
        );

        let mut pixels = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Pixels::new(
                    u32::try_from(width).unwrap(),
                    u32::try_from(height).unwrap(),
                    surface_texture,
                )
            }
            #[cfg(target_arch = "wasm32")]
            {
                Pixels::new_async(
                    u32::try_from(width).unwrap(),
                    u32::try_from(height).unwrap(),
                    surface_texture,
                )
                .block_on()
            }
        }
        .expect("should create pixels");

        // It would be nice to use any RGB8 color (e.g. one from Pico8Color enum), but
        //   apparently they do not translate correctly when just dividing by 255.0.
        //   Therefore, let's just use pure black.
        pixels.set_clear_color(pixels::wgpu::Color::BLACK);

        PixelCanvas {
            pixels,
            width: width as usize,
            height: height as usize,
        }
    }
}
