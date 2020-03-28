# Rusty Engine

Rusty Engine is a fun, cross-platform game engine written in Rust. The primary goal is to be as
simple to use as possible so game engine concepts don't get in the way of learning Rust.

[Questions], [bug reports], and contributions are most welcome.

If you find this project useful, please consider [sponsoring] me!

## Features

- 4-channel audio system supporting MP3, WAV, Vorbis and Flac.
- Keyboard, mouse, and window input events.
- OpenGL backend via glium.

## Aspirational Plans

- Companion course ([Ultimate Rust Crash Course]) with project walkthrough videos (the course
  exists, it just doesn't have project walkthrough videos yet)
- Switch to Vulkan/Metal via [rendy] before OpenGL stops working on macOS.
- A user guide

### Audio Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, iOS, and emscripten.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev
```

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

[CPAL]: https://github.com/RustAudio/cpal
[Questions]: https://github.com/CleanCut/rusty_engine/issues/new
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course
[bug reports]: https://github.com/CleanCut/rusty_engine/issues/new
[rendy]: https://github.com/amethyst/rendy
[sponsoring]: https://github.com/sponsors/CleanCut