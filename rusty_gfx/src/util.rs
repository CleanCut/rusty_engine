use rand::prelude::Rng;
use rusty_core::glm::Vec2;

/// If the length of a vector is longer than `magnitude`, scale the vector's length to equal
/// `magnitude`
pub fn clamp_vec_to_magnitude(v: &mut Vec2, magnitude: f32) {
    if v.magnitude() > magnitude {
        v.data = (v.normalize() * magnitude).data;
    }
}

/// Compute the direction (angle in radians, where 0 is in the positive x direction) from v1 to v2
pub fn angle_facing(v1: &Vec2, v2: &Vec2) -> f32 {
    (v2.data[1] - v1.data[1]).atan2(v2.data[0] - v1.data[0])
}

/// Given a length of a side of a square centered at (0.0, 0.0), return a Vec2 that resides
/// somewhere within it.
pub fn rand_in_square<T: Rng>(dimension: f32, rng: &mut T) -> Vec2 {
    Vec2::new(
        rng.gen_range(-dimension, dimension),
        rng.gen_range(-dimension, dimension),
    )
}
