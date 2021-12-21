use crate::prelude::EngineState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use std::array::IntoIter;

#[derive(Debug, Default)]
pub struct AudioManager {
    sfx_queue: Vec<(SfxPreset, f32)>,
    music_queue: Vec<Option<(MusicPreset, f32)>>,
    playing: AudioChannel,
    music_playing: bool,
}

impl AudioManager {
    /// Play a sound, `volume` ranges from `0.0` to `1.0`.
    pub fn play_sfx(&mut self, sfx_preset: SfxPreset, volume: f32) {
        self.sfx_queue.push((sfx_preset, volume.clamp(0.0, 1.0)));
    }
    /// Play looping music. `volume` ranges from `0.0` to `1.0`.  Any music already playing will be
    /// stopped.
    pub fn play_music(&mut self, music_preset: MusicPreset, volume: f32) {
        self.music_playing = true;
        self.music_queue
            .push(Some((music_preset, volume.clamp(0.0, 1.0))));
    }
    /// Stop any music currently playing
    pub fn stop_music(&mut self) {
        self.music_playing = false;
        self.music_queue.push(None);
    }
    /// Whether music is currently playing
    pub fn music_playing(&self) -> bool {
        self.music_playing
    }
}

#[derive(Default)]
pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(queue_managed_audio_system.system());
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SfxPreset {
    Click,
    Confirmation1,
    Confirmation2,
    Congratulations,
    Forcefield1,
    Forcefield2,
    Impact1,
    Impact2,
    Impact3,
    Jingle1,
    Jingle2,
    Jingle3,
    Minimize1,
    Minimize2,
    Switch1,
    Switch2,
    Tones1,
    Tones2,
}

impl SfxPreset {
    fn to_path(self) -> &'static str {
        match self {
            SfxPreset::Click => "audio/sfx/click.ogg",
            SfxPreset::Confirmation1 => "audio/sfx/confirmation1.ogg",
            SfxPreset::Confirmation2 => "audio/sfx/confirmation2.ogg",
            SfxPreset::Congratulations => "audio/sfx/congratulations.ogg",
            SfxPreset::Forcefield1 => "audio/sfx/forcefield1.ogg",
            SfxPreset::Forcefield2 => "audio/sfx/forcefield2.ogg",
            SfxPreset::Impact1 => "audio/sfx/impact1.ogg",
            SfxPreset::Impact2 => "audio/sfx/impact2.ogg",
            SfxPreset::Impact3 => "audio/sfx/impact3.ogg",
            SfxPreset::Jingle1 => "audio/sfx/jingle1.ogg",
            SfxPreset::Jingle2 => "audio/sfx/jingle2.ogg",
            SfxPreset::Jingle3 => "audio/sfx/jingle3.ogg",
            SfxPreset::Minimize1 => "audio/sfx/minimize1.ogg",
            SfxPreset::Minimize2 => "audio/sfx/minimize2.ogg",
            SfxPreset::Switch1 => "audio/sfx/switch1.ogg",
            SfxPreset::Switch2 => "audio/sfx/switch2.ogg",
            SfxPreset::Tones1 => "audio/sfx/tones1.ogg",
            SfxPreset::Tones2 => "audio/sfx/tones2.ogg",
        }
    }

    pub fn variant_iter() -> IntoIter<SfxPreset, 18> {
        static SFX_PRESETS: [SfxPreset; 18] = [
            SfxPreset::Click,
            SfxPreset::Confirmation1,
            SfxPreset::Confirmation2,
            SfxPreset::Congratulations,
            SfxPreset::Forcefield1,
            SfxPreset::Forcefield2,
            SfxPreset::Impact1,
            SfxPreset::Impact2,
            SfxPreset::Impact3,
            SfxPreset::Jingle1,
            SfxPreset::Jingle2,
            SfxPreset::Jingle3,
            SfxPreset::Minimize1,
            SfxPreset::Minimize2,
            SfxPreset::Switch1,
            SfxPreset::Switch2,
            SfxPreset::Tones1,
            SfxPreset::Tones2,
        ];
        SFX_PRESETS.into_iter()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MusicPreset {
    Classy8Bit,
    MysteriousMagic,
    WhimsicalPopsicle,
}

impl MusicPreset {
    fn to_path(self) -> &'static str {
        match self {
            MusicPreset::Classy8Bit => "audio/music/Classy 8-Bit.oga",
            MusicPreset::MysteriousMagic => "audio/music/Mysterious Magic.oga",
            MusicPreset::WhimsicalPopsicle => "audio/music/Whimsical Popsicle.oga",
        }
    }

    pub fn variant_iter() -> IntoIter<MusicPreset, 3> {
        static MUSIC_PRESETS: [MusicPreset; 3] = [
            MusicPreset::Classy8Bit,
            MusicPreset::MysteriousMagic,
            MusicPreset::WhimsicalPopsicle,
        ];
        MUSIC_PRESETS.into_iter()
    }
}

pub fn queue_managed_audio_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_state: ResMut<EngineState>,
) {
    for (sfx_preset, volume) in game_state.audio_manager.sfx_queue.drain(..) {
        let sfx_path = sfx_preset.to_path();
        let sfx_handle = asset_server.load(sfx_path);
        // To be able to set the volume of a sound effect, we need the channel it is being played
        // in. We'll start by naively creating a new channel for every single sound effect. If this
        // ends up being a performance or correctness problem, we'll need to circle back and do
        // something more sophisticated (like keep a set number of channels around at different
        // volumes).
        let new_sfx_channel = AudioChannel::new(sfx_path.into());
        audio.set_volume_in_channel(volume, &new_sfx_channel);
        audio.play_in_channel(sfx_handle, &new_sfx_channel);
    }
    let mut playing_music = game_state.audio_manager.playing.clone();
    for item in game_state.audio_manager.music_queue.drain(..) {
        audio.stop_channel(&playing_music);
        if let Some((music, volume)) = item {
            let music_path = music.to_path();
            let music_handle = asset_server.load(music_path);
            playing_music = AudioChannel::new(music_path.into());
            audio.set_volume_in_channel(volume, &playing_music);
            audio.play_looped_in_channel(music_handle, &playing_music);
        }
    }
    game_state.audio_manager.playing = playing_music;
}
