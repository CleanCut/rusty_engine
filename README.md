# Rusty Engine

A fun and easy 2D game engine to make games in Rust.

[Questions], [bug reports], and contributions are most welcome!

[Questions]: https://github.com/CleanCut/rusty_engine/issues/new
[bug reports]: https://github.com/CleanCut/rusty_engine/issues/new

## Libraries

Rusty Engine is a collection of related but independent libraries.  All of the libraries are
designed to optionally be used standalone, with the exception of `rusty_core` which is for shared
functionality and re-exports from external crates.

- `rusty_audio` - Load & play audio
- `rusty_gfx` - Create windows, display graphics, handle input events (keyboard, mouse, etc.)
- `rusty_core` - Used by the rest of the `rusty_*` crates for anything they have in common,
  including re-exports of external libraries.