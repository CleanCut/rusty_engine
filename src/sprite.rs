/// Sprites are the images that make up a game
use bevy::prelude::{Component, Quat, Transform, Vec2, Vec3};

use crate::physics::Collider;

/// A [`Sprite`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all sprites.
#[derive(Clone, Component, Debug, PartialEq)]
pub struct Sprite {
    /// READONLY: A way to identify a sprite. This must be unique, or else the game will crash.
    pub label: String,
    /// READONLY: File used for this sprite's image
    pub filepath: PathBuf,
    /// READONLY: File used for this sprite's collider. Note that this file will not exist if the
    /// sprite does not have a collider, but if you set the `collider` field to a collider and then
    /// call the `write_collider` method, the file will be written for you!
    pub collider_filepath: PathBuf,
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
    /// The actual collider for this sprite
    pub collider: Collider,
    /// If set to `true`, then the collider shown for this sprite will be regenerated (see also
    /// [`Engine.show_colliders`](crate::prelude::Engine)). Normally you shouldn't touch this, but
    /// if you manually replace a `Sprite`'s [`Collider`] in a game logic function, then you need to
    /// set this to true.
    pub collider_dirty: bool,
}

/// Reads the collider file and creates the collider
fn read_collider_from_file(filepath: &Path) -> Collider {
    match File::open(filepath) {
        Ok(fh) => match ron::de::from_reader::<_, Collider>(fh) {
            Ok(collider) => collider,
            Err(e) => {
                eprintln!("failed deserializing collider from file: {}", e);
                Collider::NoCollider
            }
        },
        Err(e) => {
            eprintln!("failed to open collider file: {}", e);
            Collider::NoCollider
        }
    }
}

impl Sprite {
    /// `label` should be a unique string (it will be used as a key in the hashmap
    /// [`Engine::sprites`](crate::prelude::Engine)). `file_or_preset` should either be a
    /// [`SpritePreset`] variant, or a relative path to an image file inside the `assets/`
    /// directory. If a collider definition exists in a file with the same name as the image file,
    /// but with the `.collider` extension, then the collider will be loaded automatically. To
    /// create a collider file you can either run the `collider` example, or
    /// programmatically create a [`Collider`], set the sprite's `.collider` field to it, and call
    /// the sprite's `.write_collider()` method.  All presets have collider files already.
    pub fn new<S: Into<String>, P: Into<PathBuf>>(label: S, file_or_preset: P) -> Self {
        let label = label.into();
        let filepath = file_or_preset.into();
        let mut collider_filepath = filepath.clone();
        collider_filepath.set_extension("collider");
        let actual_collider_filepath = PathBuf::from("assets").join(&collider_filepath);
        let collider = if actual_collider_filepath.exists() {
            read_collider_from_file(actual_collider_filepath.as_path())
        } else {
            eprintln!(
                "warning: could not find collider file {} -- consider creating one with the `collider` example.",
                actual_collider_filepath.to_string_lossy()
            );
            Collider::NoCollider
        };
        Self {
            label,
            filepath,
            collider_filepath,
            translation: Vec2::default(),
            layer: f32::default(),
            rotation: f32::default(),
            scale: 1.0,
            collision: false,
            collider,
            collider_dirty: true,
        }
    }

    /// Do the math to convert from Rusty Engine translation+rotation+scale+layer to Bevy's Transform
    #[doc(hidden)]
    pub fn bevy_transform(&self) -> Transform {
        let mut transform = Transform::from_translation(self.translation.extend(self.layer));
        transform.rotation = Quat::from_axis_angle(Vec3::Z, self.rotation);
        transform.scale = Vec3::splat(self.scale);
        transform
    }

    /// Attempt to take the current collider and write it to collider_filepath. If there isn't a
    /// collider, or writing fails, then `false` is returned. Otherwise `true` is returned.
    pub fn write_collider(&self) -> bool {
        if self.collider == Collider::NoCollider {
            return false;
        }
        // Bevy's asset system is relative from the assets/ subdirectory, so we must be too
        let filepath = PathBuf::from("assets").join(self.collider_filepath.clone());
        let mut fh = match File::create(filepath) {
            Ok(fh) => fh,
            Err(e) => {
                eprintln!("failed creating collider file: {}", e);
                return false;
            }
        };

        let collider_ron = match ron::ser::to_string_pretty(&self.collider, Default::default()) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("failed converting collider to ron: {}", e);
                return false;
            }
        };
        match fh.write_all(collider_ron.as_bytes()) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("failed writing collider file: {}", e);
                false
            }
        }
    }
    /// Add a collider point. `p` is a `Vec2` in worldspace (usually the mouse coordinate). See the
    /// `collider` example.
    pub fn add_collider_point(&mut self, mut p: Vec2) {
        self.collider_dirty = true;
        // If there isn't a collider, we better switch to one
        if self.collider == Collider::NoCollider {
            self.collider = Collider::Poly(Vec::new());
        }
        // Add the current point to the collider
        if let Collider::Poly(points) = &mut self.collider {
            // untranslate (make p relative to the sprite's position)
            p -= self.translation;
            // unscale (make p the same scale as the sprite)
            p *= 1.0 / self.scale;
            // unrotate (make p the same rotation as the sprite)
            let mut p2 = Vec2::ZERO;
            let sin = (-self.rotation).sin();
            let cos = (-self.rotation).cos();
            p2.x = p.x * cos - p.y * sin;
            p2.y = p.x * sin + p.y * cos;
            points.push(p2);
        }
    }
    /// Change the last collider point. `p` is a `Vec2` in worldspace (usually the mouse
    /// coordinate). See the `collider` example.
    pub fn change_last_collider_point(&mut self, mut p: Vec2) {
        self.collider_dirty = true;
        // If there isn't a collider, create one with a "last point" to change
        if self.collider == Collider::NoCollider {
            self.collider = Collider::Poly(vec![Vec2::ZERO]);
        }
        // Add the current point to the collider
        if let Collider::Poly(points) = &mut self.collider {
            // If the collider exists, but doesn't have any points, add a "last point" to modify.
            if points.is_empty() {
                points.push(Vec2::ZERO);
            }
            // untranslate (make p relative to the sprite's origin instead of the world's origin)
            p -= self.translation;
            // unscale (make p the same scale as the sprite)
            p *= 1.0 / self.scale;
            // unrotate (make p the same rotation as the sprite)
            let length = points.len();
            let p2 = points.get_mut(length - 1).unwrap(); // mutable reference to "last point"
            let sin = (-self.rotation).sin();
            let cos = (-self.rotation).cos();
            p2.x = p.x * cos - p.y * sin;
            p2.y = p.x * sin + p.y * cos;
        }
    }
}

use std::{
    array::IntoIter,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Sprite presets using the asset pack all have colliders
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    /// Retrieve the asset filepath. You probably won't need to call this method, since the methods
    /// which create [`Sprite`]s will accept [`SpritePreset`]s and call this method via the
    /// `impl From<SpritePreset> for PathBuf` implementation.
    pub fn filepath(&self) -> PathBuf {
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

    /// The core logic of both `next` and `prev`
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

impl From<SpritePreset> for PathBuf {
    fn from(sprite_preset: SpritePreset) -> Self {
        sprite_preset.filepath()
    }
}
