use rusty_engine::prelude::*;

rusty_engine::init!();

const ORIGIN_LOCATION: (f32, f32) = (0.0, -200.0);

fn main() {
    let mut game = Game::new();

    let race_car = game.add_actor("Race Car", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;
    race_car.layer = 2.0;

    let mover = game.add_actor("move indicator", ActorPreset::RollingHoleStart);
    mover.translation = ORIGIN_LOCATION.into();
    mover.layer = 1.0;

    let anchor = game.add_actor("move indicator origin", ActorPreset::RollingHoleEnd);
    anchor.translation = ORIGIN_LOCATION.into();
    anchor.layer = 0.0;

    let msg = game.add_text_actor("relative message", "Relative Mouse Motion Indicator");
    msg.translation.y = -300.0;
    msg.font_size = 20.0;

    let msg2 = game.add_text_actor(
        "instructions",
        "Discrete Movement with Mouse Events\n==============================\nMove the car around with your mouse.\nRotate it by clicking left/right mouse buttons.\nScale it with the mousewheel.",
    );
    msg2.font_size = 30.0;
    msg2.translation.y = 275.0;

    game.add_logic(logic);
    game.run(());
}

fn logic(engine_state: &mut EngineState, _: &mut ()) -> bool {
    if let Some(actor) = engine_state.actors.get_mut("Race Car") {
        // Use mouse button events to rotate. Every click rotates the actor by a fixed amount
        for mouse_button_input in &engine_state.mouse_button_events {
            if mouse_button_input.state != ElementState::Pressed {
                break;
            }
            match mouse_button_input.button {
                MouseButton::Left => actor.rotation += std::f32::consts::FRAC_PI_4,
                MouseButton::Right => actor.rotation -= std::f32::consts::FRAC_PI_4,
                _ => {}
            }
        }

        // Use mouse location events to set the location of the actor. This loop is effectively
        // discarding all but the last location. If that is what you want, you should use
        // GameState::mouse_state instead. See the mouse_state example for more details.
        for cursor_moved in &engine_state.mouse_location_events {
            actor.translation = cursor_moved.position;
        }

        // Use the mouse wheel events to scale the actor. Events are typically the best way to deal
        // with the mouse wheel, because then you can handle quick spins by processing each event
        // individually.
        for mouse_wheel in &engine_state.mouse_wheel_events {
            actor.scale *= 1.0 + (0.05 * mouse_wheel.y);
            actor.scale = actor.scale.clamp(0.1, 4.0);
        }
    }

    // Offset the move indicator actor from the move indicator origin to visually represent the
    // relative mouse motion for the frame
    if let Some(actor) = engine_state.actors.get_mut("move indicator") {
        // let motion = game_state.mouse_state.motion();
        // if motion != Vec2::ZERO {
        //     actor.translation = motion + ORIGIN_LOCATION.into();
        // }

        let mut cumulative_motion = Vec2::ZERO;
        for mouse_motion in &engine_state.mouse_motion_events {
            cumulative_motion += mouse_motion.delta
        }
        // There seems to be a Bevy 0.5 bug where every other frame we don't receive any mouse
        // motion events, so ignore those frames.
        // TODO: Follow up on this bug in upstream Bevy
        if cumulative_motion != Vec2::ZERO {
            actor.translation = cumulative_motion + ORIGIN_LOCATION.into();
        }
    }
    true
}
