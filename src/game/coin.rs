use bevy::log::debug;
use bevy::math::{ivec2, vec2};
use bevy::prelude::{Bundle, Commands, Component, Query, Res, ResMut, With};
use rand::Rng;

use crate::game::collision::HitCircle;
use crate::game::position::Position;
use crate::game::sprites::SpriteSheet;
use crate::game::GameArea;
use crate::pixel_canvas::PixelCanvas;

pub struct CoinSystems;

impl CoinSystems {
    pub fn there_is_no_coin(query: Query<(), With<Coin>>) -> bool {
        query.iter().count() == 0
    }

    pub fn spawn_coin(mut commands: Commands, game_area: Res<GameArea>) {
        debug!("> SPAWN COIN");

        let mut rng = rand::thread_rng();

        let margin = 10;
        let game_area_size = game_area.rect().size;

        commands.spawn(CoinBundle {
            coin: Coin,
            position: Position(ivec2(
                rng.gen_range(margin..(game_area_size.x - margin)),
                rng.gen_range(margin..(game_area_size.y - margin)),
            )),
            hit_circle: HitCircle {
                r: 5.0,
                offset: vec2(-5.0, -5.0),
            },
        });
    }

    pub fn draw_coin(
        sprite_sheet: Res<SpriteSheet>,
        query: Query<&Position, With<Coin>>,
        mut pixel_canvas: ResMut<PixelCanvas>,
        game_area: Res<GameArea>,
    ) {
        for position in query.iter() {
            let xy = game_area.game_area_xy_from(position.0);
            let sprite = SpriteSheet::COIN_FRAME_1;
            pixel_canvas.draw_sprite(
                xy - sprite.sheet_rect.size / 2,
                &sprite_sheet.main,
                sprite.sheet_rect,
            );
        }
    }
}

#[derive(Component)]
pub struct Coin;

#[derive(Bundle)]
struct CoinBundle {
    coin: Coin,
    position: Position,
    hit_circle: HitCircle,
}
