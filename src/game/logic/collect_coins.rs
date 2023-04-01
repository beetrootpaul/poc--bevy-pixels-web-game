use crate::game::audio::{AudioFiles, Playback};
use bevy::asset::Assets;
use bevy::audio::{Audio, AudioSink, PlaybackSettings};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Entity, Query, Res, With};

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
        audio: Res<Audio>,
        audio_files: Res<AudioFiles>,
        playback: Res<Playback>,
        audio_sinks: Res<Assets<AudioSink>>,
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
                    audio.play_with_settings(
                        audio_files.sfx_coin_collected.clone(),
                        PlaybackSettings::ONCE,
                    );
                    playback.toggle_melody(audio_sinks.as_ref());
                }
            }
        }
    }
}
