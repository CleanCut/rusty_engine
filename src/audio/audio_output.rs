use super::{Audio, AudioSource, Decodable};
use bevy::asset::{Asset, Assets};
use bevy::ecs::world::World;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::marker::PhantomData;

/// Used internally to play audio on the current "audio device"
pub struct AudioOutput<P = AudioSource>
where
    P: Decodable,
{
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Sink,
    phantom: PhantomData<P>,
}

impl<P> Default for AudioOutput<P>
where
    P: Decodable,
{
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let music_sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

        Self {
            _stream: stream,
            stream_handle,
            music_sink,
            phantom: PhantomData,
        }
    }
}

impl<P> AudioOutput<P>
where
    P: Asset + Decodable,
    <P as Decodable>::Decoder: rodio::Source + Send + Sync,
    <<P as Decodable>::Decoder as Iterator>::Item: rodio::Sample + Send + Sync,
{
    fn play_sfx_source(&self, audio_source: &P) {
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(audio_source.decoder());
        sink.detach();
    }

    fn play_music_source(&mut self, audio_source: &P) {
        if !self.music_sink.empty() {
            self.music_sink.stop();
            self.music_sink = Sink::try_new(&self.stream_handle).unwrap();
        }
        self.music_sink
            .append(audio_source.decoder().repeat_infinite());
    }

    fn try_play_queued(&mut self, audio_sources: &Assets<P>, audio: &mut Audio<P>) {
        let mut sfx_queue = audio.sfx_queue.write();
        let len = sfx_queue.len();
        let mut i = 0;
        while i < len {
            let audio_source_handle = sfx_queue.pop_back().unwrap();
            if let Some(audio_source) = audio_sources.get(&audio_source_handle) {
                self.play_sfx_source(audio_source);
            } else {
                // audio source hasn't loaded yet. add it back to the queue
                sfx_queue.push_front(audio_source_handle);
            }
            i += 1;
        }
        let mut music_queue = audio.music_queue.write();
        let len = music_queue.len();
        let mut i = 0;
        while i < len {
            let audio_source_handle = music_queue.pop_back().unwrap();
            if let Some(audio_source) = audio_sources.get(&audio_source_handle) {
                self.play_music_source(audio_source);
            } else {
                // audio source hasn't loaded yet. add it back to the queue
                music_queue.push_front(audio_source_handle);
            }
            i += 1;
        }
    }
}

/// Plays audio currently queued in the [Audio] resource through the [AudioOutput] resource
pub fn play_queued_audio_system<P: Asset>(world: &mut World)
where
    P: Decodable,
    <P as Decodable>::Decoder: rodio::Source + Send + Sync,
    <<P as Decodable>::Decoder as Iterator>::Item: rodio::Sample + Send + Sync,
{
    let world = world.cell();
    let mut audio_output = world.get_non_send_mut::<AudioOutput<P>>().unwrap();
    let mut audio = world.get_resource_mut::<Audio<P>>().unwrap();

    if let Some(audio_sources) = world.get_resource::<Assets<P>>() {
        audio_output.try_play_queued(&*audio_sources, &mut *audio);
    };
}
