<!-- next-header -->
## [Unreleased] - ReleaseDate

## [6.0.0] - 2023-12-03

### Breaking changes

- `WindowDescriptor` has been renamed to `Window`
- You must new add `#[derive(Resource)]` above your `GameState` struct.
- It is no longer possible to use the unit type `()` instead of a `GameState` struct. Always create a `GameState` struct (it can be an empty struct with no fields).
- The `Timer` now takes a `TimerMode` enum variant instead of a `bool`. Use `TimerMode::Once` for a timer that runs once, or `TimerMode::Repeating` for a repeating timer.
- The following `KeyCode` variants have been renamed:
  - `LShift` -> `ShiftLeft`
  - `RShift` -> `ShiftRight`
  - `LAlt` -> `AltLeft`
  - `RAlt` -> `AltRight`
  - `LBracket` -> `BracketLeft`
  - `RBracket` -> `BracketRight`
  - `LControl` -> `ControlLeft`
  - `RControl` -> `ControlRight`
  - `LShift` -> `ShiftLeft`
  - `LWin` -> `SuperLeft`
  - `RWin` -> `SuperRight`


### Improved

- Update bevy from 0.8 to 0.12
- Update bevy_prototype_lyon from 0.6 to 0.10
- Update ron from 0.7 to 0.8
- Fixed some inconsistent parameter names in examples
- Some examples' audio was turned down lower


## [5.2.1] - 2022-11-15

### Improved

- The `collider` (example) editor now snaps to half-pixels.
- The `collider` (example) editor now handles relative paths that begin with `./` or `.\`, which was especially vexing to powershell users. Contributed by [@webbertakken](https://github.com/webbertakken) in [#59].

[#59]: https://github.com/CleanCut/rusty_engine/pull/59

## [5.2.0] - 2022-09-13

### Added

- Added `KeyboardStateChain` and `MouseStateChain` to provide a functional interface for dealing with user input. Call `.chain()` on  `KeyboardState` or `MouseState` to access them. These new structs have methods with the same names as in their `Chain`-less variants which accept closures to perform the logic, and can be chained. Contributed by [@just-do-halee](https://github.com/just-do-halee) in [#55].

### Improved

- `CollisionPair` now implements `IntoIterator`, so you can do, for example: `for label in event.pair { ... }`. Contributed by [@just-do-halee](https://github.com/just-do-halee) in [#55].

[#55]: https://github.com/CleanCut/rusty_engine/pull/55

## [5.1.1] - 2022-08-01

### Improved

- Updated to `bevy 0.8` and `bevy_prototype_lyon 0.6` under the hood, which resolves the occasional stuttering problem.


## [5.1.0] - 2022-06-18

### Improved

- Fixed `CollisionPair::either_contains` to use `.contains` instead of `==`, contributed by [@etnt](https://github.com/etnt) in [#51](https://github.com/CleanCut/rusty_engine/pull/51)
- Added `CollisionPair::either_equals_to` which uses `==`, contributed by [@etnt](https://github.com/etnt) in [#51](https://github.com/CleanCut/rusty_engine/pull/51)
- Fixed documentation for a few fields of the `Engine` struct which were in the wrong place.
- Improved CI caching. Builds should now generally complete in under 3 minutes.

## [5.0.6] - 2022-05-19

### Improved

- Made a pass through the API documentation and Tutorials, clarifying, correcting, and filling in blanks.
- Reduced the debug level of logging the window dimension resizing from info to debug

## [5.0.5] - 2022-05-10

### Improved

- Dropped the lazy_static, log, and env_logger dependencies.

### BREAKING CHANGES

Yes, I know I shouldn't be releasing breaking changes in a patch release...but no one should be using any 5.x versions until I start teaching classes with it publicly...so I'm going to break it until I start teaching with 5.x.

- Updated font loading to be rooted in `assets/` instead of `assets/font/`

## [5.0.4] - 2022-05-09

### Improved

- Updated to bevy 0.7 and bevy_prototype_lyon 0.5
- Stopped using bevy_kira_audio in favor of the built-in bevy audio which nows has all the features we need.

### BREAKING CHANGES

- Renamed the music file extensions to `.ogg` since [bevy doesn't support that file extension yet](https://github.com/bevyengine/bevy/pull/4703) -- this means the asset pack is now different.

## [5.0.3] - 2022-04-29

### New

- The `level_creator` example can be installed globally with `cargo install rusty_engine --example level_creator` and run in the root of your own project with `level_creator`.

### Fixed

- Fixed sprite preset paths which broke due to the changes in 5.0.2

## [5.0.2] - 2022-04-29

### New

- The `collider` example can be installed globally with `cargo install rusty_engine --example collider` and run in the root of your own project with `collider assets/some-image.png`.

### Fixed

- The `collider` example can now load sprites from anywhere inside `assets/`, instead of only from inside `assets/sprite/`.

## [5.0.1] - 2022-04-11

### Improved

- Implemented all the [common traits](https://rust-lang.github.io/api-guidelines/interoperability.html) on public `struct`s and `enum`s that made sense.
- Added documentation for a few structs, enums, and methods that were missing it.

### Fixed

- The `collider` example once again properly regenerates the visualization for circle colliders whenever they are created or altered.
- Changed visibility of internal Bevy plugins from `pub` to `pub(crate)`. Technically this is a breaking change, but these wouldn't have been usable to anyone anyway so I'm ignoring the technicality.

## [5.0.0] - 2022-03-12

### BREAKING CHANGES

- Logic functions no longer return a `bool` to simplify the learning curve. If you want logic functions to run conditionally, instead track your state in your `GameState` and use it to exit early from your logic function.
- The `EngineState` struct and `engine_state` variables have been renamed to `Engine` and `engine`, respectively, for brevity.

## [4.0.0] - 2022-01-29

### BREAKING CHANGES

- `Game` is now generic over the user-provided game state struct, so the `init!` macro from the short-lived `3.0.0` version has been removed! All you need to do is delete the macro call if you have it.
- `EngineState.debug_sprite_colliders` has been renamed `EngineState.show_colliders` for clarity.
- Renamed the `collider_creator` example to `collider` for brevity.
- Added `Sprite.collider_dirty` which you can set to true to regenerate a collider. Necessary if you manually replace `Sprite.collider` with a new collider.

### Other Changes

- Upgraded to Bevy 0.6.
- `Text` rotation and scale now works! 🎉
- Switched to `bevy_prototype_lyon` to power the debug lines. They look much nicer now that I can choose the line thickness.
- Updated (or finished) all of the game scenario descriptions.
- Updated the [tutorial](https://cleancut.github.io/rusty_engine/).

## [3.0.0] - 2021-12-30

### BREAKING CHANGES

- The fundamental way that Rusty Engine connects a user's game state to Bevy has been heavily
refactored to a new solution based on macros so that users can provide a custom struct with their
desired game state. This obseletes the old generic vectors and maps of various types that used to be
stored on the `GameState` struct (which itself has been renamed to `EngineState` to more accurately
describe what it is used for). Please refer to the [readme](./README.md) and
[docs](https://docs.rs/rusty_engine/latest/rusty_engine/) for comprehensive documentation on the new
approach.
- Placing the module-level macro call `rusty_engine::init!(MyGameState)` is now required.
`MyGameState` is any user-defined struct type that will be passed to the logic functions each frame.
- `GameState` has been renamed to `EngineState` so that user's custom game state can be referred to
as `GameState` instead.
   - `GameState::add_actor` has been renamed to `EngineState::add_sprite`
   - `GameState::add_text_actor` has been renamed to `EngineState::add_text`
- `Game` now implements `Deref` and `DerefMut` for `EngineState`, so you can easily access
`EngineState`'s methods from `Game` in `main.rs` for your game setup. `Game::game_state_mut` has
been removed (if it had stayed it would have been renamed `engine_state_mut`, but with the deref
implementations it's not needed at all).
- `GameState.screen_dimensions`, which was set at startup and never updated, has been replaced by `EngineState.window_dimensions`, which is updated every frame so resizing the window can be handled in your game logic.
- Multiple logic functions can now be run. Pass them to `Game::run` in the order you would like them
run. Return `false` to abort running any later functions during the frame.
- Logic functions now need to fit the signature `fn somename(engine_state: &mut EngineState, game_state: &mut GameState) -> bool`, where `GameState` is the user-defined struct passed to `rusty_engine::init!()`, or `()` if nothing was passed in.
- `.play_sfx()` now takes a volume level from `0.0` to `1.0` as a second argument, e.g. `.play_sfx(SfxPreset::Congratulations, 1.0)`
- `Actor` has been renamed to `Sprite` to eliminate the confusing "actor" terminalogy.
   - `Actor::build` has been replaced by `Sprite::new`, which _must_ be used to create a `Sprite` instead of defining a struct literal (enforced via private phantom data). The `Default` implementation has been removed because of the previous restriction.
   - `Actor.preset` has been removed
   - `Actor.filename` (a `String`) has been replaced with `Sprite.filepath` (a `PathBuf`)
   - The builder methods `Actor::set_collision` and `Actor::set_collider` have been removed since we never ended up adopting a builder pattern.
   - `Sprite.collider_filepath` has been added
   - `Sprite::write_collider` has been added (see note below about changes to colliders)
- `TextActor` has been renamed to `Text` to eliminate the confusing "actor" terminology.
   - `TextActor.text` is now `Text.value` for similar reasons.
- `Sprite`s may now be created with either a `SpritePreset` or the path to an image file via both `Sprite::new` or `EngineState::add_sprite`. The image file needs to be stored in `assets/sprite` or one of its subdirectories. When specifying the path to the file, the path relative to `assets/sprite` should be used. For example, if your image is `assets/sprite/circus/animal.png` then you would pass `circus/animal.png` to one of the methods to create the sprite.
- `SpritePreset::build_from_name` and `SpritePreset::build` have been removed (see note above about the new, more flexible way to create sprites)
- `SpritePreset::collider()` has been removed since colliders are no longer hard-coded features of presets (see note below about changes to colliders)
- `SpritePreset::filename -> String` has been replaced by `SpritePreset::filepath -> PathBuf`, which powers the `impl From<SpritePreset> for PathBuf` implementation.
- Colliders are now loaded from collider files. Collider files use the [Rusty Object Notation (RON)](https://github.com/ron-rs/ron) format. The easiest way to create a collider is to run the `collider_creator` example by cloning the `rusty_engine` repository and running `cargo run --release --example collider_creator relative/path/to/my/image.png`. The image needs to be somewhere inside the `assets/` directory. You could also create the collider programmatically, set it on the `Sprite` struct, and call the sprite's `write_collider()` method. Or you could copy an existing collider file, name it the same as your image file (but with the `.collider` extension) and change it to match your image.  Collider coordinates need to define a convex polygon with points going in clockwise order. Coordinates are floating point values relative to the center of the image, with the center of the image being (0.0, 0.0).
- All sprites' colliders in the asset pack have been recreated more cleanly using the new `collider_creator` example.
- The `assets/fonts` directory in the asset pack has been renamed to `assets/font` for consistency with the other directories.
- `KeyboardState` and `MouseState` now both have 6 similar methods for processing key- and button-presses:
   - `pressed` -> `pressed_any`
   - `just_pressed` -> `just_pressed_any`
   - `just_released` -> `just_released_any`

### Other Changes

- `AudioManager::music_playing()` will return whether or not music is currently playing (accessible
through `EngineState:audio_manager`)
- A custom font may now be selected by placing it in `assets/font` and specifying the relative filepath on `Text.font`.
- Custom sounds may now be played via `AudioManager::play_music` and `AudioManager::play_sfx` by
specifying a path to a sound file relative to `assets/audio`.
- `Collider` now implements `PartialEq`, `Serialize`, and `Deserialize`
- `Collider::is_convex` was added to make it easier to tell if you have a convex collider.
- The `collider_creator` example was added to make it easy to load a sprite and make a collider file for it. Place your image file (let's call it `my_image.png`) anywhere inside your local clone of the Rusty Engine `assets/` directory and then run the example: `cargo run --release --example collider_creator -- assets/my_image.png`.  Afterwards, copy the image file and the new collider file `my_image.collider` file over to the assets directory of your own project.
- You can now toggle debug rendering of colliders by setting `EngineState.debug_sprite_colliders` to `true`. The `collision` example will now toggle that value when you press the `C` key.
- (meta) Improved CI times by using sccache together with GitHub Actions caching
- Circular colliders no longer have duplicate starting and ending coordinates

## [2.0.1] - 2021-11-15

- Print out error and URL if assets directory is not present

## [2.0.0] - 2021-09-09

### BREAKING CHANGES

- Renamed `GameState.cursor_moved_events` to `GameState.mouse_location_events`
- Renamed `Gamestate.delta_seconds` to `GameState.delta_f32`
- Renamed `Gamestate.seconds_since_startup` to `GameState.time_since_startup_f64`

### Other Changes

- Added `GameState::keyboard_state` (and a new `KeyboardState` struct), to determine the current state of the keyboard. This should be preferred over keyboard events when dealing with character movement, etc.
- Added `GameState::mouse_state` (and a new `MouseState` struct), to determine the current state of the mouse. In most cases, this should be preferred over mouse methods when dealing with character movement, etc.
- Added a new `MouseWheelState` struct to represent the state of the mouse wheel, which is a simplified representation of cumuluative mouse wheel events.
- Added an "Extreme Driver's Ed" scenario reference implementation (`cargo run --release --example extreme_drivers_ed`).
- Documented `GameState`
- Added `GameState.vec2_map` and `GameState.vec2_vec` as collections for the user to store state in.
- Switched all instances of `std::collections::HashMap` to `bevy::utils::HashMap`.
- Updated all examples to adjust for breaking changes, also:
   - The `keyboard` example has been renamed to `keyboard_events` to distinguish it from the new `keyboard_state` example which uses `KeyboardState` for smooth movement
   - The `mouse` example has been renamed to `mouse_events` to distinguish it from the new `mouse_state` example which uses `MouseState` for smooth movement
- Added now `level_creator` example to use as a rudimentary level creator (originally added in 1.1.0)

## [1.1.4] - 2021-08-26

- Improve documentation in the `actor` module

## [1.1.3] - 2021-08-26

- level_creator: enable collision in generated code

## [1.1.2] - 2021-08-26

- level_creator: have current actor on top of placed actors

## [1.1.1] - 2021-08-25

- level_creator: Start the actors at normal scale

## [1.1.0] - 2021-08-25

### Changes

- Bump `env_logger` to `0.9`
- Add `level_creator` example. Try it out with `cargo run --release --example level_creator`
- `ActorPreset` gained several new abilities:
  - It now implements the `PartialEq` trait
  - `.build_from_name()` allows building a preset from a string
  - `.prev()` and `.next()` allow iterating over presets based on the current preset
- `GameState` gained `f32_map` and `f32_vec` fields for storing `f32` values

## [1.0.3] - 2021-06-22

### Changes

- Fixed an incompatibility with Rust versions `< 1.53.0`

## [1.0.2] - 2021-06-22

### Changes

- Added Car Shoot reference implementation (still needs a scenario writeup)
- Added some helper methods to `CollisionPair`
- Fixed `CollisionPair` tuple members not being `pub`

## [1.0.1] - 2021-06-22

### Changes

- Fixed a bug in the `Hash` trait implementation for `CollisionPair`
- Fixed clippy warnings

## [1.0.0] - 2021-06-22

### Changes

- Created some game scenarios (only Road Racer is fully complete)
- Created Road Racer reference example
- Added ability to configure window settings
- Set default window title
- Overhauled readme
- Improved `GameState` and `CollisionState` by adding some methods
- Made volume configurable when starting to play music
- Made a way to stop playing music
- Added instructions for how to download assets

## [0.13.0] - 2021-06-21

### Changes

- All the basic [features for 1.0](https://github.com/CleanCut/rusty_engine/issues/9)
- This release is to test the remaining 1.0 functionality, but not all the documentation and scenarios are written, so we're not quite ready for 1.0.

## [0.12.0] - 2021-06-16

### Changes

- Gut the entire project and started over by wrapping [Bevy] with a simple interface for beginners to use.
- Implement the following features: GameState, Actors w/Transform handling, Sprites w/asset pack, audio sfx/music with asset pack, timer utility.
- Add a release config and release doc

## 0.0.1 - [0.11.0]

- Rapid, messy development based on gfx via `glium`, sound via `rusty_audio`, timing via `rusty_time`, and custom logic for everything else.  This approach never reached a very usable state.

[Bevy]: https://bevyengine.org
<!-- next-url -->
[Unreleased]: https://github.com/CleanCut/rusty_engine/compare/v6.0.0...HEAD
[6.0.0]: https://github.com/CleanCut/rusty_engine/compare/v5.2.1...v6.0.0
[5.2.1]: https://github.com/CleanCut/rusty_engine/compare/v5.2.0...v5.2.1
[5.2.0]: https://github.com/CleanCut/rusty_engine/compare/v5.1.1...v5.2.0
[5.1.1]: https://github.com/CleanCut/rusty_engine/compare/v5.1.0...v5.1.1
[5.1.0]: https://github.com/CleanCut/rusty_engine/compare/v5.0.6...v5.1.0
[5.0.6]: https://github.com/CleanCut/rusty_engine/compare/v5.0.5...v5.0.6
[5.0.5]: https://github.com/CleanCut/rusty_engine/compare/v5.0.4...v5.0.5
[5.0.4]: https://github.com/CleanCut/rusty_engine/compare/v5.0.3...v5.0.4
[5.0.3]: https://github.com/CleanCut/rusty_engine/compare/v5.0.2...v5.0.3
[5.0.2]: https://github.com/CleanCut/rusty_engine/compare/v5.0.1...v5.0.2
[5.0.1]: https://github.com/CleanCut/rusty_engine/compare/v5.0.0...v5.0.1
[5.0.0]: https://github.com/CleanCut/rusty_engine/compare/v4.0.0...v5.0.0
[4.0.0]: https://github.com/CleanCut/rusty_engine/compare/v3.0.0...v4.0.0
[3.0.0]: https://github.com/CleanCut/rusty_engine/compare/v2.0.1...v3.0.0
[2.0.1]: https://github.com/CleanCut/rusty_engine/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/CleanCut/rusty_engine/compare/v1.1.4...v2.0.0
[1.1.4]: https://github.com/CleanCut/rusty_engine/compare/v1.1.3...v1.1.4
[1.1.3]: https://github.com/CleanCut/rusty_engine/compare/v1.1.2...v1.1.3
[1.1.2]: https://github.com/CleanCut/rusty_engine/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/CleanCut/rusty_engine/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/CleanCut/rusty_engine/compare/v1.0.3...v1.1.0
[1.0.3]: https://github.com/CleanCut/rusty_engine/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/CleanCut/rusty_engine/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/CleanCut/rusty_engine/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/CleanCut/rusty_engine/compare/v0.13.0...v1.0.0
[0.13.0]: https://github.com/CleanCut/rusty_engine/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/cleancut/rusty_engine/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/cleancut/rusty_engine/compare/v0.10.0...v0.11.0
