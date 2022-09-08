//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example keyboard_state

use std::f32::consts::PI;

use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_sprite("Race Car", SpritePreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    let instructions = "Smooth movement with KeyboardState Example\n====================================\nChange translation (move): w a s d / arrows\nChange Rotation: z c\nChange Scale: + -";
    let text = game.add_text("instructions", instructions);
    text.translation.y = 250.0;

    game.add_logic(logic);
    game.run(());
}

fn logic(game_state: &mut Engine, _: &mut ()) {
    // Compute how fast we should move, rotate, and scale
    let move_amount = 200.0 * game_state.delta_f32;
    let rotation_amount = PI * game_state.delta_f32;
    let scale_amount = 1.0 * game_state.delta_f32;

    // Get the race car sprite
    let race_car = game_state.sprites.get_mut("Race Car").unwrap();

    // Handle keyboard input
    let ks = &mut game_state.keyboard_state;

    ks //
        .pressed_any_do(&[KeyCode::A, KeyCode::Left], |_| {
            race_car.translation.x -= move_amount;
        })
        .pressed_any_do(&[KeyCode::D, KeyCode::Right, KeyCode::E], |_| {
            race_car.translation.x += move_amount;
        })
        .pressed_any_do(&[KeyCode::O, KeyCode::Down, KeyCode::S], |_| {
            race_car.translation.y -= move_amount;
        })
        .pressed_any_do(&[KeyCode::W, KeyCode::Up, KeyCode::Comma], |_| {
            race_car.translation.y += move_amount;
        })
        .pressed_any_do(&[KeyCode::Z, KeyCode::Semicolon], |_| {
            race_car.rotation += rotation_amount;
        })
        .pressed_any_do(&[KeyCode::C, KeyCode::J], |_| {
            race_car.rotation -= rotation_amount;
        })
        .pressed_any_do(&[KeyCode::Plus, KeyCode::Equals], |_| {
            race_car.scale *= 1.0 + scale_amount;
        })
        .pressed_any_do(&[KeyCode::Minus, KeyCode::Underline], |_| {
            race_car.scale *= 1.0 - scale_amount;
        });

    // Clamp the scale to a certain range so the scaling is reasonable
    race_car.scale = race_car.scale.clamp(0.1, 3.0);

    // Clamp the translation so that the car stays on the screen
    race_car.translation = race_car.translation.clamp(
        -game_state.window_dimensions * 0.5,
        game_state.window_dimensions * 0.5,
    );
}
