use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Race Car".into(), ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    if let Some(actor) = game_state.actors.get_mut("Race Car") {
        for keyboard_input in &game_state.keyboard_events {
            if let KeyboardInput {
                scan_code: _,
                key_code: Some(key_code),
                state: ElementState::Pressed,
            } = keyboard_input
            {
                match key_code {
                    KeyCode::A | KeyCode::Left => actor.translation.x -= 10.0,
                    KeyCode::D | KeyCode::Right | KeyCode::E => actor.translation.x += 10.0,
                    KeyCode::O | KeyCode::Down | KeyCode::S => actor.translation.y -= 10.0,
                    KeyCode::W | KeyCode::Up | KeyCode::Comma => actor.translation.y += 10.0,
                    KeyCode::Z | KeyCode::Apostrophe => {
                        actor.rotation += std::f32::consts::FRAC_PI_4
                    }
                    KeyCode::C | KeyCode::Period => actor.rotation -= std::f32::consts::FRAC_PI_4,
                    KeyCode::Plus | KeyCode::Equals => actor.scale *= 1.1,
                    KeyCode::Minus | KeyCode::Underline => actor.scale *= 0.9,
                    _ => {}
                }
                actor.scale = actor.scale.clamp(0.1, 3.0);
                actor.translation = actor.translation.clamp(
                    -game_state.screen_dimensions * 0.5,
                    game_state.screen_dimensions * 0.5,
                );
            }
        }
    }
}
