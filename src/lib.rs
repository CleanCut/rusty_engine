pub mod actor;
pub mod consts;
pub mod game;
pub mod physics;
pub mod preset;

// Public prelude
pub mod prelude {
    pub use crate::{
        actor::Actor,
        consts::*,
        game::{Game, GameState},
        preset::ActorPreset,
    };
    pub use bevy::{
        self,
        prelude::{Time, Timer, Vec2},
    };
}
