use bevy::prelude::*;

pub struct AudioSystems;

#[derive(Resource, Default)]
pub struct AudioFiles {
    pub music_base: Handle<AudioSource>,
}

impl AudioFiles {
    pub fn all_handles(&self) -> Vec<Handle<AudioSource>> {
        vec![self.music_base.clone()]
    }
}

impl AudioSystems {
    pub fn load_music_files(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("Loading music files…");
        let handle = asset_server.load("music_base.ogg");
        commands.insert_resource(AudioFiles { music_base: handle });
    }

    pub fn play_music(
        audio: Res<Audio>,
        audio_files: Res<AudioFiles>,
        mut is_music_playing: Local<bool>,
    ) {
        if !*is_music_playing {
            info!("Play music…");
            *is_music_playing = true;
            audio.play_with_settings(audio_files.music_base.clone(), PlaybackSettings::LOOP);
        }
    }
}
