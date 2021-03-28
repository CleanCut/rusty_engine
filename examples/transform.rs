//
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.add_actor("Race Car".to_string(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(0.0, 0.0))
        .set_rotation(UP)
        .set_scale(1.0);

    game.add_logic(logic);

    game.run();
}

fn logic(_game_state: &mut GameState, actor: &mut Actor, time: &Time) {
    use std::f64::consts::TAU;
    match time.seconds_since_startup() {
        x if x % (3.0 * TAU) < TAU => {
            // reset scale and rotation
            actor.scale = 1.0;
            actor.rotation = UP;
            // play with translation
            actor.translation.x = (time.seconds_since_startup().cos() * 100.0) as f32;
            actor.translation.y = (time.seconds_since_startup().sin() * 100.0) as f32;
        }
        x if x % (3.0 * TAU) < 2.0 * TAU => {
            // reset translation and rotation
            actor.translation = Vec2::ZERO;
            actor.rotation = UP;
            // play with scale
            actor.scale = ((time.seconds_since_startup() * 0.5).cos().abs() * 2.0) as f32;
        }
        _ => {
            // reset translation and scale
            actor.translation = Vec2::ZERO;
            actor.scale = 1.0;
            // play with rotation
            actor.rotation = time.seconds_since_startup() as f32;
        }
    }
}
