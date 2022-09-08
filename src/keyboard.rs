//! Facilities for dealing with keyboard input

use crate::prelude::Engine;
use bevy::{prelude::*, utils::HashMap};

// Re-export some Bevy types to use
pub use bevy::input::keyboard::{KeyCode, KeyboardInput};

pub(crate) struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource::<KeyboardState>(KeyboardState::default())
            .add_system(sync_keyboard_events.before("game_logic_sync"))
            .add_system(sync_keyboard_state.before("game_logic_sync"));
    }
}

/// Sync any keyboard events to the engine
fn sync_keyboard_events(
    mut engine: ResMut<Engine>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // Clear any events that weren't used last frame
    engine.keyboard_events.clear();

    // Populate this frame's events
    for event in keyboard_input_events.iter() {
        engine.keyboard_events.push(event.clone());
    }
}

/// Represents the end-state of all keys during the last frame. Access it through
/// [`Engine.keyboard_state`](crate::prelude::Engine) in your game logic function.
#[derive(Clone, Debug, Default)]
pub struct KeyboardState {
    this_frame: HashMap<KeyCode, bool>,
    last_frame: HashMap<KeyCode, bool>,
}

impl KeyboardState {
    /// Returns true if a key is currently pressed
    pub fn pressed(&self, key: KeyCode) -> bool {
        *self.this_frame.get(&key).unwrap_or(&false)
    }
    /// Returns true if any of the keys are currently pressed
    pub fn pressed_any(&self, key_codes: &[KeyCode]) -> bool {
        key_codes.iter().any(|k| self.pressed(*k))
    }
    /// Returns true if a key started being pressed this frame
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        *self.this_frame.get(&key).unwrap_or(&false)
            && !*self.last_frame.get(&key).unwrap_or(&false)
    }
    /// Returns true if any of the indicated keys started being pressed this frame
    pub fn just_pressed_any(&self, key_codes: &[KeyCode]) -> bool {
        key_codes.iter().any(|k| self.just_pressed(*k))
    }
    /// Returns true if a key started being released this frame
    pub fn just_released(&self, key: KeyCode) -> bool {
        !*self.this_frame.get(&key).unwrap_or(&false)
            && *self.last_frame.get(&key).unwrap_or(&false)
    }
    /// Returns true if any of the indicated keys started being released this frame
    pub fn just_released_any(&self, key_codes: &[KeyCode]) -> bool {
        key_codes.iter().any(|k| self.just_released(*k))
    }
    // -----------------------------------
    /// Calls the closure if a key is currently pressed
    #[inline]
    pub fn pressed_do(&self, key: KeyCode, mut f: impl FnMut()) -> &Self {
        if self.pressed(key) {
            f();
        }
        self
    }
    /// Calls the closure if any of the keys are currently pressed
    #[inline]
    pub fn pressed_any_do(&self, key_codes: &[KeyCode], mut f: impl FnMut()) -> &Self {
        if self.pressed_any(key_codes) {
            f();
        }
        self
    }
    /// Calls the closure if a key started being pressed this frame
    #[inline]
    pub fn just_pressed_do(&self, key: KeyCode, mut f: impl FnMut()) -> &Self {
        if self.just_pressed(key) {
            f();
        }
        self
    }
    /// Calls the closure if any of the indicated keys started being pressed this frame
    #[inline]
    pub fn just_pressed_any_do(&self, key_codes: &[KeyCode], mut f: impl FnMut()) -> &Self {
        if self.just_pressed_any(key_codes) {
            f();
        }
        self
    }
    /// Calls the closure if a key started being released this frame
    #[inline]
    pub fn just_released_do(&self, key: KeyCode, mut f: impl FnMut()) -> &Self {
        if self.just_released(key) {
            f();
        }
        self
    }
    /// Calls the closure if any of the indicated keys started being released this frame
    #[inline]
    pub fn just_released_any_do(&self, key_codes: &[KeyCode], mut f: impl FnMut()) -> &Self {
        if self.just_released_any(key_codes) {
            f();
        }
        self
    }
}

/// store bevy's keyboard state for our own use
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
