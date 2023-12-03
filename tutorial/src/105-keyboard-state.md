# Keyboard State

You can think of keyboard _state_ as a snapshot of exactly which keys are pressed (or not) at the start of the frame. Keyboard state is best for interactive things like character movement.  If you need to process every single keystroke (like when entering text), check out the [Keyboard Event](110-keyboard-events.md) section instead.

The `Engine` struct's `keyboard_state` field is a struct through which you query the state of the key(s) you are interested in.

Rusty Engine exposes [Bevy](https://bevyengine.org/)'s [`KeyCode`](https://docs.rs/bevy/latest/bevy/input/keyboard/enum.KeyCode.html) enum through its prelude. See [the `KeyCode` documentation](https://docs.rs/bevy/latest/bevy/input/keyboard/enum.KeyCode.html) for all the possible key variants.

### Pressed / Released

Use the `pressed` method see if a single key is currently pressed or not:

```rust,ignored
if engine.keyboard_state.pressed(KeyCode::Enter) {
    // do stuff every frame that the key is still pressed 
}
```

If a key is _not_ pressed, then it is released, so there is no dedicated method to check if a key is released. Just negate the condition by putting a `!` before the method call.

### Just Pressed / Just Released

The `just_pressed` method will let you know if the key was pressed for the first time _this_ frame, which is useful for triggering actions that you only want to happen once per keypress.

```rust,ignored
if engine.keyboard_state.just_pressed(KeyCode::Escape) {
    // do a thing when the key has just been pressed
}
```

Since "just pressed" and "just released" are not logical opposites, there is also a `just_released` method. This returns `true` if the key was previously in a pressed state and was just released this frame.

```rust,ignored
if engine.keyboard_state.just_released(KeyCode::W) {
    // do a thing when the key has just been released
}
```

### Handling Multiple Keys

There is an `*_any` method for each of the three single key methods that does the same thing, but considering multiple keys at a time. This is especially helpful if you want to, e.g. treat WASD and arrow keys identically.

- `pressed` -> `pressed_any`
- `just_pressed` -> `just_pressed_any`
- `just_released` -> `just_released_any`

Instead of passing a single `KeyCode` to these methods, you pass a slice containing all of the key codes you care about:

```rust,ignored
if engine.keyboard_state.pressed_any(&[KeyCode::W, KeyCode::Up]) {
    // player moves upward
}
if engine.keyboard_state.just_pressed_any(&[KeyCode::Q, KeyCode::F1]) {
    // open menu
}
if engine.keyboard_state.just_released_any(&[KeyCode::Space, KeyCode::LControl]) {
    // re-evaluate your life choices
}
```

