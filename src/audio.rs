//! Facilities for interacting with audio, including: [`AudioManager`], [`MusicPreset`], and
//! [`SfxPreset`]
//!
//! You may add your own sound files to the `assets/audio` directory or any of its subdirectories
//! and play them as sound effects or music by providing the relative path to the file. For example,
//!  if you place a file named `my_sound_effect.mp3` in this directory, you could play it with:
//!
//! ```rust
//! # use rusty_engine::prelude::*;
//! #
//! # rusty_engine::init!();
//! #
//! # fn main() {
//! # let mut engine_state = Game::new();
//! // Inside your logic function...
//! engine_state.audio_manager.play_sfx("my_sound_effect.mp3", 1.0);
//! # }
//! ```
//!
//! Or, if you create a `my_game/` subdirectory and place a file named `spooky_loop.ogg`, you could play it as continuous music with:
//!
//! ```rust
//! # use rusty_engine::prelude::*;
//! #
//! # rusty_engine::init!();
//! #
//! # fn main() {
//! # let mut engine_state = Game::new();
//! // Inside your logic function...
//! engine_state.audio_manager.play_music("my_game/spooky_loop.ogg", 1.0);
//! # }
//! ```
//!
//! The sound effects provided in this asset pack have convenient `enum`s defined that you can use instead of a path to the file: `SfxPreset` and `MusicPreset`. For example:
//!
//! ```rust
//! // Import the enums into scope first
//! use rusty_engine::prelude::*;
//!
//! # rusty_engine::init!();
//! #
//! # fn main() {
//! # let mut engine_state = Game::new();
//! // Inside your logic function...
//! engine_state.audio_manager.play_sfx(SfxPreset::Confirmation1, 1.0);
//! engine_state.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);
//! # }
//! ```
//!

use crate::prelude::EngineState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use std::array::IntoIter;

#[derive(Default)]
#[doc(hidden)]
/// Use a Bevy plugin to run a Bevy system to handle our audio logic
pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(queue_managed_audio_system);
    }
}

/// You will interact with a [`AudioManager`] for all audio needs in Rusty Engine. It is exposed
/// through the [`EngineState`](crate::prelude::EngineState) struct provided to your logic function
/// each frame as the [`audio_manager`](crate::prelude::EngineState::audio_manager) field.
#[derive(Debug, Default)]
pub struct AudioManager {
    sfx_queue: Vec<(String, f32)>,
    music_queue: Vec<Option<(String, f32)>>,
    playing: AudioChannel,
    music_playing: bool,
}

impl AudioManager {
    /// Play a sound effect. `volume` ranges from `0.0` to `1.0`. `sfx` can be an [`SfxPreset`] or a
    /// string containing the relative path/filename of a sound file within the `assets/audio`
    /// directory. Sound effects are "fire and forget". They will play to completion and then stop.
    /// Multiple sound effects will be mixed and play simultaneously.
    pub fn play_sfx<S: Into<String>>(&mut self, sfx: S, volume: f32) {
        self.sfx_queue.push((sfx.into(), volume.clamp(0.0, 1.0)));
    }
    /// Play looping music. `volume` ranges from `0.0` to `1.0`. Music will loop until stopped with
    /// [`stop_music`](AudioManager::stop_music). Playing music stops any previously playing music.
    /// `music` can be a [`MusicPreset`] or a string containing the relative path/filename of a
    /// sound file within the `assets/audio` directory.
    pub fn play_music<S: Into<String>>(&mut self, music: S, volume: f32) {
        self.music_playing = true;
        self.music_queue
            .push(Some((music.into(), volume.clamp(0.0, 1.0))));
    }
    /// Stop any music currently playing. Ignored if no music is currently playing.
    pub fn stop_music(&mut self) {
        if self.music_playing {
            self.music_playing = false;
            self.music_queue.push(None);
        }
    }
    /// Whether music is currently playing.
    pub fn music_playing(&self) -> bool {
        self.music_playing
    }
}

#[derive(Copy, Clone, Debug)]
/// Sound effects included with the downloadable asset pack. You can hear these all played in the
/// `sfx` example by cloning the `rusty_engine` repository and running the following command:
///
/// ```text
/// cargo run --release --example sfx
/// ```
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

impl From<SfxPreset> for String {
    fn from(sfx_preset: SfxPreset) -> Self {
        match sfx_preset {
            SfxPreset::Click => "sfx/click.ogg".into(),
            SfxPreset::Confirmation1 => "sfx/confirmation1.ogg".into(),
            SfxPreset::Confirmation2 => "sfx/confirmation2.ogg".into(),
            SfxPreset::Congratulations => "sfx/congratulations.ogg".into(),
            SfxPreset::Forcefield1 => "sfx/forcefield1.ogg".into(),
            SfxPreset::Forcefield2 => "sfx/forcefield2.ogg".into(),
            SfxPreset::Impact1 => "sfx/impact1.ogg".into(),
            SfxPreset::Impact2 => "sfx/impact2.ogg".into(),
            SfxPreset::Impact3 => "sfx/impact3.ogg".into(),
            SfxPreset::Jingle1 => "sfx/jingle1.ogg".into(),
            SfxPreset::Jingle2 => "sfx/jingle2.ogg".into(),
            SfxPreset::Jingle3 => "sfx/jingle3.ogg".into(),
            SfxPreset::Minimize1 => "sfx/minimize1.ogg".into(),
            SfxPreset::Minimize2 => "sfx/minimize2.ogg".into(),
            SfxPreset::Switch1 => "sfx/switch1.ogg".into(),
            SfxPreset::Switch2 => "sfx/switch2.ogg".into(),
            SfxPreset::Tones1 => "sfx/tones1.ogg".into(),
            SfxPreset::Tones2 => "sfx/tones2.ogg".into(),
        }
    }
}

/// Music included with the downloadable asset pack. You can hear this music in the `music` example
/// by cloning the `rusty_engine` repository and running the following command:
///
/// ```text
/// cargo run --release --example music
/// ```
#[derive(Copy, Clone, Debug)]
pub enum MusicPreset {
    Classy8Bit,
    MysteriousMagic,
    WhimsicalPopsicle,
}

impl MusicPreset {
    pub fn variant_iter() -> IntoIter<MusicPreset, 3> {
        static MUSIC_PRESETS: [MusicPreset; 3] = [
            MusicPreset::Classy8Bit,
            MusicPreset::MysteriousMagic,
            MusicPreset::WhimsicalPopsicle,
        ];
        MUSIC_PRESETS.into_iter()
    }
}

impl From<MusicPreset> for String {
    fn from(music_preset: MusicPreset) -> String {
        match music_preset {
            MusicPreset::Classy8Bit => "music/Classy 8-Bit.oga".into(),
            MusicPreset::MysteriousMagic => "music/Mysterious Magic.oga".into(),
            MusicPreset::WhimsicalPopsicle => "music/Whimsical Popsicle.oga".into(),
        }
    }
}

// The Bevy system that checks and see if there is any audio management that needs to be done.
#[doc(hidden)]
pub fn queue_managed_audio_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_state: ResMut<EngineState>,
) {
    for (sfx, volume) in game_state.audio_manager.sfx_queue.drain(..) {
        let sfx_path = format!("audio/{}", sfx);
        let sfx_handle = asset_server.load(sfx_path.as_str());
        // To be able to set the volume of a sound effect, we need the channel it is being played
        // in. We'll start by naively creating a new channel for every single sound effect. If this
        // ends up being a performance or correctness problem, we'll need to circle back and do
        // something more sophisticated (like keep a set number of channels around at different
        // volumes).
        let new_sfx_channel = AudioChannel::new(sfx_path);
        audio.set_volume_in_channel(volume, &new_sfx_channel);
        audio.play_in_channel(sfx_handle, &new_sfx_channel);
    }
    let mut playing_music = game_state.audio_manager.playing.clone();
    for item in game_state.audio_manager.music_queue.drain(..) {
        audio.stop_channel(&playing_music);
        if let Some((music, volume)) = item {
            let music_path = format!("audio/{}", music);
            let music_handle = asset_server.load(music_path.as_str());
            playing_music = AudioChannel::new(music_path);
            audio.set_volume_in_channel(volume, &playing_music);
            audio.play_looped_in_channel(music_handle, &playing_music);
        }
    }
    game_state.audio_manager.playing = playing_music;
}
