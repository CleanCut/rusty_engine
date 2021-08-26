If you like Rusty Engine, please sponsor me [on GitHub] or [on Patreon], or [take one of my courses](https://agileperception.com). 💖

# Rusty Engine

Rusty Engine is a simple, 2D game engine for those who are learning Rust. Create simple game prototypes using straightforward Rust code without any advanced game engine concepts! It works on macOS, Linux, and Windows. Rusty Engine is a simplification wrapper over [Bevy], which I encourage you to use directly for more serious game engine needs.

[Questions], [bug reports], and contributions are most welcome!

https://user-images.githubusercontent.com/5838512/122880590-651bae00-d2f7-11eb-8e5c-4810b3777828.mp4

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

## Linux Dependencies (Including WSL 2)

If you are using Linux or Windows Subsystem for Linux 2, please visit Bevy's [Installing Linux Dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) page and follow the instructions to install needed dependencies.

## Quick Start

### You MUST download the assets separately!!!
Here are three different ways to download the assets (it should end up the same in the end):
- Clone the `rusty_engine` repository and copy/move the `assets/` directory over to your own project
- Download a [zip file](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.zip) or [tarball](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz) of the `rusty_engine` repository, extract it, and copy/move the `assets/` directory over to your own project.
- On a posix compatible shell, run this command inside your project directory:
```shell
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```

Add `rusty_engine` as a dependency

```toml
# In your [dependencies] section of Cargo.toml
rusty_engine = "1.1.2"
```

Write your game!

```rust
// In main.rs
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();
    // Use `game` to initialize starting state.
    let race_car: &mut Actor = game.add_actor("race car", ActorPreset::RacingCarYellow);
    race_car.translation = Vec2::new(-100.0, -100.0);
    race_car.rotation = NORTH_EAST;
    race_car.scale = 2.0;
    // Then do `game.run()` to start the game.
    game.run(game_logic);
}

// This function is called once per frame
fn game_logic(game_state: &mut GameState) {
    // Your game logic goes here
}

```

Run your game with `cargo run --release`!

<img width="1392" alt="Screen Shot 2021-06-22 at 1 10 04 AM" src="https://user-images.githubusercontent.com/5838512/122879972-b5ded700-d2f6-11eb-9066-99d5b56fcd3a.png">


See also the [game scenarios](https://github.com/CleanCut/rusty_engine/tree/main/scenarios), [code examples](https://github.com/CleanCut/rusty_engine/tree/main/examples) and the [API documentation](https://docs.rs/rusty_engine/latest/rusty_engine/)

## Contribution

All software contributions are assumed to be dual-licensed under MIT/Apache-2.  All asset contributions must be under licenses compatible with the software license, and explain their license(s) in a `README.md` file in the same directory as the source files.

## Asset Licenses

All assets included with this game engine have the appropriate license described and linked to in a `README.md` file in the same directory as the source files. In most cases, the license is [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)--meaning you may do whatever you wish with the asset.

One notable exception is some of the music files, which are under a different license and include specific attribution requirements that must be met in order to be used legally when distributed. Please see [this `README.md` file](./assets/audio/music) for more information.

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

[CPAL]: https://github.com/RustAudio/cpal
[Questions]: https://github.com/CleanCut/rusty_engine/discussions
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course
[bug reports]: https://github.com/CleanCut/rusty_engine/issues/new
[rendy]: https://github.com/amethyst/rendy
[on GitHub]: https://github.com/sponsors/CleanCut
[on Patreon]: https://patreon.com/nathanstocks
[Bevy]: https://bevyengine.org/
