# Engine Initialization

Rusty Engine has a prelude which you should import in `main.rs`:

```rust,ignore
use rusty_engine::prelude::*;
```

You should usually define a [`GameState`](20-game-state.md) struct (which we'll go over in the [game state section](20-game-state.md)). This will be a struct that you store your game-specific data in. Things like high score, player name, health left, etc.

```rust,ignore
use rusty_engine::prelude::*;

struct GameState {
    health_left: i32,
}

fn main() {
    // ...
}
```

Create a new `Game` struct in your `main` function and assign it to a mutable variable, usually called `game`.

```rust,ignore
fn main() {
    let mut game = Game::new();
    // ...
```

Use your `Game` instance to set up your game and register logic functions to run each frame.

At the end of main you will run your game with `Game::run()`. The `run` method takes an initial game state. If you didn't define a game state struct, then just give it a unit struct `()`:

```rust,ignore
fn main() {
    // ...
    game.run(());
}
```

If you did define a game state struct (we'll assume you named it `GameState`), then pass `run` a value of that type:

```rust,ignore
fn main() {
    // ...
    game.run(GameState { health_left: 42 });
}
```

## Example

If you put it all together, it looks like this. This example will run and create an empty window, but won't _do_ anything, yet.

```rust,ignore
use rusty_engine::prelude::*;

struct GameState {
    health_left: i32,
}

fn main() {
    let mut game = Game::new();

    // get your game stuff ready here

    game.run(GameState { health_left: 42 });
}
```
