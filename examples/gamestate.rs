use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.add_actor("Race Car".to_string(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(0.0, 0.0))
        .set_rotation(UP)
        .set_scale(1.0);

    // Use a timer to tell when to change state
    game.game_state_mut().timer_map.insert(
        "change_state".into(),
        Timer::from_seconds(std::f32::consts::FRAC_PI_2, true),
    );
    // Are we turning?
    game.game_state_mut()
        .bool_map
        .insert("turning".into(), false);

    game.add_logic(logic);

    game.run();
}

fn logic(game_state: &mut GameState, actor: &mut Actor, time: &Time) {
    if actor.name == "Race Car" {
        let turning_mut_ref = game_state.bool_map.get_mut("turning").unwrap();
        // gain another life every time the timer goes off
        if game_state
            .timer_map
            .get_mut("change_state")
            .unwrap()
            .tick(time.delta())
            .just_finished()
        {
            *turning_mut_ref = !*turning_mut_ref;
        }
        // Rotate the player
        if *turning_mut_ref {
            actor.rotation += time.delta_seconds() * 3.0;
        }
    }
}
