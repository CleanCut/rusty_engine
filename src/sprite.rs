use crate::physics::Collider;
use bevy::prelude::*;

/// An [`Sprite`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all sprites.
#[derive(Clone, Debug)]
pub struct Sprite {
    /// READONLY: A way to identify a sprite.
    pub label: String,
    /// READONLY: Which preset was used to create this sprite
    pub preset: Option<SpritePreset>,
    /// READONLY: File used for this sprite's image
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

impl Default for Sprite {
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

impl Sprite {
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

/// Sprite presets using the asset pack have a all have images and colliders
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpritePreset {
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

impl SpritePreset {
    /// Build an sprite from a string describing the preset. Useful when "loading" things from a text
    /// file. When writing code from scratch, you should prefer [`SpritePreset::build`].
    pub fn build_from_name(preset_name: String, label: String) -> Sprite {
        use SpritePreset::*;
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

    /// Build a usable sprite from this preset. This is called for you if you use
    /// [`EngineState::add_sprite`](crate::prelude::EngineState::add_sprite).
    pub fn build(self, label: String) -> Sprite {
        let filename = self.filename();
        let collider = self.collider();
        Sprite {
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
            SpritePreset::RacingBarrelBlue => Collider::circle(28.0),
            SpritePreset::RacingBarrelRed => Collider::circle(28.0),
            SpritePreset::RacingBarrierRed => {
                Collider::rect(Vec2::new(-105.0, 31.0), Vec2::new(105.0, -31.0))
            }
            SpritePreset::RacingBarrierWhite => {
                Collider::rect(Vec2::new(-105.0, 31.0), Vec2::new(105.0, -31.0))
            }
            SpritePreset::RacingCarBlack => Collider::poly(&[
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
            SpritePreset::RacingCarBlue => Collider::poly(&[
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
            SpritePreset::RacingCarGreen => Collider::poly(&[
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
            SpritePreset::RacingCarRed => Collider::poly(&[
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
            SpritePreset::RacingCarYellow => Collider::poly(&[
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
            SpritePreset::RacingConeStraight => {
                Collider::rect(Vec2::new(-22.0, 22.0), Vec2::new(22.0, -22.0))
            }
            SpritePreset::RollingBallBlue => Collider::circle(18.0),
            SpritePreset::RollingBallBlueAlt => Collider::circle(18.0),
            SpritePreset::RollingBallRed => Collider::circle(18.0),
            SpritePreset::RollingBallRedAlt => Collider::circle(18.0),
            SpritePreset::RollingBlockCorner => Collider::poly(&[
                (-64., 61.),
                (-64.0, 64.),
                (-56., 64.),
                (64., -56.),
                (64., -61.),
                (61., -64.),
                (-62., -64.),
                (-64., -62.),
            ]),
            SpritePreset::RollingBlockNarrow => Collider::rect((-64., 16.), (64., -16.)),
            SpritePreset::RollingBlockSmall => {
                Collider::rect(Vec2::new(-16.0, 16.0), Vec2::new(16.0, -16.0))
            }
            SpritePreset::RollingBlockSquare => {
                Collider::rect(Vec2::new(-32.0, 32.0), Vec2::new(32.0, -32.0))
            }
            SpritePreset::RollingHoleEnd => Collider::circle(18.0),
            SpritePreset::RollingHoleStart => Collider::circle(24.0),
        }
    }

    /// Retrieve the asset filename. You probably won't need this method, as it is called internally
    /// by the `.build*` methods
    pub fn filename(&self) -> String {
        match self {
            SpritePreset::RacingBarrelBlue => "sprite/racing/barrel_blue.png",
            SpritePreset::RacingBarrelRed => "sprite/racing/barrel_red.png",
            SpritePreset::RacingBarrierRed => "sprite/racing/barrier_red.png",
            SpritePreset::RacingBarrierWhite => "sprite/racing/barrier_white.png",
            SpritePreset::RacingCarBlack => "sprite/racing/car_black.png",
            SpritePreset::RacingCarBlue => "sprite/racing/car_blue.png",
            SpritePreset::RacingCarGreen => "sprite/racing/car_green.png",
            SpritePreset::RacingCarRed => "sprite/racing/car_red.png",
            SpritePreset::RacingCarYellow => "sprite/racing/car_yellow.png",
            SpritePreset::RacingConeStraight => "sprite/racing/cone_straight.png",
            SpritePreset::RollingBallBlue => "sprite/rolling/ball_blue.png",
            SpritePreset::RollingBallBlueAlt => "sprite/rolling/ball_blue_alt.png",
            SpritePreset::RollingBallRed => "sprite/rolling/ball_red.png",
            SpritePreset::RollingBallRedAlt => "sprite/rolling/ball_red_alt.png",
            SpritePreset::RollingBlockCorner => "sprite/rolling/block_corner.png",
            SpritePreset::RollingBlockNarrow => "sprite/rolling/block_narrow.png",
            SpritePreset::RollingBlockSmall => "sprite/rolling/block_small.png",
            SpritePreset::RollingBlockSquare => "sprite/rolling/block_square.png",
            SpritePreset::RollingHoleEnd => "sprite/rolling/hole_end.png",
            SpritePreset::RollingHoleStart => "sprite/rolling/hole_start.png",
        }
        .into()
    }

    /// An iterator that iterates through presets. Mostly useful for things like level builders
    /// when you want to be able to rotate something through each preset.
    pub fn variant_iter() -> IntoIter<SpritePreset, 20> {
        static SPRITE_PRESETS: [SpritePreset; 20] = [
            SpritePreset::RacingBarrelBlue,
            SpritePreset::RacingBarrelRed,
            SpritePreset::RacingBarrierRed,
            SpritePreset::RacingBarrierWhite,
            SpritePreset::RacingCarBlack,
            SpritePreset::RacingCarBlue,
            SpritePreset::RacingCarGreen,
            SpritePreset::RacingCarRed,
            SpritePreset::RacingCarYellow,
            SpritePreset::RacingConeStraight,
            SpritePreset::RollingBallBlueAlt,
            SpritePreset::RollingBallBlue,
            SpritePreset::RollingBallRedAlt,
            SpritePreset::RollingBallRed,
            SpritePreset::RollingBlockCorner,
            SpritePreset::RollingBlockNarrow,
            SpritePreset::RollingBlockSmall,
            SpritePreset::RollingBlockSquare,
            SpritePreset::RollingHoleEnd,
            SpritePreset::RollingHoleStart,
        ];
        SPRITE_PRESETS.into_iter()
    }

    fn shifted_by(&self, amount: isize) -> SpritePreset {
        let len = SpritePreset::variant_iter().len();
        let index = SpritePreset::variant_iter()
            .enumerate()
            .find(|(_, a)| *a == *self)
            .unwrap()
            .0;
        let mut new_index_isize = index as isize + amount;
        while new_index_isize < 0 {
            new_index_isize += len as isize;
        }
        let new_index = (new_index_isize as usize) % len;
        SpritePreset::variant_iter().nth(new_index).unwrap()
    }

    /// Just get the next sprite preset in the list, without dealing with an iterator
    pub fn next(&self) -> SpritePreset {
        self.shifted_by(-1)
    }

    /// Just get the previous sprite preset in the list, without dealing with an iterator
    pub fn prev(&self) -> SpritePreset {
        self.shifted_by(1)
    }
}
