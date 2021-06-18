use rusty_engine::prelude::*;

const ANCHOR_SPOT: (f32, f32) = (0.0, -200.0);

fn main() {
    let mut game = Game::new();

    game.add_actor("Race Car".into(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(0.0, 0.0))
        .set_rotation(UP)
        .set_scale(1.0);

    game.add_logic(logic);
    game.run();
}

fn logic(game_state: &mut GameState, actor: &mut Actor, _time: &Time) {
    if actor.label == "Race Car" {
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
