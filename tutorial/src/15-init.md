# Engine Initialization

Rusty Engine has a prelude which you should import all of in your `main.rs`:

```rust,ignore
use rusty_engine::prelude::*;
```

The Rusty Engine `Game` struct generated in the top level your `main.rs` by the `init` macro. If you don't have a [`GameState`](20-game-state.md) struct, then you can leave the macro call empty and a unit struct `()` will be assumed for your game state:

```rust,ignore
use rusty_engine::prelude::*;

// Call init!() before your main() function
rusty_engine::init!();

fn main() {
    // ...
}
```

If you do have a [`GameState`](20-game-state.md) struct (which we'll go over in the [game state section](20-game-state.md)), then you need to pass that type to the `init` macro:

```rust,ignore
use rusty_engine::prelude::*;

struct GameState {
    number: i32,
}

rusty_engine::init!(GameState); // There's a game state section later in the tutorial!

fn main() {
    // ...
}
```

Once you have initialized the engine, you should create a new `Game` struct in your `main` function and assign it to a mutable variable, usually called `game`.

```rust,ignore
fn main() {
    let mut game = Game::new();
    // ...
```

At this point you will use your `Game` instance to set up your game and register logic functions to run each frame.

Finally, at the end of main you will run your `Game` with `Game::run()`. The `run` method takes an initial game state. If you didn't provide a game state struct type to the `init` macro, then your initial game state is a unit struct `()`:

```rust,ignore
fn main() {
    // ...
    game.run(());
}
```

If you did provide a game state struct type to the `init` macro, then pass `run` a value of that type:

```rust,ignore
fn main() {
    // ...
    game.run(GameState { number: 42 });
}
```

## Example

If you put it all together, it looks like this. This example will run and create an empty window, but won't _do_ anything, yet.

```rust,ignore
use rusty_engine::prelude::*;

struct GameState {
    number: i32,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();

    // get your game stuff ready here

    game.run(GameState { number: 42 });
}
```
