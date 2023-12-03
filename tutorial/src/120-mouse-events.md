# Mouse Events

Every movement of the mouse, click of a mouse button, or scrolling tick of a scroll wheel generates a mouse event. All of the mouse events are stored into a set of vectors on `Engine` that can be examined. At the end of each frame, any unprocessed events are thrown away.

Mouse events are most useful when you want to process multiple events that happened within a single frame, such as processing all of the points that a mouse traversed, or all of the mousewheel clicks that happened in a single frame.

### Mouse button events

You usually want to use [mouse state](115-mouse-state.md) for mouse buttons, which are less awkward to deal with than mouse events when you only care about the state the mouse ended up in at the end of the frame. Mouse events are available in the `Engine` struct's `mouse_button_events` field, which is a vector of mouse button input events. The Bevy struct [`MouseButtonInput`](https://docs.rs/rusty_engine/latest/rusty_engine/mouse/struct.MouseButtonInput.html) is used for the event value.  Here is an example of using mouse button events to rotate a sprite by a fixed amount for each click. This is guaranteed not to miss any clicks in the (unlikely) event that two clicks come in on the same frame.


```rust,ignored
for mouse_button_input in &engine.mouse_button_events {
    if mouse_button_input.state != ButtonState::Pressed {
        break;
    }
    match mouse_button_input.button {
        MouseButton::Left => sprite.rotation += std::f32::consts::FRAC_PI_4,
        MouseButton::Right => sprite.rotation -= std::f32::consts::FRAC_PI_4,
        _ => {}
    }
}
```

### Mouse location events

Mouse location events are most useful if you are trying to capture all the points the mouse was at during the frame. Unlike mouse button events, there are *often* multiple mouse location events, since moving the mouse produces a series of events for each location that the mouse cursor is rendered on screen. If you only care about the final location of the mouse during the frame, you should use [mouse state](115-mouse-state.md) instead.

Mouse location events are accessed through the `Engine.mouse_location_events` vector and contain the [`CursorMoved`](https://docs.rs/rusty_engine/latest/rusty_engine/mouse/struct.CursorMoved.html) struct re-exported from Bevy. If you want to draw a trail of sparkles wherever a mouse went, mouse location events might be a good source of data:

```rust,ignored
for cursor_moved in &engine.mouse_location_events {
    // draw sparkles at cursor_moved.position
}
```

### Mouse motion events

Each location event has a corresponding motion event which reports the _relative_ motion of the mouse, rather than the absolute location.  Mouse motion events are accessed through the `Engine.mouse_motion_events` vector and contain the [`MouseMotion`](https://docs.rs/rusty_engine/latest/rusty_engine/mouse/struct.MouseMotion.html) struct re-exported from Bevy.

```rust,ignored
for mouse_motion in &engine.state.mouse_motion_events {
    // do something with mouse_motion.delta
}
```

### Mouse wheel events

As the mouse wheel tends to produce multiple events in a single frame, mouse wheel events may tend to be more useful than the mouse wheel state. Mouse wheel events are accessed through the `Engine.mouse_wheel_events` vector and contain the [`MouseWheel`](https://docs.rs/rusty_engine/latest/rusty_engine/mouse/struct.MouseWheel.html) struct re-exported from Bevy. Here's an example of using the mouse wheel to scale the size of a sprite up or down. The `y` field represents the turning of the wheel. The `x` field represents sideways tilting motion for mouse wheels that support it.

```rust,ignored
for mouse_wheel in &engine.mouse_wheel_events {
    sprite.scale *= 1.0 + (0.05 * mouse_wheel.y);
}
```
