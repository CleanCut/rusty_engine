# Game Scenarios

Here are some game scenarios that you run through and program using Rusty Engine.

These scenarios are more focused on _building the game_ than on learning how to use Rusty Engine. If you need help understanding how to use Rusty Engine itself, you may want to take a look at the [Rusty Engine Tutorial](https://cleancut.github.io/rusty_engine/) and then come back here.

## Common Setup (Do this first!)

All scenarios assume you've followed the setup steps below and that you are using the default window width and height. If your screen is too small to accomodate a `1280 x 720` window or if you customized the window settings, then you may have to adapt accordingly.

1. Follow the [Configuration](https://cleancut.github.io/rusty_engine/05-config.html) section of the tutorial to set up your `Cargo.toml`
1. Follow the [Asset Pack](https://cleancut.github.io/rusty_engine/10-assets.html) section of the tutorial to download the asset pack to your project.
1. Start with this skeleton `main.rs`:

```rust
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    // game setup goes here

    game.add_logic(game_logic);
    game.run(GameState {});
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
}
```

Run your game in release mode for good performance!

```
cargo run --release
```

## Scenario Legend

Legend:

|Level|Description|
| --- | --- |
| Easy | You will be told each step, and each section includes the code that you should have ended up with, and there is a complete reference project. |
| Medium | You will be told each step, but won't be shown all the code. There might be a reference project. |
| Hard | You will be told what to accomplish, and maybe be given a couple pointers. There is probably no reference project. |
| Insane | You'll need to implement some game engine features yourself |

## Scenarios

- (Easy) [Road Race](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/road_race.md)
- (Medium) [Car Shoot](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/car_shoot.md)
- (Medium) [Driver's Ed](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/extreme_drivers_ed.md)
- (Hard) [Cannon Practice](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/cannon_practice.md)
- (Hard) [Space Invaders](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/space_invaders.md)
- (Insane) [Labrinth](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/labrinth.md) - Rusty Engine doesn't yet provide all the features needed to implement this scenario.
