use bevy::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Collider {
    pub topleft: Vec2,
    pub bottomright: Vec2,
}

impl Collider {
    pub fn new(tlx: f32, tly: f32, brx: f32, bry: f32) -> Self {
        Self {
            topleft: Vec2::new(tlx, tly),
            bottomright: Vec2::new(brx, bry),
        }
    }
}
