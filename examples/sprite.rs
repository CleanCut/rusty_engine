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
            let actor_string = format!("{:?}", actor_preset);
            let mut actor = game.add_actor(&actor_string, actor_preset);
            actor.translation = Vec2::new(x as f32, (-y) as f32);

            let mut text_actor = game.add_text_actor(&actor_string, &actor_string);
            text_actor.translation = Vec2::new(x as f32, (-y - 70) as f32);
            text_actor.font_size = 22.0;
        }
    }

    game.run(|_| {});
}
