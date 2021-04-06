use bevy::prelude::{AssetServer, Res, ResMut};

use crate::prelude::{Audio, GameState};

#[derive(Default)]
pub struct AudioManager {
    sfx_queue: Vec<SfxPreset>,
    music_queue: Vec<MusicPreset>,
}

impl AudioManager {
    pub fn play_sfx(&mut self, sfx_preset: SfxPreset) {
        self.sfx_queue.push(sfx_preset);
    }
    pub fn play_music(&mut self, music_preset: MusicPreset) {
        self.music_queue.push(music_preset);
    }
}

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
            SfxPreset::Click => "audio/sfx/audio/sfx/click.ogg",
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
}

pub enum MusicPreset {
    ArcadeFantasy,
    Classy8Bit,
    MysteriousMagic,
    WhimsicalPopsicle,
}

impl MusicPreset {
    fn to_path(self) -> &'static str {
        match self {
            MusicPreset::ArcadeFantasy => "audio/music/Arcade Fantasy.oga",
            MusicPreset::Classy8Bit => "audio/music/Classy 8-Bit.oga",
            MusicPreset::MysteriousMagic => "audio/music/Mysterious Magic.oga",
            MusicPreset::WhimsicalPopsicle => "audio/music/Whimsical Popsicle.oga",
        }
    }
}

pub fn queue_managed_audio_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_state: ResMut<GameState>,
) {
    for sfx in game_state.audio_manager.sfx_queue.drain(..) {
        let sfx_handle = asset_server.load(sfx.to_path());
        audio.play_sfx(sfx_handle);
    }
    for music in game_state.audio_manager.music_queue.drain(..) {
        let music_handle = asset_server.load(music.to_path());
        audio.play_music(music_handle);
    }
}
