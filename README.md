# Rusty Engine

A fun way to learn Rust using a simple 2D game engine.

[Questions], [bug reports], and contributions are most welcome!

[Questions]: https://github.com/CleanCut/rusty_engine/issues/new
[bug reports]: https://github.com/CleanCut/rusty_engine/issues/new

## Libraries

Rusty Engine is a collection of related but independent libraries.  All of the libraries are
designed to optionally be used standalone, with the exception of `rusty_core` which is for shared
functionality and re-exports from external crates.

- `rusty_audio` - Load & play audio
- `rusty_gfx` - Create windows, display graphics, handle input events (keyboard, mouse, etc.)
- `rusty_core` - Used by the rest of the `rusty_*` crates for anything they have in common,
  including re-exports of external libraries.

### Audio Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, iOS, and emscripten.  For Linux, the
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

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).