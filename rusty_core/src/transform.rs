use crate::prelude::glm::{self, Vec2};
use std::convert::TryInto;

pub type Direction = f32;
pub type Scale = f32;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub direction: Direction,
    pub scale: Scale,
    affine: [[f32; 4]; 4],
    affine_cache: (Vec2, Direction, Scale),
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn at(pos: Vec2, direction: Direction, scale: Scale) -> Self {
        Self {
            pos,
            direction,
            scale,
            ..Self::default()
        }
    }
    pub fn get_affine(&mut self) -> [[f32; 4]; 4] {
        if self.affine_cache != (self.pos, self.direction, self.scale) {
            let translated = glm::translation(&glm::vec2_to_vec3(&self.pos));
            let rotated = glm::rotate(&translated, self.direction, &glm::vec3(0.0f32, 0., 1.));
            let scaled = glm::scale(&rotated, &glm::vec3(self.scale, self.scale, 0.));
            self.affine_cache = (self.pos, self.direction, self.scale);
            self.affine = scaled.try_into().unwrap();
        }
        self.affine
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            direction: 0.0,
            scale: 1.0,
            affine: [
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
            ],
            affine_cache: (
                Vec2::new(std::f32::NAN, std::f32::NAN),
                std::f32::NAN,
                std::f32::NAN,
            ),
        }
    }
}
