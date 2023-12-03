//! Rusty Engine is a simple, 2D game engine for those who are learning Rust. Create simple game
//! prototypes using straightforward Rust code without any advanced game engine concepts! It works
//! on macOS, Linux, and Windows. Rusty Engine is a simplification wrapper over
//! [Bevy](https://bevyengine.org/), which I encourage you to use directly for more serious game
//! engine needs.
//!
//! # Quick Start Example
//!
//! You start by importing `rusty_engine::prelude::*`, define your own `GameState` struct,
//! create a [`Game`](crate::game::Game), set up your game, and then call
//! [`Game::run`](crate::game::Game::run).
//!
//! ```no_run
//! use rusty_engine::prelude::*;
//!
//! // Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
//! #[derive(Resource)]
//! struct GameState {
//!     health: i32,
//! }
//!
//! fn main() {
//!     // Create a game
//!     let mut game = Game::new();
//!     // Set up your game. `Game` exposes all of the methods and fields of `Engine`.
//!     let sprite = game.add_sprite("player", SpritePreset::RacingCarBlue);
//!     sprite.scale = 2.0;
//!     game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);
//!     // Add one or more functions with logic for your game. When the game is run, the logic
//!     // functions will run in the order they were added.
//!     game.add_logic(game_logic);
//!     // Run the game, with an initial state
//!     let initial_game_state = GameState { health: 100 };
//!     game.run(initial_game_state);
//! }
//!
//! // Your game logic functions can be named anything, but the first parameter is always a
//! // `&mut Engine`, and the second parameter is a mutable reference to your custom game
//! // state struct (`&mut GameState` in this case).
//! //
//! // This function will be run once each frame.
//! fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
//!     // The `Engine` contains all sorts of built-in goodies.
//!     // Get access to the player sprite...
//!     let player = engine.sprites.get_mut("player").unwrap();
//!     // Rotate the player...
//!     player.rotation += std::f32::consts::PI * engine.delta_f32;
//!     // Damage the player if it is out of bounds...
//!     if player.translation.x > 100.0 {
//!         game_state.health -= 1;
//!     }
//! }
//! ```
//!
//! # Asset Licenses
//!
//! All assets included with this game engine have the appropriate license described and linked to
//! in a `README.md` file in the same directory as the source files. In most cases, the license is
//! [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)--meaning you may do
//! whatever you wish with the asset.
//!
//! One notable exception is some of the music files, which are under a different license and
//! include specific attribution requirements that must be met in order to be used legally when
//! distributed. Please see
//! [this `README.md` file](https://github.com/CleanCut/rusty_engine/tree/main/assets/audio/music)
//! for more information.
//!
pub mod audio;
pub mod game;
pub mod keyboard;
pub mod mouse;
pub mod physics;
pub mod sprite;
pub mod text;

// Public prelude
pub mod prelude {
    pub use crate::{audio::*, game::*, keyboard::*, mouse::*, physics::*, sprite::*, text::*};
    pub use crate::{
        DOWN, EAST, LEFT, NORTH, NORTH_EAST, NORTH_WEST, RIGHT, SOUTH, SOUTH_EAST, SOUTH_WEST, UP,
        WEST,
    };
    pub use bevy::ecs as bevy_ecs;
    pub use bevy::{
        self,
        prelude::{Resource, Time, Timer, TimerMode, Vec2},
    };
}

// Used in our public constants
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// Screen directions
/// The rotation (in radians) for a sprite to face right
pub const RIGHT: f32 = 0.0;
/// The rotation (in radians) for a sprite to face left
pub const LEFT: f32 = PI;
/// The rotation (in radians) for a sprite to face up
pub const UP: f32 = FRAC_PI_2;
/// The rotation (in radians) for a sprite to face down
pub const DOWN: f32 = PI + FRAC_PI_2;

// Compass directions
/// The rotation (in radians) for a sprite to face north
pub const NORTH: f32 = FRAC_PI_2;
/// The rotation (in radians) for a sprite to face north east
pub const NORTH_EAST: f32 = FRAC_PI_4;
/// The rotation (in radians) for a sprite to face east
pub const EAST: f32 = 0.0;
/// The rotation (in radians) for a sprite to face south east
pub const SOUTH_EAST: f32 = PI + FRAC_PI_2 + FRAC_PI_4;
/// The rotation (in radians) for a sprite to face south
pub const SOUTH: f32 = PI + FRAC_PI_2;
/// The rotation (in radians) for a sprite to face south west
pub const SOUTH_WEST: f32 = PI + FRAC_PI_4;
/// The rotation (in radians) for a sprite to face west
pub const WEST: f32 = PI;
/// The rotation (in radians) for a sprite to face north west
pub const NORTH_WEST: f32 = FRAC_PI_2 + FRAC_PI_4;
