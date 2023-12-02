# Game State

You will need somewhere to store data for your game between frames. That somewhere is your _game state struct_.  You provide a struct to use for your own game state. Within that struct, you can store just about anything. Some examples of things you may want to put in your game state:

- Player attributes (name, color, health, energy, money, etc.)
- Game attributes (score, day, turn, etc.)
- Timers for animation, spawning events, etc.
- Collections of sprite labels to iterate through (perhaps a vector of labels of all the enemy sprites, or a vector of widgets to animate, or...whatever)
- Collections of text labels to iterate through and update (maybe text representing current health is placed above your player)
- Anything else that needs to persist across frames that isn't already stored in the engine

You can name your game state struct whatever you want, but since there can only ever be one game state type, it probably makes sense just to name it `GameState` as we will in this tutorial.

You must always include the line `#[derive(Resource)]` immediately before the `GameState`. This is a requirement from the Bevy game engine which we're using under-the-hood.

Here is an example of a game state you might define for a simple game which keeps track of a current score and a high score, the labels of enemies which need to move around, and a timer for when to spawn a new enemy.

```rust,ignore
#[derive(Resource)]
struct GameState {
    current_score: u32,
    high_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}
```

When you start your game, you will need to pass it an initial value of the game state.  You will receive a mutable reference to this game state value in your [game logic function](25-game-logic-function.md) each frame.

```rust,ignore
fn main() {
    // ...
    let game_state = GameState {
        current_score: 0,
        high_score: 2345,
        enemy_labels: Vec::new(),
        spawn_timer: Timer::from_seconds(10.0, TimerMode::Once),
    };
    game.run(game_state);
}
```

