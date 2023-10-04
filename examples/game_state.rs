//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example game_state

use std::f32::consts::TAU;

use rusty_engine::prelude::*;

// You can name your game state struct whatever you want, and it can contain many different types as
// its fields
#[derive(Resource)]
struct MyCustomGameStateStuff {
    // Use a timer to tell when to start/stop turning
    change_timer: Timer,
    // Usa a boolean track of whether we are currently turning or not
    turning: bool,
}

fn main() {
    let mut game = Game::new();
    let _ = game.add_sprite("Race Car", SpritePreset::RacingCarGreen);

    let initial_game_state = MyCustomGameStateStuff {
        change_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        turning: false,
    };

    game.add_logic(logic);
    game.run(initial_game_state);
}

fn logic(engine: &mut Engine, game_state: &mut MyCustomGameStateStuff) {
    // Get mutable references to the variables in the game state that we care about
    let race_car = engine.sprites.get_mut("Race Car").unwrap();

    // If we aren't turning, then tick the timer until it's time to start turning again
    if !game_state.turning && game_state.change_timer.tick(engine.delta).just_finished() {
        game_state.turning = true;
    }

    // Rotate the player
    if game_state.turning {
        race_car.rotation += engine.delta_f32 * 3.0;
        // If the player rotated all the way around, reset direction, stop turning
        // TAU == (2 * PI), which is exactly one rotation in radians
        if race_car.rotation > TAU {
            race_car.rotation = 0.0;
            game_state.turning = false;
        }
    }
}
