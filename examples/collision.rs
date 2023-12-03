//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example collision

use rusty_engine::prelude::*;

const ROTATION_SPEED: f32 = 3.0;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();
    let msg2 = game.add_text(
        "instructions",
        "Move the car with your mouse. Rotate it by holding left/right mouse buttons. Scale it with the mousewheel. Toggle collider visibility with C.",
    );
    msg2.font_size = 20.0;
    msg2.translation.y = 340.0;

    let race_car = game.add_sprite("Player", SpritePreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.layer = 100.0;
    race_car.collision = true;

    let mut sprite_presets_iter = SpritePreset::variant_iter().peekable();
    'outer: for y in (-265..=400).step_by(175) {
        for x in (-550..=550).step_by(275) {
            if sprite_presets_iter.peek().is_none() {
                break 'outer;
            }
            let sprite_preset = sprite_presets_iter.next().unwrap();
            let sprite = game.add_sprite(format!("{:?}", sprite_preset), sprite_preset);
            sprite.translation = Vec2::new(x as f32, (-y) as f32);
            sprite.collision = true;
        }
    }

    let text = game.add_text("collision text", "");
    text.translation = Vec2::new(0.0, -200.0);

    game.add_logic(logic);
    game.run(GameState {});
}

fn logic(engine: &mut Engine, _: &mut GameState) {
    // If a collision event happened last frame, print it out and play a sound
    for collision_event in engine.collision_events.drain(..) {
        let text = engine.texts.get_mut("collision text").unwrap();
        match collision_event.state {
            CollisionState::Begin => {
                text.value = format!("{:?}", collision_event.pair);
                engine.audio_manager.play_sfx(SfxPreset::Switch1, 1.0)
            }
            CollisionState::End => {
                text.value = "".into();
                engine.audio_manager.play_sfx(SfxPreset::Switch2, 1.0)
            }
        }
    }

    if let Some(sprite) = engine.sprites.get_mut("Player") {
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

        // Mousewheel scales the car
        for mouse_wheel in &engine.mouse_wheel_events {
            sprite.scale *= 1.0 + (0.05 * mouse_wheel.y);
            sprite.scale = sprite.scale.clamp(0.1, 4.0);
        }
    }

    // Pressing C toggles sprite collider debug lines
    if engine.keyboard_state.just_pressed(KeyCode::C) {
        engine.show_colliders = !engine.show_colliders;
    }
}
