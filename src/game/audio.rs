use bevy::prelude::*;

pub struct AudioSystems;

#[derive(Resource, Default)]
pub struct AudioFiles {
    pub music_base: Handle<AudioSource>,
    pub music_melody: Handle<AudioSource>,
    pub music_percussion: Handle<AudioSource>,
    pub sfx_coin_collected: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct Playback {
    music_melody_sink_handle: Handle<AudioSink>,
}

impl Playback {
    pub fn toggle_melody(&self, audio_sinks: &Assets<AudioSink>) {
        if let Some(sink) = audio_sinks.get(&self.music_melody_sink_handle) {
            sink.set_volume(1.0 - sink.volume());
        }
    }
}

impl AudioFiles {
    pub fn all_handles(&self) -> Vec<Handle<AudioSource>> {
        vec![
            self.music_base.clone(),
            self.music_melody.clone(),
            self.music_percussion.clone(),
            self.sfx_coin_collected.clone(),
        ]
    }
}

impl AudioSystems {
    pub fn load_music_files(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("Loading music files…");
        let music_base = asset_server.load("music_base.ogg");
        let music_melody = asset_server.load("music_melody.ogg");
        let music_percussion = asset_server.load("music_percussion.ogg");
        let sfx_coin_collected = asset_server.load("sfx_coin_collected.ogg");
        commands.insert_resource(AudioFiles {
            music_base,
            music_melody,
            music_percussion,
            sfx_coin_collected,
        });
    }

    pub fn play_music(
        mut is_music_playing: Local<bool>,
        audio: Res<Audio>,
        audio_files: Res<AudioFiles>,
        audio_sinks: Res<Assets<AudioSink>>,
        mut commands: Commands,
    ) {
        if !*is_music_playing {
            info!("Play music…");
            *is_music_playing = true;
            audio.play_with_settings(audio_files.music_base.clone(), PlaybackSettings::LOOP);
            audio.play_with_settings(audio_files.music_percussion.clone(), PlaybackSettings::LOOP);
            let music_melody_sink_handle = audio_sinks.get_handle(audio.play_with_settings(
                audio_files.music_melody.clone(),
                PlaybackSettings::LOOP.with_volume(0.0),
            ));

            commands.insert_resource(Playback {
                music_melody_sink_handle,
            });
        }
    }
}
