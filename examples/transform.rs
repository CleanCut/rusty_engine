//
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.add_actor("Race Car".to_string(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(-100.0, 0.0))
        .set_rotation(UP)
        .set_scale(0.5);

    game.add_logic(logic);

    game.run();
}

fn logic(actor: &mut Actor) {
    //println!("Processing logic for {}", actor.name);
    actor.scale += 0.01;
}
