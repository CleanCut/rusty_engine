use std::f32::consts::TAU;

use rusty_engine::prelude::*;

// There are LOTS of fields on the GameState struct that you can use. For more info, please see:
// https://docs.rs/rusty_engine/latest/rusty_engine/game/struct.GameState.html

fn main() {
    let mut game = Game::new();
    let _ = game.add_actor("Race Car", ActorPreset::RacingCarGreen);

    // Use a timer to tell when to change state
    game.game_state_mut()
        .timer_map
        .insert("change_state".into(), Timer::from_seconds(1.0, true));
    // Are we turning?
    game.game_state_mut()
        .bool_map
        .insert("turning".into(), false);

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    // Get mutable references to the variables in the game state that we care about
    let race_car = game_state.actors.get_mut("Race Car").unwrap();
    let turning = game_state.bool_map.get_mut("turning").unwrap();
    let timer = game_state.timer_map.get_mut("change_state").unwrap();

    // If we aren't turning, then tick the timer until it's time to start turning again
    if !*turning && timer.tick(game_state.delta).just_finished() {
        *turning = true;
    }

    // Rotate the player
    if *turning {
        race_car.rotation += game_state.delta_f32 * 3.0;
        // If the player rotated all the way around, reset direction, stop turning
        // TAU == (2 * PI), which is exactly one rotation in radians
        if race_car.rotation > TAU {
            race_car.rotation = 0.0;
            *turning = false;
        }
    }
}
