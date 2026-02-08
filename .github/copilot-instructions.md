# Rusty Engine - Copilot Instructions

## Project Overview

Rusty Engine is a 2D game engine for learning Rust, built as a beginner-friendly wrapper around [Bevy](https://bevyengine.org/). It provides a simplified API for sprites, text, audio, input handling, collision detection, and game state management.

This is a **library crate** — there is no `main.rs`. Users create games by depending on this crate and using the `Game` struct as their entry point.

## Architecture

The codebase is flat — all modules live directly in `src/` with no subdirectories:

- **`lib.rs`** — Public API surface, prelude module, direction constants (UP, DOWN, LEFT, RIGHT, etc.)
- **`game.rs`** — Core `Engine` and `Game<S>` structs. `Engine` holds all game state (sprites, texts, input, audio, timing). `Game` wraps a Bevy `App` and provides `add_logic()` / `run()`.
- **`sprite.rs`** — `Sprite` struct (label, position, rotation, scale, collider) and `SpritePreset` enum (20 built-in sprites).
- **`text.rs`** — `Text` struct for on-screen text rendering.
- **`audio.rs`** — `AudioManager` with `play_sfx()` / `play_music()` / `stop_music()`. Includes `SfxPreset` (18 sounds) and `MusicPreset` (3 tracks).
- **`keyboard.rs`** — `KeyboardState` with pressed/just_pressed/just_released tracking and fluent `chain()` API.
- **`mouse.rs`** — `MouseState` with location, motion, buttons, and wheel state. Coordinates are game-space (positive x=right, positive y=up).
- **`physics.rs`** — Convex polygon collision detection using Separating Axis Theorem (SAT). `Collider` enum, `CollisionEvent`, `CollisionPair`.

### Key Design Pattern

The engine uses a **sync pattern** between user code and Bevy ECS:
1. Each frame, Bevy state is copied into the `Engine` struct
2. User logic functions receive `&mut Engine` and a mutable reference to a user-defined game state struct that the user marked with `#[derive(Resource)` and passed into `Game::run()`
3. After user logic runs, `game_logic_sync()` pushes changes back to Bevy entities

User-facing game logic functions always have the following signature, where `GameState` is the user-defined game state struct:
```rust
fn my_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic here
}
```

## Code Conventions

- **Prelude pattern**: All public types are re-exported via `rusty_engine::prelude::*`
- **Labels as identifiers**: Sprites and texts are stored in `bevy::utils::HashMap<String, Sprite/Text>` keyed by a unique label string
- **Constants**: Use `SCREAMING_SNAKE_CASE` (e.g., `ROTATION_SPEED`, `TEXT_DEFAULT_LAYER`)
- **Enums for presets**: Built-in assets use enums (`SpritePreset`, `SfxPreset`, `MusicPreset`) that implement `IntoIterator` and filepath conversion
- **Collider files**: Sprite colliders are stored as `.collider` files in RON format alongside sprite assets
- **Resource derive**: Game state structs must derive `#[derive(Resource)]` for Bevy compatibility
- **Doc comments**: Use `///` for public API documentation. Include usage examples in doc comments.
- **No `unwrap()` in library code** — prefer proper error handling or `expect()` with descriptive messages if the error is not recoverable.

## Dependencies

Key dependencies (keep versions aligned when updating):
- `bevy` — Core engine (selective features: audio, rendering, text, gamepad, GLTF)
- `bevy_prototype_lyon` — Shape rendering for collider visualization
- `ron` — RON format serialization for collider data
- `serde` — Serialization/deserialization for colliders
- `rand` — Dev dependency only, used in examples

Dependencies that start with `bevy_` are bevy plugins and need to be a version supported by the version of bevy we are currently on. The README.md file for these dependencies usually has a table indicating which version(s) are compatible with bevy version(s).

## Build, Test, and Lint

```bash
# Run tests
cargo test --workspace --all-targets --all-features

# Run clippy linter
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Run a specific example
cargo run --release --example <name>
# Available examples: collision, collider, game_state, keyboard, layer,
#   level_creator, mouse, music, placement, sfx, sound, sprite, text, window
```

Every change should be validated by running the example(s) that touch the change. If no example touches the change, then an existing example related to the change should be updated to touch the change or, for completely new features, a new example should be added.

Examples need to compile, run, and exit successfully. Additionally, examples need to behave correctly while running. In order to verify that examples run correctly, a human should be prompted that an example is going to be run, and then if it compiles, runs, and exits successfully then you should ask the human if the example worked correctly.

For changes that affect more than three examples, `script/test_examples` should be run to test all examples. Upon successful build and run of all examples, you should ask a human if all examples worked correctly.

CI runs on Ubuntu, macOS, and Windows. Linux requires `libasound2-dev` and `libudev-dev` system packages.

## Examples and Scenarios

- **`examples/`** — Runnable demos of individual engine features. Each has a doc comment explaining how to run it.
- **`scenarios/`** — Game programming challenges at varying difficulty (Easy → Insane). Include skeleton code and step-by-step instructions.
- **`tutorial/`** — mdBook-based tutorial covering all engine features, built with `mdbook`.

All examples, scenarios, and the tutorial documentation should be updated whenever the public API is changed.

## Release Process

Uses `cargo-release` configured via `release.toml`. Version replacements are automated across README.md, tutorial files, and CHANGELOG.md. See `RELEASE.md` for full instructions.

## Important Guidelines

- This engine is designed for **beginners learning Rust** — keep the API simple and approachable
- Avoid exposing raw Bevy types in the public API; wrap them in engine-specific types. Large enums used for input are an exception, and can be passed straight through from Bevy.
- All coordinates use a game-space system where positive x is right and positive y is up (origin at center)
- Sprite layers default to 0.0; text layers default to 900.0 (rendered on top)
- When adding new presets (sprites, sounds, music), add the corresponding assets to `assets/` and update the relevant enum
- Collider polygons must be convex for the SAT collision algorithm to work correctly
- The `Engine` struct fields are split into "SYNCED" (user-modifiable, synced back to Bevy) and "INFO" (read-only, populated from Bevy each frame) categories
