//
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut layer = 0.0;
    let preset_iterator = ActorPreset::variant_iter().peekable();
    for (x, actor_preset) in (-300..=600).step_by(30).zip(preset_iterator) {
        let mut actor = game.add_actor(format!("{:?}", actor_preset), actor_preset);
        actor.translation = Vec2::new(x as f32, (-x) as f32);
        actor.layer = layer;
        layer += 1.0;
    }

    // We don't do anything after game setup, so our game logic can be an empty closure
    game.run(|_| {});
}
