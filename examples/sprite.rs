//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example sprite

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    let mut sprite_presets_iter = SpritePreset::variant_iter().peekable();
    'outer: for y in (-290..=400).step_by(175) {
        for x in (-530..=530).step_by(265) {
            if sprite_presets_iter.peek().is_none() {
                break 'outer;
            }
            let sprite_preset = sprite_presets_iter.next().unwrap();
            let sprite_string = format!("{:?}", sprite_preset);
            let sprite = game.add_sprite(&sprite_string, sprite_preset);
            sprite.translation = Vec2::new(x as f32, (-y) as f32);

            let text = game.add_text(&sprite_string, &sprite_string);
            text.translation = Vec2::new(x as f32, (-y - 75) as f32);
            text.font_size = 22.0;
        }
    }

    game.run(GameState {});
}
