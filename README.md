# Rusty Engine

Rusty Engine is a simple, 2D game engine for those who are learning Rust. Create simple game prototypes using straightforward Rust code without needing to learning difficult game engine concepts! It works on macOS, Linux, and Windows. Rusty Engine is a simplification wrapper over [Bevy], which I encourage you to use directly for more serious game engine needs.

[Questions], [bug reports], and contributions are most welcome!

https://user-images.githubusercontent.com/5838512/122880590-651bae00-d2f7-11eb-8e5c-4810b3777828.mp4

## Documentation

- [Tutorial](https://cleancut.github.io/rusty_engine/)
- [API Reference](https://docs.rs/rusty_engine/latest/rusty_engine/)
- [Code Examples](https://github.com/CleanCut/rusty_engine/tree/main/examples)
- [Game Scenarios](https://github.com/CleanCut/rusty_engine/tree/main/scenarios)

## Features

- Asset pack included (sprites, music, sound effects, and fonts)
- Sprites (2D images)
  - Use sprites from the included asset pack, or bring your own
  - Collision detection with custom colliders
- Audio (music & sound effects)
  - Looping music
  - Multi-channel sound effects
- Text
  - 2 fonts included, or bring your own
- Input handling (keyboard, mouse)
- Timers
- Custom game state
- Window customization

## Courses

If you like Rusty Engine, please sponsor me [on GitHub] or [on Patreon], or take one of my courses below!

The following courses use Rusty Engine in their curriculum:

- [Ultimate Rust 2: Intermediate Concepts](https://www.udemy.com/course/ultimate-rust-2/?referralCode=8ED694EBE5637F954414) on Udemy (the sequel to [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/?referralCode=AF30FAD8C6CCCC2C94F0))
- [Rust in 3 Weeks](https://agileperception.com) conducted live on O'Reilly Online approximately once each quarter.

## Linux Dependencies (Including WSL 2)

If you are using Linux or Windows Subsystem for Linux 2, please visit Bevy's [Installing Linux Dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) page and follow the instructions to install needed dependencies.

## Quick Start

### You MUST download the assets separately!!!

Here are three different ways to download the assets (pick any of them--it should end up the same in the end):
- Clone the `rusty_engine` repository and copy/move the `assets/` directory over to your own project
- Download a [zip file](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.zip) or [tarball](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz) of the `rusty_engine` repository, extract it, and copy/move the `assets/` directory over to your own project.
- (My favorite!) On a posix compatible shell, run this command inside your project directory:
```shell
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```

Add `rusty_engine` as a dependency

```toml
# In your [dependencies] section of Cargo.toml
rusty_engine = "5.1.1"
```

Write your game!

```rust
// in src/main.rs
 use rusty_engine::prelude::*;

 // Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
 struct GameState {
     health: i32,
 }

 fn main() {
     // Create a game
     let mut game = Game::new();
     // Set up your game. `Game` exposes all of the methods and fields of `Engine`.
     let sprite = game.add_sprite("player", SpritePreset::RacingCarBlue);
     sprite.scale = 2.0;
     game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);
     // Add one or more functions with logic for your game. When the game is run, the logic
     // functions will run in the order they were added.
     game.add_logic(game_logic);
     // Run the game, with an initial state
     let initial_game_state = GameState { health: 100 };
     game.run(initial_game_state);
 }

 // Your game logic functions can be named anything, but the first parameter is always a
 // `&mut Engine`, and the second parameter is a mutable reference to your custom game
 // state struct (`&mut GameState` in this case).
 //
 // This function will be run once each frame.
 fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
     // The `Engine` contains all sorts of built-in goodies.
     // Get access to the player sprite...
     let player = engine.sprites.get_mut("player").unwrap();
     // Rotate the player...
     player.rotation += std::f32::consts::PI * engine.delta_f32;
     // Damage the player if it is out of bounds...
     if player.translation.x > 100.0 {
         game_state.health -= 1;
     }
 }
```

Run your game with `cargo run --release`!

<img width="1348" alt="example screenshot" src="https://user-images.githubusercontent.com/5838512/146858022-1d91c7f4-8b21-4f85-a72a-c4b93edcabc6.png">


See also the [tutorial](https://cleancut.github.io/rusty_engine/), [game scenarios](https://github.com/CleanCut/rusty_engine/tree/main/scenarios), [code examples](https://github.com/CleanCut/rusty_engine/tree/main/examples) and the [API documentation](https://docs.rs/rusty_engine/latest/rusty_engine/)

## Student Showcase

Show off the project you made with Rusty Engine! Learning Rust can be fun. 😄 Just [send me a link](https://agileperception.com/contact) and I'll add it to the list!

- [Rusty Asteroids](https://github.com/etnt/rusty-asteroids) by [@etnt](https://github.com/etnt)
- [Rusty Breakout](https://github.com/etnt/rusty-breakout) by [@etnt](https://github.com/etnt)

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
