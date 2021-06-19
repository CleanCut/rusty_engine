use crate::prelude::GameState;
use bevy::prelude::*;

// Re-export some Bevy types to use
pub use bevy::{
    input::{
        mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel},
        ElementState,
    },
    window::CursorMoved,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(sync_mouse_input.system().before("game_logic_sync"));
    }
}

fn sync_mouse_input(
    mut game_state: ResMut<GameState>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    // Clear any events that weren't used last frame
    game_state.mouse_button_events.clear();
    game_state.cursor_moved_events.clear();
    game_state.mouse_motion_events.clear();
    game_state.mouse_wheel_events.clear();

    // Populate this frame's events
    for ev in mouse_button_events.iter() {
        game_state.mouse_button_events.push(ev.clone());
    }
    for ev in cursor_moved_events.iter() {
        let mut new_event = ev.clone();
        // Convert from screen space to game space
        new_event.position -= game_state.screen_dimensions * 0.5;
        game_state.cursor_moved_events.push(new_event);
    }
    for ev in mouse_motion_events.iter() {
        game_state.mouse_motion_events.push(ev.clone());
    }
    for ev in mouse_wheel_events.iter() {
        game_state.mouse_wheel_events.push(ev.clone());
    }
}
