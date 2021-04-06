use bevy::prelude::IntoSystem;

mod audio;
mod audio_manager;
mod audio_output;
mod audio_source;

pub mod prelude {
    pub use super::{
        Audio, AudioManager, AudioOutput, AudioSource, Decodable, MusicPreset, SfxPreset,
    };
}

pub use audio::*;
pub use audio_manager::*;
pub use audio_output::*;
pub use audio_source::*;

use bevy::app::prelude::*;
use bevy::asset::AddAsset;
use bevy::ecs::system::IntoExclusiveSystem;

/// Adds support for audio playback to an App
#[derive(Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_non_send_resource::<AudioOutput<AudioSource>>()
            .add_asset::<AudioSource>()
            .init_asset_loader::<AudioLoader>()
            .init_resource::<Audio<AudioSource>>()
            .add_system(queue_managed_audio_system.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                play_queued_audio_system::<AudioSource>.exclusive_system(),
            );
    }
}
