# Game State

You will need somewhere to store data for your game that is not part of the engine but that you need access to for more than a single frame. That somewhere is your _game state struct_.  You provide a struct to use for your own game state. Within that struct, you can store just about anything. Some examples of things you may want to put in your game state:

- Player attributes (name, color, health, energy, money, etc.)
- Game attributes (score, day, turn, etc.)
- Timers for animation, spawning events, etc.
- Collections of sprite labels to iterate through (perhaps a vector of labels of all the enemy sprites, or a vector of widgets to animate, or...whatever)
- Collections of text labels to iterate through and update (maybe text representing current health is placed above your player)
- Anything else that needs to persist across frames that isn't already stored in the engine

You can name your game state struct whatever you want, but since there can only ever be one game state type, we suggest you call it `GameState` as we will in this tutorial.

Here is an example of a game state you might define for a simple game which keeps track of a current score and a high score, the labels of enemies which need to move around, and a timer for when to spawn a new enemy.

```rust,ignore
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}
```

When you start your game, you will need to pass it an initial value of the game state.  You will receive a mutable reference to this game state value in your logic function(s) each frame.

```rust,ignore
fn main() {
    // ...
    let game_state = GameState {
        high_score: 2345,
        current_score: 0,
        enemy_labels: Vec::new(),
        spawn_timer: Timer::from_seconds(10.0, false),
    };
    game.run(game_state);
}
```

