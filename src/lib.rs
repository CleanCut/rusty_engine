pub use rusty_audio as audio;
pub use rusty_core as core;
pub use rusty_gfx as gfx;

// rusty_core's re-exports should all be attached to the root of rusty_engine
pub use rusty_core::glm;

pub mod prelude {
    pub use glm::{self, Vec2};
    pub use rusty_audio::prelude::*;
    pub use rusty_core::prelude::*;
    pub use rusty_gfx::prelude::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn rusty_engine_works() {
        assert_eq!(2 + 2, 4);
    }
}
