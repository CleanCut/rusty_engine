use crate::prelude::GameState;
use bevy::prelude::{EventReader, IntoSystem, Plugin, ResMut};

// Re-export some Bevy types to use
pub use bevy::input::keyboard::{KeyCode, KeyboardInput};

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(sync_keyboard_input.system());
    }
}

fn sync_keyboard_input(
    mut game_state: ResMut<GameState>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // Clear any events that weren't used last frame
    game_state.keyboard_events.clear();

    // Populate this frame's events
    for event in keyboard_input_events.iter() {
        game_state.keyboard_events.push(event.clone());
    }
}
