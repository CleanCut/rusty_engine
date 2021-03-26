# Rusty Engine

Rusty Engine is a simple, 2D game engine for those who are learning Rust. Create simple
prototypes using straightforward Rust code, without any advanced game engine concepts.

[Questions], [bug reports], and contributions are most welcome!

If you would like to support this project consider [sponsoring me] on GitHub. 💖

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

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

If you like Rusty Engine, please consider [sponsoring me] on GitHub. 💖

[CPAL]: https://github.com/RustAudio/cpal
[Questions]: https://github.com/CleanCut/rusty_engine/issues/new
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course
[bug reports]: https://github.com/CleanCut/rusty_engine/issues/new
[rendy]: https://github.com/amethyst/rendy
[sponsoring me]: https://github.com/sponsors/CleanCut
