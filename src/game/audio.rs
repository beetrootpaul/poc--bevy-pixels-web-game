use bevy::prelude::*;
use image::EncodableLayout;

pub struct AudioSystems;

#[derive(Resource, Default)]
pub struct AudioFiles {
    pub music_base: Handle<AudioSource>,
}

// TODO: assets loading from https://github.com/NiklasEi/bevy_game_template

// TODO: add a button which mutes all sounds

impl AudioSystems {
    pub fn load_music_files(mut commands: Commands, mut audio_assets: ResMut<Assets<AudioSource>>) {
        let bytes: &[u8] = include_bytes!("../../assets/music_base.ogg");
        let handle = audio_assets.add(AudioSource {
            bytes: bytes.as_bytes().into(),
        });
        commands.insert_resource(AudioFiles { music_base: handle });
    }

    pub fn play_music(
        audio: Res<Audio>,
        audio_files: Res<AudioFiles>,
        // TODO: another approach to "let's start music, once"?
        mut is_music_playing: Local<bool>,
    ) {
        if !*is_music_playing {
            // TODO: make it debug and visible as web console log?
            info!("Play music…");
            *is_music_playing = true;
            audio.play_with_settings(audio_files.music_base.clone(), PlaybackSettings::LOOP);
        }
    }
}
