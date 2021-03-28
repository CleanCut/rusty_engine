//
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let variants = vec![
        ActorPreset::RacingBarrelBlue,
        ActorPreset::RacingBarrelRed,
        ActorPreset::RacingBarrierRed,
        ActorPreset::RacingBarrierWhite,
        ActorPreset::RacingCarBlack,
        ActorPreset::RacingCarBlue,
        ActorPreset::RacingCarGreen,
        ActorPreset::RacingCarRed,
        ActorPreset::RacingCarYellow,
        ActorPreset::RacingConeStraight,
        ActorPreset::RollingBallBlueAlt,
        ActorPreset::RollingBallBlue,
        ActorPreset::RollingBallRedAlt,
        ActorPreset::RollingBallRed,
        ActorPreset::RollingBlockCorner,
        ActorPreset::RollingBlockNarrow,
        ActorPreset::RollingBlockSmall,
        ActorPreset::RollingBlockSquare,
        ActorPreset::RollingHoleEnd,
        ActorPreset::RollingHoleStart,
    ];

    let mut presets_iter = variants.into_iter().enumerate().peekable();
    'outer: for y in (-265..=400).step_by(175) {
        for x in (-550..=550).step_by(275) {
            if presets_iter.peek().is_none() {
                break 'outer;
            }
            let (i, preset) = presets_iter.next().unwrap();
            let name: String = "preset".to_string() + i.to_string().as_str();
            game.add_actor(name, preset)
                .set_translation(Vec2::new(x as f32, (-y) as f32));
        }
    }

    game.run();
}
