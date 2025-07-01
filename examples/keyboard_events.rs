//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example keyboard_events

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    let race_car = game.add_sprite("Race Car", SpritePreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.scale = 1.0;

    let instructions = "Discrete Movement with Keyboard Events\n==============================\nChange translation (move): w a s d / arrows\nChange Rotation: z c\nChange Scale: + -";
    let text = game.add_text("instructions", instructions);
    text.translation.y = 250.0;

    game.add_logic(logic);
    game.run(GameState {});
}

fn logic(engine: &mut Engine, _: &mut GameState) {
    // Get the race car sprite
    let race_car = engine.sprites.get_mut("Race Car").unwrap();

    // Loop through any keyboard input that hasn't been processed this frame
    for keyboard_event in &engine.keyboard_events {
        if let KeyboardInput {
            key_code,
            state: ButtonState::Pressed,
            window: _,
            logical_key: _,
        } = keyboard_event
        {
            // Handle various keypresses. The extra keys are for the Dvorak keyboard layout. ;-)
            match key_code {
                KeyCode::KeyA | KeyCode::ArrowLeft => race_car.translation.x -= 10.0,
                KeyCode::KeyD | KeyCode::ArrowRight => race_car.translation.x += 10.0,
                KeyCode::KeyO | KeyCode::ArrowDown => race_car.translation.y -= 10.0,
                KeyCode::KeyW | KeyCode::ArrowUp  => race_car.translation.y += 10.0,
                KeyCode::KeyZ => race_car.rotation += std::f32::consts::FRAC_PI_4,
                KeyCode::KeyC => race_car.rotation -= std::f32::consts::FRAC_PI_4,
                KeyCode::Equal => race_car.scale *= 1.1,
                KeyCode::Minus  => race_car.scale *= 0.9,
                _ => {}
            }

            // Clamp the scale to a certain range so the scaling is reasonable
            race_car.scale = race_car.scale.clamp(0.1, 3.0);

            // Clamp the translation so that the car stays on the screen
            race_car.translation = race_car.translation.clamp(
                -engine.window_dimensions * 0.5,
                engine.window_dimensions * 0.5,
            );
        }
    }
}
