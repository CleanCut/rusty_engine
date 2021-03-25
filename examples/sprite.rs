//
use rusty_engine::prelude::*;

fn logic(actor: &mut Actor) {
    println!("{}", actor.name);
}

fn main() {
    // A Game represents the entire game, the entire program that rusty_engine is aware of.
    // By default the game will spawn an empty window, and exit upon Esc or closing of the window.
    let mut game = Game::new();

    // An actor is the basic abstraction for something that can be seen and interacted with.
    // Players, obstacles, platforms, doors, bullets, etc. are all actors.
    //
    // Actors always have:
    // - a name
    // - a simple transform (translation, rotation, scale)
    // and optionally have
    // - a sprite (image)
    // - a collision box
    // - a timer
    game.add_actor("player".into(), ActorPreset::RacingCarRed)
        .set_translation(Vec2::new(-200.0, 0.0));
    game.add_actor("red barrel".into(), ActorPreset::RacingBarrelRed)
        .set_translation(Vec2::new(200.0, 0.0));

    game.add_logic(logic);

    game.run();
}
