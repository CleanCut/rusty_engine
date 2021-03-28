//
use rusty_engine::prelude::*;

#[derive(Default)]
struct DataForMySystemsToUse {
    value: u32,
}
unsafe impl Sync for DataForMySystemsToUse {}
unsafe impl Send for DataForMySystemsToUse {}

fn main() {
    let mut game = Game::new(DataForMySystemsToUse { value: 0 });

    game.add_actor("Race Car".to_string(), ActorPreset::RacingCarGreen)
        .set_translation(Vec2::new(0.0, 0.0))
        .set_rotation(UP)
        .set_scale(1.0);

    game.add_logic(logic);

    game.run();
}

fn logic(actor: &mut Actor, time: &Time) {
    actor.rotation += time.delta_seconds() * 3.0;
}
