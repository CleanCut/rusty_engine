//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example mouse_state

use rusty_engine::prelude::*;

const ORIGIN_LOCATION: (f32, f32) = (0.0, -200.0);
const ROTATION_SPEED: f32 = 3.0;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    let race_car = game.add_sprite("Race Car", "sprite/racing/car_blue.png");
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;
    race_car.layer = 2.0;

    let mover = game.add_sprite("move indicator", SpritePreset::RollingHoleStart);
    mover.translation = ORIGIN_LOCATION.into();
    mover.layer = 1.0;

    let anchor = game.add_sprite("move indicator origin", SpritePreset::RollingHoleEnd);
    anchor.translation = ORIGIN_LOCATION.into();
    anchor.layer = 0.0;

    let msg = game.add_text("relative message", "Relative Mouse Motion Indicator");
    msg.translation.y = -300.0;
    msg.font_size = 20.0;

    let msg2 = game.add_text(
        "instructions",
        "Smooth Movement with Mouse State\n==============================\nMove the car around with your mouse.\nRotate it by holding left/right mouse buttons.\nScale it with the mousewheel.",
    );
    msg2.font_size = 30.0;
    msg2.translation.y = 275.0;

    game.add_logic(logic);
    game.run(GameState {});
}

fn logic(engine: &mut Engine, _: &mut GameState) {
    if let Some(sprite) = engine.sprites.get_mut("Race Car") {
        // Use the latest state of the mouse buttons to rotate the sprite
        let mut rotation_amount = 0.0;
        if engine.mouse_state.pressed(MouseButton::Left) {
            rotation_amount += ROTATION_SPEED * engine.delta_f32;
        }
        if engine.mouse_state.pressed(MouseButton::Right) {
            rotation_amount -= ROTATION_SPEED * engine.delta_f32;
        }
        sprite.rotation += rotation_amount;

        // Use the latest state of the mouse wheel to scale the sprite
        if let Some(location) = engine.mouse_state.location() {
            sprite.translation = location
        }

        // Honestly, this is probably the one "state" thing that you should ignore in favor of
        // processing each event instead (see the mouse_events example), since you can then handle
        // fast spins of the wheel. But here is how to use the mouse wheel state sort of like a
        // button. `wheel_direction` will be `1.0`, `0.0`, or `-1.0` depending on what's going on
        // with the mouse wheel.
        let wheel_direction = engine.mouse_state.wheel().y;
        sprite.scale *= 1.0 + (wheel_direction * 0.1);
        sprite.scale = sprite.scale.clamp(0.1, 4.0);
    }

    // Offset the move indicator from the move indicator origin to visually represent the relative
    // mouse motion for the frame
    if let Some(sprite) = engine.sprites.get_mut("move indicator") {
        let motion = engine.mouse_state.motion();
        // There seems to be a Bevy 0.6 bug where every other frame we don't receive any mouse
        // motion events, so ignore those frames.
        // TODO: Follow up on this bug in upstream Bevy
        if motion != Vec2::ZERO {
            sprite.translation = motion + Vec2::from(ORIGIN_LOCATION);
        }
    }
}
