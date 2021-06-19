use bevy::prelude::*;

/// Default depth of the text actor, positioned so it will be on top of other default layers. Depth
/// can range from `0.0` (back) to `999.0` (front)
pub const TEXT_ACTOR_DEFAULT_LAYER: f32 = 900.0;
pub const TEXT_ACTOR_DEFAULT_FONT_SIZE: f32 = 30.0;

#[derive(Clone, Debug)]
pub struct TextActor {
    /// READONLY: A label to identify the text. This is not the text that is displayed! This is the
    /// label you use to retrieve and modify your text in code.
    pub label: String,
    /// SYNCED: The actual text you want to display.
    pub text: String,
    /// SYNCED: The font size of the text you want to display. WARNING: As font sizes get larger,
    /// the sprites we generate for them get slower to create. Very large sizes will crash.
    pub font_size: f32,
    /// SYNCED: Where you are in 2D game space. Positive x is right. Positive y is up. (0.0, 0.0) is the
    /// center of the screen.
    pub translation: Vec2,
    /// SYNCED: Depth of the text. 0.0 (back) to 999.0 (front)  Defaults to [`TEXT_ACTOR_DEFAULT_LAYER`]
    pub layer: f32,
    /// SYNCED: Direction you face in radians. Defaults to [`RIGHT`](crate::consts::RIGHT). See also
    /// the [direction constants](crate::consts). WARNING: This field will not affect text rotation
    /// until Bevy 0.6 is released and Rusty Engine is updated to use it.
    pub rotation: f32,
    /// SYNCED: 1.0 is the normal 100%. WARNING: This field will not affect text scale
    /// until Bevy 0.6 is released and Rusty Engine is updated to use it.
    pub scale: f32,
}

/// An [`Actor`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all actors.
impl Default for TextActor {
    fn default() -> Self {
        Self {
            label: String::default(),
            text: String::default(),
            font_size: TEXT_ACTOR_DEFAULT_FONT_SIZE,
            translation: Vec2::default(),
            layer: TEXT_ACTOR_DEFAULT_LAYER,
            rotation: f32::default(),
            scale: 1.0,
        }
    }
}

impl TextActor {
    pub(crate) fn bevy_transform(&self) -> Transform {
        let mut transform = Transform::from_translation(self.translation.extend(self.layer));
        transform.rotation = Quat::from_axis_angle(Vec3::Z, self.rotation);
        transform.scale = Vec3::splat(self.scale);
        transform
    }
}
