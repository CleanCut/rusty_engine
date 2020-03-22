pub use rusty_audio as audio;
pub use rusty_gfx as gfx;
pub use rusty_core as core;

// Deeper re-exports
pub use rusty_core::nalgebra_glm;

pub mod prelude {
}

#[cfg(test)]
mod tests {
    #[test]
    fn rusty_engine_works() {
        assert_eq!(2 + 2, 4);
    }
}