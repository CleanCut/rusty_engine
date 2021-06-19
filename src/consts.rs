use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
// Screen directions
/// The direction representing facing right, in radians.
pub const RIGHT: f32 = 0.0;
/// The direction representing facing left, in radians.
pub const LEFT: f32 = PI;
/// The direction representing facing up, in radians.
pub const UP: f32 = FRAC_PI_2;
/// The direction representing facing down, in radians.
pub const DOWN: f32 = PI + FRAC_PI_2;

// Compass directions
/// The direction representing facing North (up), in radians.
pub const NORTH: f32 = FRAC_PI_2;
/// The direction representing facing Northeast (up and to the right), in radians.
pub const NORTH_EAST: f32 = FRAC_PI_4;
/// The direction representing facing East (right), in radians.
pub const EAST: f32 = 0.0;
/// The direction representing facing Southeast (down and to the right), in radians.
pub const SOUTH_EAST: f32 = PI + FRAC_PI_2 + FRAC_PI_4;
/// The direction representing facing South (down), in radians.
pub const SOUTH: f32 = PI + FRAC_PI_2;
/// The direction representing facing Southwest (down and to the left), in radians.
pub const SOUTH_WEST: f32 = PI + FRAC_PI_4;
/// The direction representing facing West (left), in radians.
pub const WEST: f32 = PI;
/// The direction representing facing Northwest (up and to the left), in radians.
pub const NORTH_WEST: f32 = FRAC_PI_2 + FRAC_PI_4;
