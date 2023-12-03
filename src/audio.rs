//! Facilities for interacting with audio, including: [`AudioManager`], [`MusicPreset`], and
//! [`SfxPreset`]
//!
//! You may add your own sound files to the `assets/` directory or any of its subdirectories
//! and play them as sound effects or music by providing the relative path to the file. For example,
//!  if you place a file named `my_sound_effect.mp3` in `assets/`, you can play it with:
//!
//! ```rust,no_run
//! # use rusty_engine::prelude::*;
//! #
//! # fn main() {
//! # let mut game = Game::new();
//! // Inside your logic function...
//! game.audio_manager.play_sfx("my_sound_effect.mp3", 1.0);
//! # game.run(());
//! # }
//! ```
//!
//! Or, if you create a `assets/my_game/` subdirectory and place a file named `spooky_loop.ogg`, you
//! could play it as continuous music with:
//!
//! ```rust,no_run
//! # use rusty_engine::prelude::*;
//! #
//! # fn main() {
//! # let mut game = Game::new();
//! // Inside your logic function...
//! game.audio_manager.play_music("my_game/spooky_loop.ogg", 1.0);
//! # game.run(());
//! # }
//! ```
//!
//! The sound effects provided in this asset pack have convenient `enum`s defined that you can use
//! instead of a path to the file: `SfxPreset` and `MusicPreset`. For example:
//!
//! ```rust,no_run
//! // Import the enums into scope first
//! use rusty_engine::prelude::*;
//!
//! # fn main() {
//! # let mut game = Game::new();
//! // Inside your logic function...
//! game.audio_manager.play_sfx(SfxPreset::Confirmation1, 1.0);
//! game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);
//! # game.run(());
//! # }
//! ```
//!

use crate::prelude::Engine;
use bevy::{
    audio::{AudioSink, PlaybackMode, Volume},
    prelude::*,
};
use std::{array::IntoIter, fmt::Debug};

#[derive(Default)]
#[doc(hidden)]
/// Use a Bevy plugin to run a Bevy system to handle our audio logic
pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, queue_managed_audio_system);
    }
}

/// You will interact with the [`AudioManager`] for all audio needs in Rusty Engine. It is exposed
/// through the [`Engine`](crate::prelude::Engine) struct provided to your logic function
/// each frame as the [`audio_manager`](crate::prelude::Engine::audio_manager) field. It is also
/// accessible through the [`Game`](crate::prelude::Game) struct in your `main` function.
#[derive(Default)]
pub struct AudioManager {
    sfx_queue: Vec<(String, f32)>,
    music_queue: Vec<Option<(String, f32)>>,
    playing: Option<Entity>,
    music_playing: bool,
}

impl Debug for AudioManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioManager")
            .field("sfx_queue", &self.sfx_queue)
            .field("music_queue", &self.music_queue)
            .field("music_playing", &self.music_playing)
            .finish()
    }
}

impl AudioManager {
    /// Play a sound effect. `volume` ranges from `0.0` to `1.0`. `sfx` can be an [`SfxPreset`] or a
    /// string containing the relative path/filename of a sound file within the `assets/`
    /// directory. Sound effects are "fire and forget". They will play to completion and then stop.
    /// Multiple sound effects will be mixed and play simultaneously.
    pub fn play_sfx<S: Into<String>>(&mut self, sfx: S, volume: f32) {
        self.sfx_queue.push((sfx.into(), volume.clamp(0.0, 1.0)));
    }
    /// Play looping music. `volume` ranges from `0.0` to `1.0`. Music will loop until stopped with
    /// [`stop_music`](AudioManager::stop_music). Playing music stops any previously playing music.
    /// `music` can be a [`MusicPreset`] or a string containing the relative path/filename of a
    /// sound file within the `assets/` directory.
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
/// cargo run --release --example sfx_sampler
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
/// cargo run --release --example music_sampler
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
            MusicPreset::Classy8Bit => "music/Classy 8-Bit.ogg".into(),
            MusicPreset::MysteriousMagic => "music/Mysterious Magic.ogg".into(),
            MusicPreset::WhimsicalPopsicle => "music/Whimsical Popsicle.ogg".into(),
        }
    }
}

#[derive(Component)]
pub struct Music;

/// The Bevy system that checks to see if there is any audio management that needs to be done.
#[doc(hidden)]
pub fn queue_managed_audio_system(
    mut commands: Commands,
    music_query: Query<(Entity, &AudioSink), With<Music>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<Engine>,
) {
    for (sfx, volume) in game_state.audio_manager.sfx_queue.drain(..) {
        commands.spawn(AudioBundle {
            source: asset_server.load(format!("audio/{}", sfx)),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new_relative(volume),
                ..Default::default()
            },
        });
    }
    #[allow(for_loops_over_fallibles)]
    if let Some(item) = game_state.audio_manager.music_queue.drain(..).last() {
        // stop any music currently playing
        if let Ok((entity, music)) = music_query.get_single() {
            music.stop();
            commands.entity(entity).despawn();
        }
        // start the new music...if we have some
        if let Some((music, volume)) = item {
            let entity = commands
                .spawn(AudioBundle {
                    source: asset_server.load(format!("audio/{}", music)),
                    settings: PlaybackSettings {
                        volume: Volume::new_relative(volume),
                        mode: PlaybackMode::Loop,
                        ..Default::default()
                    },
                })
                .insert(Music)
                .id();
            game_state.audio_manager.playing = Some(entity);
        }
    }
}
