# `rusty_audio`

`rusty_audio` is a fun and easy audio library that provides a 4-track audio system to load/decode
audio files and play them. Supported formats are: MP3, WAV, Vorbis and Flac.

This library is part of the [`rusty_engine`] game engine, but can be used as a standalone library as
well. It uses the very powerful [rodio] audio playback library under the hood, which you should
consider using directly if your needs are more complex.

[`rusty_engine`]: https://github.com/cleancut/rusty_engine
[rodio]: https://github.com/tomaka/rodio

### Dependencies on Linux

`rusty_audio` should work out-of-the-box on macOS, Windows, iOS, and emscripten.  For Linux, the
downstream package for actually _playing_ sound ([CPAL](https://github.com/RustAudio/cpal) requires
the *alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev
```