use rusty_engine::prelude::*;

const ANCHOR_SPOT: (f32, f32) = (0.0, -200.0);

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Race Car", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;
    race_car.layer = 2.0;

    let mut anchor = game.add_actor("anchor", ActorPreset::RollingHoleEnd);
    anchor.translation = ANCHOR_SPOT.into();
    anchor.layer = 0.0;

    let mut mover = game.add_actor("mover", ActorPreset::RollingHoleStart);
    mover.translation = ANCHOR_SPOT.into();
    mover.layer = 1.0;

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
        for cursor_moved in &game_state.mouse_location_events {
            actor.translation = cursor_moved.position;
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
            actor.translation = actor.translation.lerp(Vec2::from(ANCHOR_SPOT), 0.05);
        }
    }
}
