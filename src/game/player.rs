use bevy::prelude::*;

use crate::game::game_area::GameArea;
use crate::game::sprites::SpriteSheet;
use crate::game::xy::Xy;
use crate::pixel_canvas::PixelCanvas;

const MOVEMENT_PER_FRAME: f32 = 1.;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    player_movement: PlayerMovement,
    xy: Xy,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub enum PlayerMovement {
    Left,
    Right,
    Up,
    Down,
}

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn there_is_no_player(query: Query<&Player>) -> bool {
        query.iter().count() == 0
    }

    pub fn spawn_player(mut commands: Commands) {
        debug!("> SPAWN PLAYER");

        let initial_movement = PlayerMovement::Right;

        commands.spawn(PlayerBundle {
            player: Player,
            player_movement: initial_movement.clone(),
            xy: Xy(1., 1.),
        });
    }

    pub fn move_player(mut query: Query<(&PlayerMovement, &mut Xy)>) {
        for (player_movement, mut xy) in query.iter_mut() {
            match player_movement {
                PlayerMovement::Left => xy.0 -= MOVEMENT_PER_FRAME,
                PlayerMovement::Right => xy.0 += MOVEMENT_PER_FRAME,
                PlayerMovement::Up => xy.1 -= MOVEMENT_PER_FRAME,
                PlayerMovement::Down => xy.1 += MOVEMENT_PER_FRAME,
            }

            xy.0 = xy.0.clamp(0., (GameArea::width() - 1) as f32);
            xy.1 = xy.1.clamp(0., (GameArea::height() - 1) as f32);
        }
    }

    pub fn draw_player(
        sprite_sheet: ResMut<SpriteSheet>,
        query: Query<(&Xy, &PlayerMovement), With<Player>>,
        mut pixel_canvas: ResMut<PixelCanvas>,
    ) {
        if let Some(rgba_image) = sprite_sheet.maybe_rgba_image.as_ref() {
            for (xy, player_movement) in query.iter() {
                let source_rect = SpriteSheet::source_rect_of_cell(
                    Self::get_sprite_index_for_movement(player_movement),
                );
                pixel_canvas.draw_sprite(
                    GameArea::game_area_xy_from(*xy),
                    rgba_image,
                    source_rect,
                );
            }
        }
    }

    fn get_sprite_index_for_movement(movement: &PlayerMovement) -> usize {
        match *movement {
            PlayerMovement::Left => SpriteSheet::PLAYER_LEFT,
            PlayerMovement::Right => SpriteSheet::PLAYER_RIGHT,
            PlayerMovement::Up => SpriteSheet::PLAYER_UP,
            PlayerMovement::Down => SpriteSheet::PLAYER_DOWN,
        }
    }
}
