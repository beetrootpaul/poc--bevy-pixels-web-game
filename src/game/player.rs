use bevy::math::ivec2;
use bevy::prelude::*;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::{Point, Primitive};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment};
use embedded_graphics::Drawable;

use crate::game::game_area::GameArea;
use crate::game::position::Position;
use crate::game::sprites::{Sprite, SpriteSheet};
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
            player_movement: initial_movement.clone(),
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
            let game_area_xy = game_area.game_area_xy_from(position.0);

            let sprite = Self::get_sprite_for_movement(player_movement);
            pixel_canvas.draw_sprite(game_area_xy, &sprite_sheet.main, sprite.sheet_rect);
            let s_w = sprite.sheet_rect.1.x - sprite.sheet_rect.0.x;
            let s_h = sprite.sheet_rect.1.y - sprite.sheet_rect.0.y;

            let style = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::new(
                    Pico8Color::Blue.rgb8().0,
                    Pico8Color::Blue.rgb8().1,
                    Pico8Color::Blue.rgb8().2,
                ))
                .stroke_alignment(StrokeAlignment::Center)
                .stroke_width(1)
                .build();

            let c_r1 = 30;
            Circle::new(
                Point::new(
                    s_w / 2 + game_area_xy.x - c_r1 / 2,
                    s_h / 2 + game_area_xy.y - c_r1 / 2,
                ),
                u32::try_from(c_r1).unwrap(),
            )
            // .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 1))
            .into_styled(style)
            .draw(pixel_canvas.as_mut())
            .expect("lol 1");

            let style = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::RED)
                .stroke_width(1)
                .fill_color(Rgb888::GREEN)
                .build();

            let c_r2 = 20;
            Circle::new(
                Point::new(
                    s_w / 2 + game_area_xy.x - c_r2 / 2,
                    s_h / 2 + game_area_xy.y - c_r2 / 2,
                ),
                u32::try_from(c_r2).unwrap(),
            )
            .into_styled(style)
            .draw(pixel_canvas.as_mut())
            .expect("lol 2");
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
