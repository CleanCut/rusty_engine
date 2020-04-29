use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// A color with 32-bit float parts from `[0.0, 1.0]` suitable for OpenGL.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Color([f32; 3]);

impl Color {
    /// Red, Green, Blue! Values should be in the range `[0.0, 1.0]`
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b])
    }
}

/// So converting back and forth between `Color` and `[f32; 3]` is easy.
impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0[0] as u32).hash(state);
        (self.0[1] as u32).hash(state);
        (self.0[2] as u32).hash(state);
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.0[0] == other.0[0]) && (self.0[1] == other.0[1]) && (self.0[2] == other.0[2])
    }
}
