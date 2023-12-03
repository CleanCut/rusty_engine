# Quick Start Example

- Create a new Rust project and run `cargo add rusty_engine` to add Rusty Engine as a dependency (see the [Configuration](05-config.md) page for more details). Your `Cargo.toml` file should end up with a line similar to this:
```toml
# In your [dependencies] section of Cargo.toml
rusty_engine = "6.0.0"
```
- Download the Asset Pack to your project (see the [Asset Pack](10-assets.md) page for more details).
```shell
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```
- Write your game:

```rust,ignore
// in src/main.rs
use rusty_engine::prelude::*;
// Define a struct to hold custom data for your game. If you don't yet know what data fields you
// need, it can be an empty struct. It must have `#[derive(Resource)]` on the line before it.
#[derive(Resource)]
struct GameState {
    health: i32, // add any fields you want, or leave the struct without fields
}
fn main() {
    // Create a game
    let mut game = Game::new();
    // Set up your game. `Game` exposes all of the methods and fields of `Engine`
    let sprite = game.add_sprite("player", SpritePreset::RacingCarBlue);
    sprite.scale = 2.0;
    // Start some music!
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);
    // Add one or more functions with logic for your game. When the game is run, the logic
    // functions will run in the order they were added.
    game.add_logic(game_logic);
    // Run the game, with an initial state
    game.run(GameState { health: 100 });
}
// Your game logic functions can be named anything, but the first parameter is always a
// `&mut Engine`, and the second parameter is a mutable reference to your custom game
// state struct (`&mut GameState` in this case). The function returns a `bool`.
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

- Run your game with `cargo run --release`.  Don't forget the `--release`!

<img width="1348" alt="example screenshot" src="https://user-images.githubusercontent.com/5838512/146858022-1d91c7f4-8b21-4f85-a72a-c4b93edcabc6.png">
