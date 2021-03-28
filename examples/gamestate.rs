use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.add_actor("Race Car".to_string(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(0.0, 0.0))
        .set_rotation(UP)
        .set_scale(1.0);

    // Use a timer to tell when to change state
    game.game_state_mut().timers[0] = Timer::from_seconds(std::f32::consts::FRAC_PI_2, true);
    // Are we turning?
    game.game_state_mut().bools[0] = false;

    game.add_logic(logic);

    game.run();
}

fn logic(game_state: &mut GameState, actor: &mut Actor, time: &Time) {
    if actor.name == "Race Car" {
        // gain another life every time the timer goes off
        if game_state.timers[0].tick(time.delta()).just_finished() {
            game_state.bools[0] = !game_state.bools[0];
        }
        // Rotate the player, for fun
        if game_state.bools[0] {
            actor.rotation += time.delta_seconds() * 3.0;
        }
    }
}
