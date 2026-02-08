//! Facilities for dealing with keyboard input

use crate::prelude::Engine;
use bevy::{platform::collections::HashMap, prelude::*};

// Re-export some Bevy types to use
pub use bevy::input::keyboard::{KeyCode, KeyboardInput};

pub(crate) struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource::<KeyboardState>(KeyboardState::default())
            .add_systems(Update, (sync_keyboard_events, sync_keyboard_state));
    }
}

/// Sync any keyboard events to the engine
fn sync_keyboard_events(
    mut engine: ResMut<Engine>,
    mut keyboard_input_events: MessageReader<KeyboardInput>,
) {
    // Clear any events that weren't used last frame
    engine.keyboard_events.clear();

    // Populate this frame's events
    for event in keyboard_input_events.read() {
        engine.keyboard_events.push(event.clone());
    }
}

pub struct KeyboardStateChain(KeyboardState);

impl KeyboardStateChain {
    /// Calls the closure if a key is currently pressed
    #[inline]
    pub fn pressed(&self, key: KeyCode, mut then: impl FnMut(&KeyboardState)) -> &Self {
        if self.0.pressed(key) {
            then(&self.0);
        }
        self
    }
    /// Calls the closure if any of the keys are currently pressed
    #[inline]
    pub fn pressed_any(
        &self,
        key_codes: &[KeyCode],
        mut then: impl FnMut(&KeyboardState),
    ) -> &Self {
        if self.0.pressed_any(key_codes) {
            then(&self.0);
        }
        self
    }
    /// Calls the closure if a key started being pressed this frame
    #[inline]
    pub fn just_pressed(&self, key: KeyCode, mut then: impl FnMut(&KeyboardState)) -> &Self {
        if self.0.just_pressed(key) {
            then(&self.0);
        }
        self
    }
    /// Calls the closure if any of the indicated keys started being pressed this frame
    #[inline]
    pub fn just_pressed_any(
        &self,
        key_codes: &[KeyCode],
        mut then: impl FnMut(&KeyboardState),
    ) -> &Self {
        if self.0.just_pressed_any(key_codes) {
            then(&self.0);
        }
        self
    }
    /// Calls the closure if a key started being released this frame
    #[inline]
    pub fn just_released(&self, key: KeyCode, mut then: impl FnMut(&KeyboardState)) -> &Self {
        if self.0.just_released(key) {
            then(&self.0);
        }
        self
    }
    /// Calls the closure if any of the indicated keys started being released this frame
    #[inline]
    pub fn just_released_any(
        &self,
        key_codes: &[KeyCode],
        mut then: impl FnMut(&KeyboardState),
    ) -> &Self {
        if self.0.just_released_any(key_codes) {
            then(&self.0);
        }
        self
    }
}

/// Represents the end-state of all keys during the last frame. Access it through
/// [`Engine.keyboard_state`](crate::prelude::Engine) in your game logic function.
#[derive(Clone, Debug, Default, Resource)]
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
    pub fn chain(&self) -> KeyboardStateChain {
        KeyboardStateChain(self.clone())
    }
}

/// store bevy's keyboard state for our own use
fn sync_keyboard_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
const KEYCODEVARIANTS: [KeyCode; 194] = [
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
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
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
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
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
];
