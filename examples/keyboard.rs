use std::f32::consts::PI;

use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Race Car", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    let instructions = "Keyboard Example\n================\nChange translation (move): w a s d / arrows\nChange Rotation: z c\nChange Scale: + -";
    let text = game.add_text_actor("instructions", instructions);
    text.translation.y = 250.0;

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    // Compute how fast we should move, rotate, and scale
    let move_amount = 200.0 * game_state.delta_f32;
    let rotation_amount = PI * game_state.delta_f32;
    let scale_amount = 1.0 * game_state.delta_f32;

    // Get the race car actor
    let race_car = game_state.actors.get_mut("Race Car").unwrap();

    // Handle keyboard input
    let ks = &mut game_state.keyboard_state;
    if ks.pressed_any(&[KeyCode::A, KeyCode::Left]) {
        race_car.translation.x -= move_amount;
    }
    if ks.pressed_any(&[KeyCode::D, KeyCode::Right, KeyCode::E]) {
        race_car.translation.x += move_amount;
    }
    if ks.pressed_any(&[KeyCode::O, KeyCode::Down, KeyCode::S]) {
        race_car.translation.y -= move_amount;
    }
    if ks.pressed_any(&[KeyCode::W, KeyCode::Up, KeyCode::Comma]) {
        race_car.translation.y += move_amount;
    }
    if ks.pressed_any(&[KeyCode::Z, KeyCode::Semicolon]) {
        race_car.rotation += rotation_amount;
    }
    if ks.pressed_any(&[KeyCode::C, KeyCode::J]) {
        race_car.rotation -= rotation_amount;
    }
    if ks.pressed_any(&[KeyCode::Plus, KeyCode::Equals]) {
        race_car.scale *= 1.0 + scale_amount;
    }
    if ks.pressed_any(&[KeyCode::Minus, KeyCode::Underline]) {
        race_car.scale *= 1.0 - scale_amount;
    }

    // Clamp the scale to a certain range so the scaling is reasonable
    race_car.scale = race_car.scale.clamp(0.1, 3.0);

    // Clamp the translation so that the car stays on the screen
    race_car.translation = race_car.translation.clamp(
        -game_state.screen_dimensions * 0.5,
        game_state.screen_dimensions * 0.5,
    );
}
