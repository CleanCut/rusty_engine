use rusty_engine::prelude::*;

const ANCHOR_SPOT: (f32, f32) = (0.0, -200.0);

fn main() {
    let mut game = Game::new();

    game.add_actor("Race Car".into(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(0.0, 0.0))
        .set_rotation(UP)
        .set_scale(1.0)
        .set_layer(2.0);

    game.add_actor("anchor".into(), ActorPreset::RollingHoleEnd)
        .set_translation(ANCHOR_SPOT.into())
        .set_layer(0.0);

    game.add_actor("mover".into(), ActorPreset::RollingHoleStart)
        .set_translation(ANCHOR_SPOT.into())
        .set_layer(1.0);

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    if let Some(actor) = game_state.actors.get_mut("Race Car") {
        for mouse_button_input in &game_state.mouse_button_events {
            if mouse_button_input.state != ElementState::Pressed {
                break;
            }
            match mouse_button_input.button {
                MouseButton::Left => actor.rotation += std::f32::consts::FRAC_PI_4,
                MouseButton::Right => actor.rotation -= std::f32::consts::FRAC_PI_4,
                _ => {}
            }
        }
        for cursor_moved in &game_state.cursor_moved_events {
            actor.set_translation(cursor_moved.position);
        }
        for mouse_wheel in &game_state.mouse_wheel_events {
            if mouse_wheel.y > 0.0 {
                actor.scale *= 1.1;
            } else {
                actor.scale *= 0.9;
            }
            actor.scale = actor.scale.clamp(0.1, 3.0);
        }
    }

    if let Some(actor) = game_state.actors.get_mut("mover") {
        let mut moved = false;
        for mouse_motion in &game_state.mouse_motion_events {
            actor.translation.x = ANCHOR_SPOT.0 + mouse_motion.delta.x;
            actor.translation.y = ANCHOR_SPOT.1 - mouse_motion.delta.y;
            moved = true;
        }
        if !moved {
            actor.set_translation(actor.translation.lerp(Vec2::from(ANCHOR_SPOT), 0.05));
        }
    }
}
