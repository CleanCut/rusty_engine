use crate::prelude::GameState;
use bevy::{prelude::*, utils::HashMap};

// Re-export some Bevy types to use
pub use bevy::input::keyboard::{KeyCode, KeyboardInput};

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource::<KeyboardState>(KeyboardState::default())
            .add_system(sync_keyboard_events.system().before("game_logic_sync"))
            .add_system(sync_keyboard_state.system().before("game_logic_sync"));
    }
}

fn sync_keyboard_events(
    mut game_state: ResMut<GameState>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // Clear any events that weren't used last frame
    game_state.keyboard_input_events.clear();

    // Populate this frame's events
    for event in keyboard_input_events.iter() {
        game_state.keyboard_input_events.push(event.clone());
    }
}

/// Represents the state (pressed or not) of all keys during this frame.
#[derive(Clone, Debug, Default)]
pub struct KeyboardState {
    this_frame: HashMap<KeyCode, bool>,
    last_frame: HashMap<KeyCode, bool>,
}

impl KeyboardState {
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        *self.this_frame.get(&key).unwrap_or(&false)
    }
    pub fn any_pressed(&self, key_codes: &[KeyCode]) -> bool {
        key_codes.iter().any(|k| self.is_pressed(*k))
    }
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        *self.this_frame.get(&key).unwrap() && !*self.last_frame.get(&key).unwrap()
    }
    pub fn just_released(&self, key: KeyCode) -> bool {
        !*self.this_frame.get(&key).unwrap() && *self.last_frame.get(&key).unwrap()
    }
}

fn sync_keyboard_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut keyboard_state: ResMut<KeyboardState>,
) {
    keyboard_state.last_frame = keyboard_state.this_frame.clone();
    for keycode in KEYCODEVARIANTS {
        let this_key = keyboard_state.this_frame.entry(keycode).or_insert(false);
        *this_key = keyboard_input.pressed(keycode);
    }
    // The very first frame, pretend the frame before was the same
    if keyboard_state.last_frame.is_empty() {
        keyboard_state.last_frame = keyboard_state.this_frame.clone();
    }
}

use KeyCode::*;
const KEYCODEVARIANTS: [KeyCode; 163] = [
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    Plus,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    Oem102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
];
