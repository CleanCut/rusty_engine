# Mouse State

Everything said about the [Keyboard State](105-keyboard-state.md) is true for Mouse State as well, just for your mouse instead of your keyboard. Mouse state is perfect for character movement or game controls such as buttons. If you need to process every bit of mouse input, such as all the locations the mouse was at since the beginning of the last frame, then you'll need to look at [Mouse Events](120-mouse-events.md) instead.

All mouse state is stored in the `Engine` struct's `mouse_state` field, and queried via methods.

### Mouse Buttons

Mouse button handling closely parallels [keyboard state handling](105-keyboard-state.md), with the same six methods. Only instead of accepting `KeyCode` variants, they accept [`MouseButton`](https://docs.rs/rusty_engine/latest/rusty_engine/mouse/enum.MouseButton.html) variants.

- `pressed` -> `pressed_any`
- `just_pressed` -> `just_pressed_any`
- `just_released` -> `just_released_any`

Rather than repeat the entire discussion for each of the six methods, here's a quick example covering them all:

```rust,ignored
if engine.mouse_state.pressed(MouseButton::Left) {
    // The left mousebutton is currently pressed -- process some continuous movement
}
if engine.mouse_state.just_pressed(MouseButton::Right) {
    // click that button!
}
if engine.mouse_state.just_released(MouseButton::Right) {
    // nope, unclick the button.
}
if engine.mouse_state.pressed_any(&[MouseButton::Left, MouseButton::Right]) {
    // one or more of the main mouse buttons are currently pressed
}
if engine.mouse_state.just_pressed_any(&[MouseButton::Middle, MouseButton::Other(4)]) {
    // the middle button or the 4th button (or both) was just pressed
}
if engine.mouse_state.just_released_any(&[MouseButton::Left, MouseButton::Middle]) {
    // one of those buttons was just released
}
```

### Location

Use the `location` method to see where the mouse is. It returns an `Option<Vec2>`. If `None` is returned, then either the window isn't focused or the mouse pointer isn't in the window. If present, the `Vec2` value is in the same 2D world coordinate system as the rest of the game. See the [section on sprite translation](60-sprite-placement.html) for more info about `Vec2` or the world coordinate system.

A fun way to demonstrate mouse `location` is by having a sprite appear wherever your mouse is located:

```rust,ignored
// `player` is a sprite
if let Some(location) = engine.mouse_state.location() {
    player.translation = location;
}
```

### Motion

The relative motion that the mouse moved last frame is accumulated into a single `Vec2`. This could be useful if you want to base some logic on how fast or in which direction the mouse is moving.

```rust,ignored
let motion = engine.mouse_state.motion();
if motion.length() > 50.0 {
    // mouse is moving pretty fast
}
```

### Mouse Wheel

This represents both the final scrolling (vertical, y) state of the mouse wheel and the final tilt (horizontal, x) state of the mouse wheel. See the [`MouseWheelState` docs](https://docs.rs/rusty_engine/latest/rusty_engine/mouse/struct.MouseWheelState.html) for more info on that.

```rust,ignored
let mouse_wheel_state = engine.mouse_state.wheel();
if mouse_wheel_state.y > 0 {
    // scrolling in one direction...
}
```

