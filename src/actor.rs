use crate::physics::Collider;
use bevy::prelude::*;

/// An [`Actor`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all actors.
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
    /// Whether or not to calculate collisions
    pub collision: bool,
    /// Relative to translation
    pub collider: Collider,
}

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
            collision: false,
            collider: Collider::default(),
        }
    }
}

impl Actor {
    #[doc(hidden)]
    pub fn bevy_transform(&self) -> Transform {
        let mut transform = Transform::from_translation(self.translation.extend(self.layer));
        transform.rotation = Quat::from_axis_angle(Vec3::Z, self.rotation);
        transform.scale = Vec3::splat(self.scale);
        transform
    }
    pub fn set_collision(&mut self, value: bool) -> &mut Self {
        self.collision = value;
        self
    }
    pub fn set_collider(&mut self, collider: Collider) -> &mut Self {
        self.collider = collider;
        self
    }
}

use std::array::IntoIter;

/// Actor presets that contain a sprite in the asset pack and have a predetermined collider
#[derive(Copy, Clone, Debug, PartialEq)]
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
    /// Build an actor from a string describing the preset. Useful when "loading" things from a text
    /// file. When writing code from scratch, you should prefer [`ActorPreset::build`].
    pub fn build_from_name(preset_name: String, label: String) -> Actor {
        use ActorPreset::*;
        match preset_name.as_str() {
            "RacingBarrelBlue" => RacingBarrelBlue,
            "RacingBarrelRed" => RacingBarrelRed,
            "RacingBarrierRed" => RacingBarrierRed,
            "RacingBarrierWhite" => RacingBarrierWhite,
            "RacingCarBlack" => RacingCarBlack,
            "RacingCarBlue" => RacingCarBlue,
            "RacingCarGreen" => RacingCarGreen,
            "RacingCarRed" => RacingCarRed,
            "RacingCarYellow" => RacingCarYellow,
            "RacingConeStraight" => RacingConeStraight,
            "RollingBallBlue" => RollingBallBlue,
            "RollingBallBlueAlt" => RollingBallBlueAlt,
            "RollingBallRed" => RollingBallRed,
            "RollingBallRedAlt" => RollingBallRedAlt,
            "RollingBlockCorner" => RollingBlockCorner,
            "RollingBlockNarrow" => RollingBlockNarrow,
            "RollingBlockSmall" => RollingBlockSmall,
            "RollingBlockSquare" => RollingBlockSquare,
            "RollingHoleEnd" => RollingHoleEnd,
            "RollingHoleStart" => RollingHoleStart,
            _ => panic!(
                "Cannot find preset named {}, does it need to be added to the list?",
                preset_name
            ),
        }
        .build(label)
    }

    /// Build a usable actor from this preset. This is called for you if you use
    /// [`GameState::add_actor`].
    pub fn build(self, label: String) -> Actor {
        let filename = self.filename();
        let collider = self.collider();
        Actor {
            label,
            preset: Some(self),
            filename,
            collider,
            ..Default::default()
        }
    }

    /// Retrieve the collider for a preset. You don't usually need to call this yourself, as the
    /// `.build*` methods will call it for you.
    pub fn collider(&self) -> Collider {
        match self {
            ActorPreset::RacingBarrelBlue => Collider::circle(28.0),
            ActorPreset::RacingBarrelRed => Collider::circle(28.0),
            ActorPreset::RacingBarrierRed => {
                Collider::rect(Vec2::new(-105.0, 31.0), Vec2::new(105.0, -31.0))
            }
            ActorPreset::RacingBarrierWhite => {
                Collider::rect(Vec2::new(-105.0, 31.0), Vec2::new(105.0, -31.0))
            }
            ActorPreset::RacingCarBlack => Collider::poly(&[
                (-59., 28.),
                (-58., 31.),
                (-54., 34.),
                (51., 34.),
                (56., 31.5),
                (58.5, 28.5),
                (58.5, -26.),
                (57.5, -29.5),
                (52.5, -33.5),
                (-54.5, -33.5),
                (-59., -29.),
            ]),
            ActorPreset::RacingCarBlue => Collider::poly(&[
                (-59., 28.),
                (-58., 31.),
                (-54., 34.),
                (51., 34.),
                (56., 31.5),
                (58.5, 28.5),
                (58.5, -26.),
                (57.5, -29.5),
                (52.5, -33.5),
                (-54.5, -33.5),
                (-59., -29.),
            ]),
            ActorPreset::RacingCarGreen => Collider::poly(&[
                (-59., 28.),
                (-58., 31.),
                (-54., 34.),
                (51., 34.),
                (56., 31.5),
                (58.5, 28.5),
                (58.5, -26.),
                (57.5, -29.5),
                (52.5, -33.5),
                (-54.5, -33.5),
                (-59., -29.),
            ]),
            ActorPreset::RacingCarRed => Collider::poly(&[
                (-59., 28.),
                (-58., 31.),
                (-54., 34.),
                (51., 34.),
                (56., 31.5),
                (58.5, 28.5),
                (58.5, -26.),
                (57.5, -29.5),
                (52.5, -33.5),
                (-54.5, -33.5),
                (-59., -29.),
            ]),
            ActorPreset::RacingCarYellow => Collider::poly(&[
                (-59., 28.),
                (-58., 31.),
                (-54., 34.),
                (51., 34.),
                (56., 31.5),
                (58.5, 28.5),
                (58.5, -26.),
                (57.5, -29.5),
                (52.5, -33.5),
                (-54.5, -33.5),
                (-59., -29.),
            ]),
            ActorPreset::RacingConeStraight => {
                Collider::rect(Vec2::new(-22.0, 22.0), Vec2::new(22.0, -22.0))
            }
            ActorPreset::RollingBallBlue => Collider::circle(18.0),
            ActorPreset::RollingBallBlueAlt => Collider::circle(18.0),
            ActorPreset::RollingBallRed => Collider::circle(18.0),
            ActorPreset::RollingBallRedAlt => Collider::circle(18.0),
            ActorPreset::RollingBlockCorner => Collider::poly(&[
                (-64., 61.),
                (-64.0, 64.),
                (-56., 64.),
                (64., -56.),
                (64., -61.),
                (61., -64.),
                (-62., -64.),
                (-64., -62.),
            ]),
            ActorPreset::RollingBlockNarrow => Collider::rect((-64., 16.), (64., -16.)),
            ActorPreset::RollingBlockSmall => {
                Collider::rect(Vec2::new(-16.0, 16.0), Vec2::new(16.0, -16.0))
            }
            ActorPreset::RollingBlockSquare => {
                Collider::rect(Vec2::new(-32.0, 32.0), Vec2::new(32.0, -32.0))
            }
            ActorPreset::RollingHoleEnd => Collider::circle(18.0),
            ActorPreset::RollingHoleStart => Collider::circle(24.0),
        }
    }

    /// Retrieve the asset filename. You probably won't need this method, as it is called internally
    /// by the `.build*` methods
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

    /// An iterator that iterates through presets. Mostly useful for things like level builders
    /// when you want to be able to rotate something through each preset.
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
        ACTOR_PRESETS.into_iter()
    }

    fn shifted_by(&self, amount: isize) -> ActorPreset {
        let len = ActorPreset::variant_iter().len();
        let index = ActorPreset::variant_iter()
            .enumerate()
            .find(|(_, a)| *a == *self)
            .unwrap()
            .0;
        let mut new_index_isize = index as isize + amount;
        while new_index_isize < 0 {
            new_index_isize += len as isize;
        }
        let new_index = (new_index_isize as usize) % len;
        ActorPreset::variant_iter().nth(new_index).unwrap()
    }

    /// Just get the next actor preset in the list, without dealing with an iterator
    pub fn next(&self) -> ActorPreset {
        self.shifted_by(-1)
    }

    /// Just get the previous actor preset in the list, without dealing with an iterator
    pub fn prev(&self) -> ActorPreset {
        self.shifted_by(1)
    }
}
