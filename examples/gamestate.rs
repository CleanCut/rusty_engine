use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Race Car", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    // Use a timer to tell when to change state
    game.game_state_mut().timer_map.insert(
        "change_state".into(),
        Timer::from_seconds(std::f32::consts::FRAC_PI_2, true),
    );
    // Are we turning?
    game.game_state_mut()
        .bool_map
        .insert("turning".into(), false);

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    if let Some(actor) = game_state.actors.get_mut("Race Car") {
        let turning_mut_ref = game_state.bool_map.get_mut("turning").unwrap();
        // gain another life every time the timer goes off
        if game_state
            .timer_map
            .get_mut("change_state")
            .unwrap()
            .tick(game_state.delta)
            .just_finished()
        {
            *turning_mut_ref = !*turning_mut_ref;
        }
        // Rotate the player
        if *turning_mut_ref {
            actor.rotation += game_state.delta_seconds * 3.0;
        }
    }
}
