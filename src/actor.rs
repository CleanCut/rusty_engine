use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Actor {
    /// READONLY: A way to identify an actor.
    pub label: String,
    /// READONLY: Which preset was used to create this actor
    pub preset: Option<ActorPreset>,
    /// READONLY: File used for this actor's sprite
    pub filename: String,
    /// SYNCED: Where you are in 2D game space. Positive x is right. Positive y is up. (0.0, 0.0) is the
    /// center of the screen.
    pub translation: Vec2,
    /// SYNCED: Depth of the sprite. 0.0 (back) to 999.0 (front)
    pub layer: f32,
    /// SYNCED: Direction you face in radians. See constants UP, DOWN, LEFT, RIGHT
    pub rotation: f32,
    /// SYNCED: 1.0 is the normal 100%
    pub scale: f32,
}

/// An [`Actor`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all actors.
impl Default for Actor {
    fn default() -> Self {
        Self {
            label: String::default(),
            preset: None,
            filename: String::default(),
            translation: Vec2::default(),
            layer: f32::default(),
            rotation: f32::default(),
            scale: 1.0,
        }
    }
}

impl Actor {
    pub(crate) fn bevy_transform(&self) -> Transform {
        let mut transform = Transform::from_translation(self.translation.extend(self.layer));
        transform.rotation = Quat::from_axis_angle(Vec3::Z, self.rotation);
        transform.scale = Vec3::splat(self.scale);
        transform
    }
    pub(crate) fn bevy_scale(&self) -> Vec3 {
        Vec3::splat(self.scale)
    }
}

use std::array::IntoIter;

#[derive(Copy, Clone, Debug)]
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
    pub fn variant_iter() -> IntoIter<ActorPreset, 20> {
        static ACTOR_PRESETS: [ActorPreset; 20] = [
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
        std::array::IntoIter::new(ACTOR_PRESETS)
    }
}
