pub use nalgebra_glm as glm;

pub mod prelude {
    pub use nalgebra_glm::{self as glm, Vec2};
}

#[cfg(test)]
mod tests {
    #[test]
    fn rusty_core_works() {
        assert_eq!(2 + 2, 4);
    }
}
