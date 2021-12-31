# Text Creation

Text creation is quite similar to sprite creation. You create text through the [`EngineState.add_text`](400-engine-state.md) method.  Since `Game` implements `DerefMut<EngineState>`, you can also call all of `EngineState`'s creation methods through `Game` in your `main()` function. In either case, it looks something like this when you create text:

```rust,ignored
// Through your `Game` in `main()`
let _ = game.add_text("title", "The Fun Game");

// Or later in a game logic function through the `EngineState`
let _ = engine_state.add_text("score", "Score: 0");
```

The first parameter is a unique label. It is used in the same way as sprite labels are used (to identify the text later on). The second parameter is actual string value to render.

`add_text` returns a mutable reference to a `Text` (`&mut Text`). Note that this is one case where Rusty Engine does _not_ re-export something from Bevy. Bevy has also has a struct named `Text`, but it is entirely a different thing.

Since it will emit a warning to silently ignore the mutable reference to the `Text`, you should explicitly ignore it if you are not going to use it by doing `let _ = ...` as in the examples above. However, most of the time you will want to use the mutable reference to immediately adjust your text, as we'll see in the following sections.

