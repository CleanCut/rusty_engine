//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example keyboard_state

use std::f32::consts::PI;

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    let race_car = game.add_sprite("Race Car", SpritePreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    let instructions = "Smooth movement with KeyboardState Example\n====================================\nChange translation (move): w a s d / arrows\nChange Rotation: z c\nChange Scale: + -";
    let text = game.add_text("instructions", instructions);
    text.translation.y = 250.0;

    game.add_logic(logic);
    game.run(GameState {});
}

fn logic(engine: &mut Engine, _: &mut GameState) {
    // Compute how fast we should move, rotate, and scale
    let move_amount = 200.0 * engine.delta_f32;
    let rotation_amount = PI * engine.delta_f32;
    let scale_amount = 1.0 * engine.delta_f32;

    // Get the race car sprite
    let race_car = engine.sprites.get_mut("Race Car").unwrap();

    // Handle keyboard input
    let ks = &mut engine.keyboard_state;
    if ks.pressed_any(&[KeyCode::W, KeyCode::Up, KeyCode::Comma]) {
        race_car.translation.y += move_amount;
    }
    if ks.pressed_any(&[KeyCode::A, KeyCode::Left]) {
        race_car.translation.x -= move_amount;
    }
    if ks.pressed_any(&[KeyCode::S, KeyCode::Down, KeyCode::O]) {
        race_car.translation.y -= move_amount;
    }
    if ks.pressed_any(&[KeyCode::D, KeyCode::Right, KeyCode::E]) {
        race_car.translation.x += move_amount;
    }

    // If you prefer a more functional style that is equivalent to the kind of logic above,
    // but takes closures to run if the buttons are pressed, you can call `.chain()`
    ks.chain()
        .pressed_any(&[KeyCode::Z, KeyCode::Semicolon], |_| {
            race_car.rotation += rotation_amount;
        })
        .pressed_any(&[KeyCode::C, KeyCode::J], |_| {
            race_car.rotation -= rotation_amount;
        })
        .pressed_any(&[KeyCode::Plus, KeyCode::Equals], |_| {
            race_car.scale *= 1.0 + scale_amount;
        })
        .pressed_any(&[KeyCode::Minus, KeyCode::Underline], |_| {
            race_car.scale *= 1.0 - scale_amount;
        });

    // Clamp the scale to a certain range so the scaling is reasonable
    race_car.scale = race_car.scale.clamp(0.1, 3.0);

    // Clamp the translation so that the car stays on the screen
    race_car.translation = race_car.translation.clamp(
        -engine.window_dimensions * 0.5,
        engine.window_dimensions * 0.5,
    );
}
