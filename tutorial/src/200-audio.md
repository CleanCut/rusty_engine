# Audio

Rusty Engine has a basic audio system. You can play one looping music track, and quite a few concurrent sound effects.  There are some music and sound effect files included in the asset pack which can be used via the `MusicPreset` and `SfxPreset` enums, respectively.

Supported audio file formats are `ogg`, `mp3`, `flac`, and `wav`.

All audio is accessed through methods on the audio manager accessed through `Engine.audio_manager`.
