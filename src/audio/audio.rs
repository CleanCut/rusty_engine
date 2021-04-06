use super::{AudioSource, Decodable};
use bevy::asset::{Asset, Handle};
use parking_lot::RwLock;
use std::{collections::VecDeque, fmt};

/// The external struct used to play audio
pub struct Audio<P = AudioSource>
where
    P: Asset + Decodable,
{
    pub sfx_queue: RwLock<VecDeque<Handle<P>>>,
    pub music_queue: RwLock<VecDeque<Handle<P>>>,
}

impl<P: Asset> fmt::Debug for Audio<P>
where
    P: Decodable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Audio")
            .field("sfx_queue", &self.sfx_queue)
            .field("music_queue", &self.music_queue)
            .finish()
    }
}

impl<P> Default for Audio<P>
where
    P: Asset + Decodable,
{
    fn default() -> Self {
        Self {
            sfx_queue: Default::default(),
            music_queue: Default::default(),
        }
    }
}

impl<P> Audio<P>
where
    P: Asset + Decodable,
    <P as Decodable>::Decoder: rodio::Source + Send + Sync,
    <<P as Decodable>::Decoder as Iterator>::Item: rodio::Sample + Send + Sync,
{
    pub fn play_sfx(&self, audio_source: Handle<P>) {
        self.sfx_queue.write().push_front(audio_source);
    }
    pub fn play_music(&self, audio_source: Handle<P>) {
        self.music_queue.write().push_front(audio_source);
    }
}
