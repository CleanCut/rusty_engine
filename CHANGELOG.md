<!-- next-header -->
## [Unreleased] - ReleaseDate

### BREAKING CHANGES

- The fundamental way that Rusty Engine connects a user's game state to Bevy has been heavily
refactored to a new solution based on macros so that users can provide a custom struct with their
desired game state. This obseletes the old generic vectors and maps of various types that used to be
stored on the `GameState` struct (which has been renamed to `EngineState` to more accurately
describe what it is used for). Please refer to the [readme](./README.md) and
[docs](https://docs.rs/rusty_engine/latest/rusty_engine/) for more information on the new approach.
- `GameState` has been renamed to `EngineState` so that user's custom game state can be referred to
as `GameState` instead.

### Other CHanges

- (meta) Improved CI times by using sccache together with GitHub Actions caching

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
- 


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
[Unreleased]: https://github.com/CleanCut/rusty_engine/compare/v2.0.1...HEAD
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
