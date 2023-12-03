# Timer

Rusty Engine re-export's Bevy's [`Timer`](https://docs.rs/rusty_engine/latest/rusty_engine/prelude/struct.Timer.html) struct. Please see the [`Timer` API documentation](https://docs.rs/rusty_engine/latest/rusty_engine/prelude/struct.Timer.html) for full details. Below, is a quick introduction to the most vital parts.

Timers are super cheap, performance-wise. Feel free to create them and throw them away as much as you like.

### Creation

It is easy to create a timer with the `from_seconds` method. The first parameter is a number of seconds to countdown, and the second parameter is whether or not the timer is repeating. `TimerMode::Once` means the timer will only countdown once, and then remain at `0.0` once finished. `TimerMode::Repeating` means the timer will start counting down again from the same countdown time as soon as it has reached `0.0`.

```rust,ignored
// A one-shot timer.
let timer_once = Timer::from_seconds(10.0, TimerMode::Once);

// A repeating timer.
let timer_repeating = Timer::from_seconds(3.0, TimerMode::Repeating);
```

### Counting down & Finishing

Timers must be ticked with the `tick` method to make them actually count down the time. The `tick` method takes a `Duration` of time that has gone by, which is exactly what `Engine.delta` is for.  `tick` returns an immutable reference to the timer, so you can chain a method call to `finished` or `just_finished`.

```rust,ignored
if timer_once.tick(engine.delta).just_finished() {
    // the one-shot timer just finished, do the thing
}

if timer_repeating.tick(engine.delta).just_finished() {
    // the repeating timer finished (again), do the thing (again)
    // the timer has already begun counting down from the top again at this point
}
```

If you don't tick a timer, it is effectively paused.
