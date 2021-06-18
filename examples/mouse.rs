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

    game.add_logic(logic);

    game.run();
}

fn logic(game_state: &mut GameState, actor: &mut Actor, _time: &Time) {
    if actor.label == "Race Car" {
        for mouse_button_input in game_state.mouse_events.button_events() {
            if mouse_button_input.state != ElementState::Pressed {
                break;
            }
            match mouse_button_input.button {
                MouseButton::Left => actor.rotation += std::f32::consts::FRAC_PI_4,
                MouseButton::Right => actor.rotation -= std::f32::consts::FRAC_PI_4,
                _ => {}
            }
        }
        for cursor_moved in game_state.mouse_events.cursor_moved_events() {
            actor.translation.x = cursor_moved.position.x;
            actor.translation.y = cursor_moved.position.y;
        }
    }
}
