//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example placement

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    let car1 = game.add_sprite("car1", SpritePreset::RacingCarRed);
    car1.translation = Vec2::new(-300.0, 0.0);
    car1.rotation = UP;
    car1.scale = 1.0;

    let car2 = game.add_sprite("car2", SpritePreset::RacingCarGreen);
    car2.translation = Vec2::new(0.0, 0.0);
    car2.rotation = UP;
    car2.scale = 1.0;

    let car3 = game.add_sprite("car3", SpritePreset::RacingCarBlue);
    car3.translation = Vec2::new(300.0, 0.0);
    car3.rotation = UP;
    car3.scale = 1.0;

    game.add_logic(logic);
    game.run(GameState {});
}

fn logic(engine: &mut Engine, _: &mut GameState) {
    let car1 = engine.sprites.get_mut("car1").unwrap();
    car1.translation.x = -300.0 + (engine.time_since_startup_f64.cos() * 100.0) as f32;
    car1.translation.y = (engine.time_since_startup_f64.sin() * 100.0) as f32;

    let car2 = engine.sprites.get_mut("car2").unwrap();
    car2.scale = ((engine.time_since_startup_f64 * 0.5).cos().abs() * 2.0) as f32;

    let car3 = engine.sprites.get_mut("car3").unwrap();
    car3.rotation = engine.time_since_startup_f64 as f32;
}
