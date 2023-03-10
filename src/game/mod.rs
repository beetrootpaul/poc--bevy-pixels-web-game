use bevy::prelude::*;

pub use xy::Xy;

use crate::game::input::GameInputPlugin;
use crate::game::player::PlayerPlugin;
use crate::pixel_art::{PixelCanvas, PixelCanvasPlugin, PixelCanvasSystemSet};

mod input;
mod player;
mod xy;

pub const GAME_TITLE: &str = "Bevy/pixels web game PoC";
// TODO: make it square
pub const GAME_AREA_WIDTH: u32 = 40;
pub const GAME_AREA_HEIGHT: u32 = 16;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // TODO: extract canvas size as constants
        app.add_plugin(PixelCanvasPlugin {
            width: GAME_AREA_WIDTH,
            height: GAME_AREA_HEIGHT,
        });

        app.configure_set(GameSystemSet::InputHandling.in_base_set(CoreSet::PreUpdate))
            .configure_set(GameSystemSet::SpawnPlayer.in_base_set(CoreSet::PreUpdate))
            .configure_set(GameSystemSet::UpdateGame.in_base_set(CoreSet::Update))
            .configure_sets(
                (GameSystemSet::DrawBackground, GameSystemSet::DrawPlayer)
                    .chain()
                    .in_base_set(PixelCanvasSystemSet::DrawPixelCanvas),
            );

        app.add_plugin(GameInputPlugin);

        app.add_plugin(PlayerPlugin);

        // TODO: move somewhere else?
        app.add_system(draw_background.in_set(GameSystemSet::DrawBackground));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSystemSet {
    InputHandling,
    SpawnPlayer,
    UpdateGame,
    DrawBackground,
    DrawPlayer,
}

// TODO: move somewhere else?
fn draw_background(mut pixel_canvas: ResMut<PixelCanvas>) {
    // TODO: encapsulate frame access
    let frame = pixel_canvas.pixels.get_frame_mut();
    // TODO: use desired PICO-8 color
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}
