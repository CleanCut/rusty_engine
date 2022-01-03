# Quick Start Example

- Create a new Rust project and add `rusty_engine` to the `[dependencies]` section of your `Cargo.toml` file.
- Download the [Asset Pack](10-assets.md) to your project.
- Write your game:

```rust,ignore
// in src/main.rs
 use rusty_engine::prelude::*;

 // Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
 struct GameState {
     health: i32,
 }

 // Initialize the engine with your custom struct
 rusty_engine::init!(GameState);

 fn main() {
     // Create a game
     let mut game = Game::new();
     // Set up your game. `Game` exposes all of the methods (but not fields) of `EngineState` as well.
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
 // `&mut EngineState`, and the second parameter is a mutable reference to your custom game
 // state struct (`&mut GameState` in this case). The function returns a `bool`.
 //
 // This function will be run once each frame.
 fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
     // The `EngineState` contains all sorts of built-in goodies.
     // Get access to the player sprite...
     let player = engine_state.sprites.get_mut("player").unwrap();
     // Rotate the player...
     player.rotation += std::f32::consts::PI * engine_state.delta_f32;
     // Damage the player if it is out of bounds...
     if player.translation.x > 100.0 {
         game_state.health -= 1;
     }
     // Returning `true` means the next logic function in line should be run.
     true
 }
 ```

- Run your game with `cargo run --release`.  Don't forget the `--release`!

<img width="1348" alt="example screenshot" src="https://user-images.githubusercontent.com/5838512/146858022-1d91c7f4-8b21-4f85-a72a-c4b93edcabc6.png">
