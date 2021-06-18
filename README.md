# Rusty Engine

Rusty Engine is a simple, 2D game engine for those who are learning Rust. Create simple prototypes using straightforward Rust code, without any advanced game engine concepts.  Rusty engine is a simplification wrapper over [Bevy], which is what I encourage folks to use directly for their more serious game engine needs.

[Questions], [bug reports], and contributions are most welcome!

## Features

See [this issue](https://github.com/CleanCut/rusty_engine/issues/9) for details about currently-supported features.

## Courses

I plan to integrate using this game engine into some of my learning courses:

- [Ultimate Rust Crash Course]
- As-yet-unnamed 3 day course on O'Reilly online.

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
sudo apt install libasound2-dev pkg-config
```

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## Asset Licenses

All assets included with this game engine have the appropriate license described and linked to in a `README.md` file in the same directory as the source files. In most cases, the license is [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)--meaning you may do whatever you wish with the asset.

One notable exception is some of the music files, which are under a different license and include specific attribution requirements that must be met in order to be used legally when distributed. Please see [this `README.md` file](./assets/audio/music) for more information.

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

If you like Rusty Engine, please sponsor me [on GitHub] or [on Patreon]. 💖

[CPAL]: https://github.com/RustAudio/cpal
[Questions]: https://github.com/CleanCut/rusty_engine/issues/new
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course
[bug reports]: https://github.com/CleanCut/rusty_engine/issues/new
[rendy]: https://github.com/amethyst/rendy
[on GitHub]: https://github.com/sponsors/CleanCut
[on Patreon]: https://patreon.com/nathanstocks
[Bevy]: https://bevyengine.org/
