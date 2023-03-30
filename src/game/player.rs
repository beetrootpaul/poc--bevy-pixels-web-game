use bevy::math::ivec2;
use bevy::prelude::*;

use crate::game::game_area::GameArea;
use crate::game::position::Position;
use crate::game::sprites::{Sprite, SpriteSheet};
use crate::irect::IRect;
use crate::pico8::Pico8Color;
use crate::pixel_canvas::PixelCanvas;

const MOVEMENT_PER_FRAME: i32 = 1;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    player_movement: PlayerMovement,
    position: Position,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
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
            player_movement: initial_movement,
            position: Position(ivec2(1, 1)),
        });
    }

    pub fn move_player(
        mut query: Query<(&PlayerMovement, &mut Position)>,
        game_area: Res<GameArea>,
    ) {
        for (player_movement, mut position) in query.iter_mut() {
            match player_movement {
                PlayerMovement::Left => position.0.x -= MOVEMENT_PER_FRAME,
                PlayerMovement::Right => position.0.x += MOVEMENT_PER_FRAME,
                PlayerMovement::Up => position.0.y -= MOVEMENT_PER_FRAME,
                PlayerMovement::Down => position.0.y += MOVEMENT_PER_FRAME,
            }

            position.0 = position.0.clamp(
                ivec2(0, 0),
                ivec2(game_area.width() - 1, game_area.height() - 1),
            );
        }
    }

    pub fn draw_player(
        sprite_sheet: ResMut<SpriteSheet>,
        query: Query<(&Position, &PlayerMovement), With<Player>>,
        mut pixel_canvas: ResMut<PixelCanvas>,
        game_area: Res<GameArea>,
    ) {
        for (position, player_movement) in query.iter() {
            let sprite = Self::get_sprite_for_movement(player_movement);
            let xy = game_area.game_area_xy_from(position.0);
            pixel_canvas.draw_sprite(
                xy,
                &sprite_sheet.main,
                sprite.sheet_rect,
            );

            let size = ivec2(20, 20);
            pixel_canvas.draw_ellipse_filled(
                IRect {
                     top_left: xy,
                    size
                },
                Pico8Color::LightPeach.into()
            )
        }
    }

    fn get_sprite_for_movement(movement: &PlayerMovement) -> Sprite {
        match *movement {
            PlayerMovement::Left => SpriteSheet::PLAYER_LEFT,
            PlayerMovement::Right => SpriteSheet::PLAYER_RIGHT,
            PlayerMovement::Up => SpriteSheet::PLAYER_UP,
            PlayerMovement::Down => SpriteSheet::PLAYER_DOWN,
        }
    }
}
