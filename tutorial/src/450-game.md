# Game

The `Game` struct exists to get your custom game state injected into Bevy, and to serve as a proxy for `Engine` before the game has started.

Since `Game` implements `DerefMut<Engine>`, any field or method not found on `Game` will be searched for on `Engine` and used if it is found. So, in a sense, `Game` is also the `Engine` while you are setting things up in `main`. However, there are a couple additional things that are unique to `Game`:

### New

The first, and most obvious, difference is that `Game` has a `new` method, as documented in the [Engine Initialization](15-init.md) section. You need to call `new` in your `main` function to create a new game. The variable you assign this value to should be mutable.

```rust,ignored
fn main() {
    let mut game = Game::new();
}
```

### Window Settings

Rusty Engine re-exports the [`Window`](https://docs.rs/rusty_engine/latest/rusty_engine/game/struct.Window.html) struct from Bevy, whose fields are all used to request certain window attributes. Please be aware that these are only _requests_ for configuration, and that the underlying operating system may refuse (or be unable) to give you exactly what you ask for. For example, you may not be able to obtain a window with larger dimensions than the physical monitor.

Pass a `Window` to the `window_settings` method to request specific settings for your game window. This is a great time to take advantage of "struct update" syntax so you don't have to re-specify the fields which you aren't customizing.

```rust,ignored
game.window_settings(Window {
    title: "My Awesome Game".into(),
    width: 800.0,
    height: 200.0,
    ..Default::default()
```

### Adding Game Logic Functions

Game has an `add_logic` method to add game logic functions to your game. Please see the [Engine Initialization](15-init.md) for more details on this method.

### Running the game

The last thing you will do in your main function is to call the `run` method to begin your game. The `run` method takes an instance of whatever game state struct you defined.
