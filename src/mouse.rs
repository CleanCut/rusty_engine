use bevy::{
    math::Vec2,
    prelude::{EventReader, IntoSystem, Plugin, ResMut},
    window::CursorMoved,
};

// Re-export some Bevy types to use
pub use bevy::input::{
    mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel},
    ElementState,
};

use crate::prelude::GameState;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(sync_mouse_input.system());
    }
}

#[derive(Debug, Default)]
pub struct MouseEvents {
    button_events: Vec<MouseButtonInput>,
    cursor_moved_events: Vec<CursorMoved>,
    mouse_motion_events: Vec<MouseMotion>,
    mouse_wheel_events: Vec<MouseWheel>,
}

impl MouseEvents {
    pub(crate) fn update_button_events(
        &mut self,
        mut mouse_button_input_events: EventReader<MouseButtonInput>,
    ) {
        self.button_events.clear();
        for event in mouse_button_input_events.iter() {
            self.button_events.push(event.clone());
        }
    }
    pub(crate) fn update_cursor_moved_events(
        &mut self,
        mut cursor_moved_events: EventReader<CursorMoved>,
        screen_dimensions: Vec2,
    ) {
        self.cursor_moved_events.clear();
        for event in cursor_moved_events.iter() {
            let mut new_event = event.clone();
            new_event.position.x -= screen_dimensions.x * 0.5;
            new_event.position.y -= screen_dimensions.y * 0.5;
            self.cursor_moved_events.push(new_event);
        }
    }
    pub(crate) fn update_mouse_motion_events(
        &mut self,
        mut mouse_motion_events: EventReader<MouseMotion>,
    ) {
        self.mouse_motion_events.clear();
        for event in mouse_motion_events.iter() {
            self.mouse_motion_events.push(event.clone());
        }
    }
    pub(crate) fn update_mouse_wheel_events(
        &mut self,
        mut mouse_wheel_events: EventReader<MouseWheel>,
    ) {
        self.mouse_wheel_events.clear();
        for event in mouse_wheel_events.iter() {
            self.mouse_wheel_events.push(event.clone());
        }
    }
    /// Mouse button events. Cleared after every frame. Includes information about which
    /// [`MouseButton`] was involved and what the new [`ElementState`] is.
    pub fn button_events(&mut self) -> Vec<MouseButtonInput> {
        let events = self.button_events.clone();
        self.button_events.clear();
        events
    }
    /// Mouse cursor moved events. Cleared after every frame.  These events represent the new
    /// location of the mouse cursor after it has moved.
    pub fn cursor_moved_events(&mut self) -> Vec<CursorMoved> {
        let events = self.cursor_moved_events.clone();
        self.cursor_moved_events.clear();
        events
    }
    /// Mouse motion events. Cleared after every frame. Represents the relative motion (not
    /// location) of the mouse from where it previously was.
    pub fn mouse_motion_events(&mut self) -> Vec<MouseMotion> {
        let events = self.mouse_motion_events.clone();
        self.mouse_motion_events.clear();
        events
    }
    /// Mousewheel events. Cleared after every frame. Represents the movement of the mousewheel.
    pub fn mouse_wheel_events(&mut self) -> Vec<MouseWheel> {
        let events = self.mouse_wheel_events.clone();
        self.mouse_wheel_events.clear();
        events
    }
}

fn sync_mouse_input(
    mut game_state: ResMut<GameState>,
    mouse_button_input_events: EventReader<MouseButtonInput>,
    cursor_moved_events: EventReader<CursorMoved>,
    mouse_motion_events: EventReader<MouseMotion>,
    mouse_wheel_events: EventReader<MouseWheel>,
) {
    game_state
        .mouse_events
        .update_button_events(mouse_button_input_events);
    let screen_dimensions = game_state.screen_dimensions;
    game_state
        .mouse_events
        .update_cursor_moved_events(cursor_moved_events, screen_dimensions);
    game_state
        .mouse_events
        .update_mouse_motion_events(mouse_motion_events);
    game_state
        .mouse_events
        .update_mouse_wheel_events(mouse_wheel_events);
}
