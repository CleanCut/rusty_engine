use rusty_core::prelude::Vec2;
use serde::{Deserialize, Serialize};

/// Stateful, stack-based button processor.  You can use this to process button state/values and
/// update a `PlayerInput` that you can send to the server.  Also handles the attack button.
pub struct ButtonProcessor {
    horizontal: Vec<ButtonValue>,
    vertical: Vec<ButtonValue>,
    pub direction: Vec2,
}

impl ButtonProcessor {
    /// Create a new `ButtonProcessor`
    pub fn new() -> Self {
        Self {
            horizontal: Vec::new(),
            vertical: Vec::new(),
            direction: Vec2::new(0.0, 0.0),
        }
    }
    /// Process one button, and update the direction vector.
    pub fn process(&mut self, button_value: ButtonValue, button_state: ButtonState) {
        match button_state {
            ButtonState::Pressed => match button_value {
                ButtonValue::Up | ButtonValue::Down => self.vertical.push(button_value),
                ButtonValue::Left | ButtonValue::Right => self.horizontal.push(button_value),
                _ => (),
            },
            ButtonState::Released => match button_value {
                ButtonValue::Up | ButtonValue::Down => self.vertical.retain(|&x| x != button_value),
                ButtonValue::Left | ButtonValue::Right => {
                    self.horizontal.retain(|&x| x != button_value)
                }
                _ => (),
            },
        }
        // Set horizontal movement based on the stack
        if let Some(last_horiz) = self.horizontal.last() {
            match last_horiz {
                ButtonValue::Left => self.direction.x = -1.0,
                ButtonValue::Right => self.direction.x = 1.0,
                _ => {}
            }
        } else {
            self.direction.x = 0.0;
        }
        // Set vertical movement based on the stack
        if let Some(last_vert) = self.vertical.last() {
            match last_vert {
                ButtonValue::Up => self.direction.y = 1.0,
                ButtonValue::Down => self.direction.y = -1.0,
                _ => {}
            }
        } else {
            self.direction.y = 0.0;
        }
        // Normalize
        if self.direction.magnitude() > 0.01 {
            self.direction = self.direction.normalize();
        }
    }
}

impl Default for ButtonProcessor {
    fn default() -> Self {
        ButtonProcessor::new()
    }
}

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
