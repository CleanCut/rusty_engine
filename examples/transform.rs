//
use rusty_engine::prelude::*;
use std::f64::consts::TAU;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Race Car", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    for actor in &mut game_state.actors.values_mut() {
        match game_state.seconds_since_startup {
            x if x % (3.0 * TAU) < TAU => {
                // reset scale and rotation
                actor.scale = 1.0;
                actor.rotation = UP;
                // play with translation
                actor.translation.x = (game_state.seconds_since_startup.cos() * 100.0) as f32;
                actor.translation.y = (game_state.seconds_since_startup.sin() * 100.0) as f32;
            }
            x if x % (3.0 * TAU) < 2.0 * TAU => {
                // reset translation and rotation
                actor.translation = Vec2::ZERO;
                actor.rotation = UP;
                // play with scale
                actor.scale = ((game_state.seconds_since_startup * 0.5).cos().abs() * 2.0) as f32;
            }
            _ => {
                // reset translation and scale
                actor.translation = Vec2::ZERO;
                actor.scale = 1.0;
                // play with rotation
                actor.rotation = game_state.seconds_since_startup as f32;
            }
        }
    }
}
