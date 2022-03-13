# Engine

The `Engine` struct is central to Rusty Engine and has already shown up in many places in this tutorial.  It is highly recommended to read through all of the [`Engine` API documentation](https://docs.rs/rusty_engine/latest/rusty_engine/game/struct.Engine.html).

Here are a few other tidbits that are worth calling out:

- `should_exit` - a `bool` field you can set to `true` to cause Rusty Engine to cleanly exit at the end of the frame.
- `delta` - the duration of the previous frame as a `Duration`. This should be used for ticking any `Timer`s.
- `delta_f32` - the duration of the previous frame as an `f32`. This should be used to produce smooth animation. For example, if you define a movement speed in `pixels per second` such as `const MOVE_SPEED: f32 = 50.0`, then you can use it to actually move a sprite at that speed by multiplying it by `delta_f32` like this: `sprite.translation.x += MOVE_SPEED * engine.delta_f32`
- `time_since_startup` - the duration since the start of the program as a `Duration`
- `time_since_startup_f64` - the duration since the start of the program as an `f64`. This needs to be a 64-bit float because it would be easy for an `f32` to reach a number high enough to be low precision. If you want to do math with this number, you should do the math with `f64`'s, and then convert it to an `f32` at the very end.
- `window_dimensions` - a `Vec2` describing the width and height of the window in pixels. Since `(0.0, 0.0)` is the center of the screen, the edges of the screen are +/- `window_dimensions / 2.0`.

...for the rest of the fields (and methods), see the [`Engine` API documentation](https://docs.rs/rusty_engine/latest/rusty_engine/game/struct.Engine.html)
