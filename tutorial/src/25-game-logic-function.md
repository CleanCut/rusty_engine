# Game Logic Function

A game is divided up into _frames_. A _frame_ is one run through your game logic to produce a new image to display on the screen. On most hardware you will usually get about 60 frames each second.  Rusty Engine tries to run your game logic function(s) once each frame.

A game logic function definition looks like this:

```rust,ignore
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // your actual game logic goes here
}
```

If you passed in a unit struct for your game state, then use a unit struct in your logic function definition:

```rust,ignore
fn game_logic(engine: &mut Engine, game_state: &mut ()) {
    // logic...without any state to look at.
}
```

The function may be named anything you want. We used `game_logic` in the example above, which is the conventional name if you only have one. However, if you use more than one game logic function, each will need to have a unique name.

You need to "add" your game logic functions to Rusty Engine by calling `Game::add_logic` in your `main` function:

```rust,ignore
game.add_logic(game_logic);
```

You can add multiple game logic functions, which will always run in the order they were added. For example, this game will always run the `menu_logic` function first, and then the `game_logic`.

```rust,ignore
game.add_logic(menu_logic);
game.add_logic(game_logic);
```

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

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    game_state.current_score += 1;
    println!("Current score: {}", game_state.current_score);
}
```
