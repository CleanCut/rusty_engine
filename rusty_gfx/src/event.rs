use rusty_core::prelude::Vec2;
use serde::{Deserialize, Serialize};

/// Abstracted button values you may receive (arrow keys and WASD keys combined into directions, for
/// example)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonValue {
    /// An abstracted button that combines: Arrow Up, W, Comma (Dvorak)
    Up,
    /// An abstracted button that combines: Arrow Down, S, O (Dvorak)
    Down,
    /// An abstracted button that combines: Arrow Left, A
    Left,
    /// An abstracted button that combines: Arrow Right, D, E (Dvorak)
    Right,
    /// An abstracted button that combines: Left Mouse Button, Space Bar, Backspace
    Action1,
    /// An abstracted button that combines: Right Mouse Button, Enter, Return
    Action2,
    /// An abstracted button that combines: Any other Mouse Button, Tab
    Action3,
    /// An abstracted button that combines: =/+ key
    Increase,
    /// An abstracted button that combines: -/_ key
    Decrease,
}

/// Whether a button was pressed or released
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonState {
    /// A button was just pressed
    Pressed,
    /// A button was just released
    Released,
}

/// `GameEvent` represents game events caused by a user, such as the mouse moving around, buttons
/// being pushed, or the window being closed.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum GameEvent {
    /// The user pressed Escape or closed the window. We should quit the game.
    Quit,
    /// Indicates the current position the mouse has moved to.  The mouse is now at this location in
    /// OpenGL coordinates.  Note that on some operating systems this event will fire even if the
    /// cursor is outside the bounds of the window.
    MouseMoved { position: Vec2 },
    /// Indicates that a button with variant `ButtonValue` has been either pressed or released
    /// (variant of `ButtonState`).  Note that both mouse buttons and keyboard buttons are
    /// abstracted and collected together into a few logical game buttons.
    Button {
        button_value: ButtonValue,
        button_state: ButtonState,
    },
}
