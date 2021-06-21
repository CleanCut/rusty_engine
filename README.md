# Rusty Engine

Rusty Engine is a simple, 2D game engine for those who are learning Rust. Create simple game prototypes using straightforward Rust code without any advanced game engine concepts! It works on macOS, Linux, and Windows. Rusty Engine is a simplification wrapper over [Bevy], which I encourage you to use directly for more serious game engine needs.

[Questions], [bug reports], and contributions are most welcome!

## Features

- Sprites (2D images)
  - Includes 2 built-in asset packs
- Audio (music & sound effects)
  - Includes 2 built-in asset packs
- Collision detection
- Text
  - Includes 2 built-in fonts
- Input handling (keyboard, mouse)
- Timers

## Courses

I teach courses which use this game engine:

- `Ultimate Rust 2: Intermediate Concepts` on Udemy, etc. Coming soon!
- [Rust in 3 Weeks](https://agileperception.com) conducted live on O'Reilly Online.

### Linux Dependencies (Including WSL 2)

If you are using Linux, or Windows Subsystem for Linux 2, please visit Bevy's [Installing Linux Dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) page and follow the instructions to install needed dependencies.

## Contribution

All software contributions are assumed to be dual-licensed under MIT/Apache-2.  All asset contributions must be under licenses compatible with the software license, and explain their license(s) in a `README.md` file in the same directory as the source files.

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
