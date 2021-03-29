//
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut layer = 0.0;
    let preset_iterator = ActorPreset::iterator().peekable();
    for (x, preset) in (-300..=600).step_by(30).zip(preset_iterator) {
        game.add_actor("preset".into(), preset)
            .set_translation(Vec2::new(x as f32, (-x) as f32))
            .set_layer(layer);
        layer += 1.0;
    }

    game.run();
}
