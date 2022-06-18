/// Facilities for dealing with text
use bevy::prelude::{Component, Quat, Transform, Vec2, Vec3};

/// Default depth of the text, positioned so it will be on top of other default layers. Depth
/// can range from `0.0` (back) to `999.0` (front)
pub const TEXT_DEFAULT_LAYER: f32 = 900.0;
/// Default font size for a text.
pub const TEXT_DEFAULT_FONT_SIZE: f32 = 30.0;

/// A [`Text`] is a bit of text that exists on the screen.
#[derive(Clone, Component, Debug, PartialEq)]
pub struct Text {
    /// READONLY: A label to identify the text. This is not the text that is displayed! This is the
    /// label you use to retrieve and modify your text from the
    /// [`Engine::texts`](crate::prelude::Engine::texts) HashMap. This must be *unique* or the game
    /// will crash.
    pub label: String,
    /// SYNCED: The actual text value you want to display.
    pub value: String,
    /// SYNCED: The font to use. Should be a file name of an .otf or .ttf font located within the
    /// assets/ folder somewhere. Defaults to "font/FiraSans-Bold.ttf" (included in the asset pack).
    pub font: String,
    /// SYNCED: The font size of the text you want to display. WARNING: As font sizes get larger,
    /// the sprites we generate for them get slower to create. Very large sizes will crash. The
    /// default font size is `30.0`.
    pub font_size: f32,
    /// SYNCED: Where you are in 2D game space. Positive x is right. Positive y is up. (0.0, 0.0)
    /// is the center of the screen.
    pub translation: Vec2,
    /// SYNCED: Depth of the text. 0.0 (back) to 999.0 (front)  Defaults to [`TEXT_DEFAULT_LAYER`]
    pub layer: f32,
    /// SYNCED: Direction you face in radians. Defaults to [`RIGHT`](crate::RIGHT). See also
    /// the [direction constants](https://docs.rs/rusty_engine/latest/rusty_engine/#constants).
    pub rotation: f32,
    /// SYNCED: `1.0` is the normal 100%.
    pub scale: f32,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            label: String::default(),
            value: String::default(),
            font: "font/FiraSans-Bold.ttf".to_string(),
            font_size: TEXT_DEFAULT_FONT_SIZE,
            translation: Vec2::default(),
            layer: TEXT_DEFAULT_LAYER,
            rotation: f32::default(),
            scale: 1.0,
        }
    }
}

impl Text {
    #[doc(hidden)]
    pub fn bevy_transform(&self) -> Transform {
        let mut transform = Transform::from_translation(self.translation.extend(self.layer));
        transform.rotation = Quat::from_axis_angle(Vec3::Z, self.rotation);
        transform.scale = Vec3::splat(self.scale);
        transform
    }
}
