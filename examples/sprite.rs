//
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut actor_presets_iter = ActorPreset::variant_iter().peekable();
    'outer: for y in (-265..=400).step_by(175) {
        for x in (-550..=550).step_by(275) {
            if actor_presets_iter.peek().is_none() {
                break 'outer;
            }
            let actor_preset = actor_presets_iter.next().unwrap();
            let mut actor = game.add_actor(format!("{:?}", actor_preset), actor_preset);
            actor.translation = Vec2::new(x as f32, (-y) as f32);
        }
    }

    game.run(|_| {});
}
