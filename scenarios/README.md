# Game Scenarios

Here are some interesting scenarios that you can use to walk through building a working game prototype using Rusty Engine.  If you would like to contribute your own game scenario idea, please feel free to open a pull request!

## Common Setup! Do this first!

All scenarios assume you've followed the basic setup below.  All scenarios assume default window width and height. If your screen is too small to accomodate a `1280 x 720` window or if you [customized the window settings](https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs) then you may have to adapt the translation coordinates and other numerical variables in the scenario to account for the difference.

First, create your project and download the `rusty_engine` assets
1. Create your project with `cargo new somename`. Replace `somename` with a fun name!
1. Download the asset packs. I can think of 3 easy ways to do this:
    1. Clone the `rusty_engine` repository and copy the `assets/` directory over to your own project
    1. Download a [zip file](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.zip) or [tarball](https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz) of the `rusty_engine` repository, extract it, and copy the `assets/` directory over to your own project.
    1. On a posix compatible shell, run this command inside your project directory:
```shell
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```

Next, set up the skeleton of your project:
1. Add `rusty_engine` as a dependency in your `Cargo.toml`
    1. Forgot the latest version number?  You can always peek at [the `rusty_engine` page on Crates.io](https://crates.io/crates/rusty_engine)
    1. It's recommended to leave off the "patch" version number, ie use version number`6.7` not `6.7.8`.
1. In `src/main.rs`, add a `use` statement to bring everything from `rusty_engine`'s prelude into scope. Yes, this is one of those times you can use a `*` in a `use` statement!
1. Create a `fn game_logic(game_state: &mut GameState)` for your game logic.
1. In `main()` create a _mutable_ `Game` struct
1. Then add a `// setup goes here` placeholder line after your new `Game` struct.
1. Finally, at the end of `main()` call `.run(game_logic)` on whatever you named your `Game` instance.

Now you're ready to proceed with your scenario!

**Remember to run your game in release mode for good performance! `cargo run --release`**

## Scenarios (description, full instructions, reference code)
1. Road Race - [Instructions](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/road_race.md), [Reference Code](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/road_race.rs)

## Scenarios (description, reference code)

1. Car Shoot - [Description](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/car_shoot.md), [Reference Code](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/road_race.rs)
1. Driver's Ed - [Description](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/car_shoot.md), [Reference Code](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/extreme_drivers_ed.rs)

## Scenarios (description only)

1. [Cannon Practice](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/cannon_practice.md)
1. [Car Invaders](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/car_invaders.md)
1. [Labrinth](https://github.com/CleanCut/rusty_engine/tree/main/scenarios/labrinth.md)
