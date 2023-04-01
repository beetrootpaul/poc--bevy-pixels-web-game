use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Entity, Query, With};

use crate::game::coin::Coin;
use crate::game::collision::{Collision, HitCircle};
use crate::game::player::Player;
use crate::game::position::Position;

pub struct CollectCoinsSystems;

impl CollectCoinsSystems {
    pub fn collect_coins(
        mut commands: Commands,
        players_query: Query<(&Position, &HitCircle), With<Player>>,
        coins_query: Query<(Entity, &Position, &HitCircle), With<Coin>>,
    ) {
        for (player_position, player_hit_circle) in players_query.iter() {
            for (coin_entity, coin_position, coin_hit_circle) in coins_query.iter() {
                if Collision::are_colliding(
                    player_position,
                    player_hit_circle,
                    coin_position,
                    coin_hit_circle,
                ) {
                    commands.entity(coin_entity).despawn_recursive();
                }
            }
        }
    }
}
