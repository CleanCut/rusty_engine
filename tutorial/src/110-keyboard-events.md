# Keyboard Events

Keyboard events are what your operating system passes to text input boxes. If you go to a text box in a browser and hold down the space bar, you'll typically see one space, a short pause, and then several spaces come out faster after that. Those are keyboard events. You typically _only_ want keyboard events if you are trying to capture sequences of keypresses as if they are text. For things like character movement and button presses where you only care about the final state of the keyboard each frame, you should check out the [Keyboard State](105-keyboard-state.md) section instead.

Keyboard events are passed through from Bevy as instances of the [`KeyboardInput`](https://docs.rs/rusty_engine/latest/rusty_engine/keyboard/struct.KeyboardInput.html) struct. Here is an example of processing keyboard events to adjust the position of a sprite:

```rust,ignored
for keyboard_event in game_state.keyboard_events.drain(..) {
    // We're using `if let` and a pattern to destructure the KeyboardInput struct and only look at
    // keyboard input if the state is "Pressed". Then we match on the KeyCode and take action.
    if let KeyboardInput {
        scan_code: _,
        key_code: Some(key_code),
        state: ButtonState::Pressed,
    } = keyboard_event
    {
        match key_code {
            KeyCode::W | KeyCode::Up => race_car.translation.y += 10.0,
            KeyCode::A | KeyCode::Left => race_car.translation.x -= 10.0,
            KeyCode::S | KeyCode::Down => race_car.translation.y -= 10.0,
            KeyCode::D | KeyCode::Right => race_car.translation.x += 10.0,
            _ => {}
        }
    }
}
```

