# Audio

Rusty Engine has a basic audio system. You can play one looping music track, and quite a few concurrent sound effects.  There are some music and sound effect files included in the asset pack.

Supported audio file formats are `ogg` (including `oga`), `mp3`, `flac`, and `wav`.

All audio is accessed through methods on the audio manager accessed through `EngineState.audio_manager`.
