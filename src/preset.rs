use crate::actor::Actor;

#[derive(Clone, Debug)]
pub enum ActorPreset {
    RacingBarrelBlue,
    RacingBarrelRed,
    RacingBarrierRed,
    RacingBarrierWhite,
    RacingCarBlack,
    RacingCarBlue,
    RacingCarGreen,
    RacingCarRed,
    RacingCarYellow,
    RacingConeStraight,
    RollingBallBlue,
    RollingBallBlueAlt,
    RollingBallRed,
    RollingBallRedAlt,
    RollingBlockCorner,
    RollingBlockNarrow,
    RollingBlockSmall,
    RollingBlockSquare,
    RollingHoleEnd,
    RollingHoleStart,
}

impl ActorPreset {
    pub fn build(self, label: String) -> Actor {
        let filename = self.filename();
        Actor {
            label,
            preset: Some(self),
            filename,
            ..Default::default()
        }
    }
    pub fn filename(&self) -> String {
        match self {
            ActorPreset::RacingBarrelBlue => "sprite/racing/barrel_blue.png",
            ActorPreset::RacingBarrelRed => "sprite/racing/barrel_red.png",
            ActorPreset::RacingBarrierRed => "sprite/racing/barrier_red.png",
            ActorPreset::RacingBarrierWhite => "sprite/racing/barrier_white.png",
            ActorPreset::RacingCarBlack => "sprite/racing/car_black.png",
            ActorPreset::RacingCarBlue => "sprite/racing/car_blue.png",
            ActorPreset::RacingCarGreen => "sprite/racing/car_green.png",
            ActorPreset::RacingCarRed => "sprite/racing/car_red.png",
            ActorPreset::RacingCarYellow => "sprite/racing/car_yellow.png",
            ActorPreset::RacingConeStraight => "sprite/racing/cone_straight.png",
            ActorPreset::RollingBallBlue => "sprite/rolling/ball_blue.png",
            ActorPreset::RollingBallBlueAlt => "sprite/rolling/ball_blue_alt.png",
            ActorPreset::RollingBallRed => "sprite/rolling/ball_red.png",
            ActorPreset::RollingBallRedAlt => "sprite/rolling/ball_red_alt.png",
            ActorPreset::RollingBlockCorner => "sprite/rolling/block_corner.png",
            ActorPreset::RollingBlockNarrow => "sprite/rolling/block_narrow.png",
            ActorPreset::RollingBlockSmall => "sprite/rolling/block_small.png",
            ActorPreset::RollingBlockSquare => "sprite/rolling/block_square.png",
            ActorPreset::RollingHoleEnd => "sprite/rolling/hole_end.png",
            ActorPreset::RollingHoleStart => "sprite/rolling/hole_start.png",
        }
        .into()
    }
}
