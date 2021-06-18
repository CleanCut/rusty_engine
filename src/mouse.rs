use bevy::{
    math::Vec2,
    prelude::{EventReader, IntoSystem, Plugin, ResMut},
    window::CursorMoved,
};

// Re-export some Bevy types to use
pub use bevy::input::{
    mouse::{MouseButton, MouseButtonInput},
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
}

impl MouseEvents {
    pub fn update_button_events(
        &mut self,
        mut mouse_button_input_events: EventReader<MouseButtonInput>,
    ) {
        self.button_events.clear();
        for event in mouse_button_input_events.iter() {
            self.button_events.push(event.clone());
        }
    }
    pub fn update_cursor_moved_events(
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
    pub fn button_events(&mut self) -> Vec<MouseButtonInput> {
        let events = self.button_events.clone();
        self.button_events.clear();
        events
    }
    pub fn cursor_moved_events(&mut self) -> Vec<CursorMoved> {
        let events = self.cursor_moved_events.clone();
        self.cursor_moved_events.clear();
        events
    }
}

fn sync_mouse_input(
    mut game_state: ResMut<GameState>,
    mouse_button_input_events: EventReader<MouseButtonInput>,
    cursor_moved_events: EventReader<CursorMoved>,
) {
    game_state
        .mouse_events
        .update_button_events(mouse_button_input_events);
    let screen_dimensions = game_state.screen_dimensions;
    game_state
        .mouse_events
        .update_cursor_moved_events(cursor_moved_events, screen_dimensions);
}
