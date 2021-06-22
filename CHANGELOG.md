<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.1] - 2021-06-22

### Fixed

- Fixed a bug in the `Hash` trait implementation for `CollisionPair`
- Fixed clippy warnings

## [1.0.0] - 2021-06-22

### Everything

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

### Added

- All the basic [features for 1.0](https://github.com/CleanCut/rusty_engine/issues/9)
- This release is to test the remaining 1.0 functionality, but not all the documentation and scenarios are written, so we're not quite ready for 1.0.

## [0.12.0] - 2021-06-16

### Changed

- Gut the entire project and started over by wrapping [Bevy] with a simple interface for beginners to use.
- Implement the following features: GameState, Actors w/Transform handling, Sprites w/asset pack, audio sfx/music with asset pack, timer utility.
- Add a release config and release doc

## 0.0.1 - [0.11.0]

- Rapid, messy development based on gfx via `glium`, sound via `rusty_audio`, timing via `rusty_time`, and custom logic for everything else.  This approach never reached a very usable state.

[Bevy]: https://bevyengine.org
<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/assert-rs/predicates-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/assert-rs/predicates-rs/compare/v0.13.0...v1.0.0
[0.13.0]: https://github.com/assert-rs/predicates-rs/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/cleancut/rusty_engine/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/cleancut/rusty_engine/compare/v0.10.0...v0.11.0
