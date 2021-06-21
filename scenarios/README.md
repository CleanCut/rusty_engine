# Game Scenarios

Here are some interesting scenarios that you can use to walk through building a working game prototype using Rusty Engine.  If you would like to contribute your own game scenario idea, please feel free to open a pull request!

## Common Setup!

All scenarios assume you've followed the basic setup below.  All scenarios assume default window width and height. If you [customized the window settings](https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs) then you may have to adapt the scenario accordingly.

1. Create your project. Give it a fun name!
1. Add `rusty_engine` as a dependency in your `Cargo.toml`
  1. Forgot the latest version number?  You can always peek at [the `rusty_engine` page on Crates.io](https://crates.io/crates/rusty_engine)
  1. It's recommended to leave off the "patch" version number, ie use version number`6.7` not `6.7.8`.
1. In `src/main.rs`, add a `use` statement to bring everything from `rusty_engine`'s prelude into scope. Yes, this is one of those times you can use a `*` in a `use` statement!
1. Create a `fn game_logic(game_state: &mut GameState)` for your game logic.
1. In `main()` create a mutable `Game` struct
1. Then a `// setup goes here` placeholder after your new `Game` struct.
1. Finally, at the end of `main()` call `.run(game_logic)` on whatever you named your `Game` instance.

Now you're ready to proceed with your scenario!

## Scenarios...

- [Cannon Practice](https://github.com/CleanCut/rusty_engine/tree/main/doc/scenarios/cannon_practice.md)
- [Car Invaders](https://github.com/CleanCut/rusty_engine/tree/main/doc/scenarios/car_invaders.md)
- [Car Shoot](https://github.com/CleanCut/rusty_engine/tree/main/doc/scenarios/car_shoot.md)
- [Driver's Ed](https://github.com/CleanCut/rusty_engine/tree/main/doc/scenarios/drivers_ed.md)
- [Labrinth](https://github.com/CleanCut/rusty_engine/tree/main/doc/scenarios/labrinth.md)
- [Road Race](https://github.com/CleanCut/rusty_engine/tree/main/doc/scenarios/road_race.md)
