# Game Logic Function

A game is divided up into _frames_. A _frame_ is one run through your game logic to produce a new image to display on the screen. Commonly, you will get about 60 frames each second.  Each frame, Rusty Engine tries to run your game logic function(s).

A game logic function definition looks like this:

```rust,ignore
fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    // your actual game logic goes here
}
```

The function may be named anything you want. We used `game_logic` in the example above, which is the conventional name if you only have one. However, if you use more than one game logic function, each will need to have a unique name.

Your `game_state` parameter should be a mutable reference to your game state struct type. The example above is for a game state struct literally named `GameState`. If you did not provide a game state type, then you will need to use a unit struct `()` instead:

```rust,ignore
game_state: &mut ()
```

Rusty Engine will attempt to run all of your game logic functions that it knows about each frame. You need to "add" your game logic functions by calling `Game::add_logic` in your `main` function:

```rust,ignore
game.add_logic(game_logic);
```

You can add multiple game logic functions, which will always run in the order they were added. For example, this game will always run the `menu_logic` function first, and then the `game_logic`.

```rust,ignore
game.add_logic(menu_logic);
game.add_logic(game_logic);
```

After each logic function is run, Rusty Engine checks the return value to see if it should continue with the next logic function, or skip the rest. Game logic functions return a `bool`. If the return value is `true`, that means "keep on going!" and run the next logic function. If the return value is `false`, then Rusty Engine skips the rest of the logic functions this frame.  This is so, for example, if you don't want your game logic to run while you are in a menu, then your menu logic can return `false` and the game logic simply won't run.

Most of the time you want the next function to run, so most game logic functions end like this:

```rust,ignore
    // ...
    true
}
```

The return value of the last game logic function in the chain is ignored.

## Example

Here's an example game logic function using the game state from the [game state section](20-game-state.md). It simply increments the score and outputs that score to the console once per frame.

```rust,ignore
use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();
    let game_state = GameState {
        high_score: 2345,
        current_score: 0,
        enemy_labels: Vec::new(),
        spawn_timer: Timer::from_seconds(10.0, false),
    };
    game.add_logic(game_logic); // Don't forget to add the logic function to the game!
    game.run(game_state);
}

fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    game_state.current_score += 1;
    println!("Current score: {}", game_state.current_score);
    true
}
```
