//! ## Asset Licenses
//!
//! All assets included with this game engine have the appropriate license described and linked to
//! in a `README.md` file in the same directory as the source files. In most cases, the license is
//! [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)--meaning you may do
//! whatever you wish with the asset.
//!
//! One notable exception is some of the music files, which are under a different license and
//! include specific attribution requirements that must be met in order to be used legally when
//! distributed. Please see
//! [this `README.md` file](https://github.com/CleanCut/rusty_engine/tree/main/assets/audio/music)
//! for more information.

pub mod actor;
pub mod audio;
pub mod consts;
pub mod game;
pub mod mouse;
pub mod physics;

// Public prelude
pub mod prelude {
    pub use crate::{actor::*, audio::*, consts::*, game::*, mouse::*};
    pub use bevy::{
        self,
        prelude::{Time, Timer, Vec2},
    };
}
