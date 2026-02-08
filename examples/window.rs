//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example window

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        resolution: WindowResolution::new(800, 200),
        title: "Custom Window Settings".into(),
        resizable: false,
        decorations: false,
        ..Default::default() // for the rest of the options, see https://docs.rs/bevy/0.17/bevy/index.html
    });
    let _ = game.add_text(
        "message",
        "This is a heavily-customized window.\nResizing and window decorations have been disabled.\nPress Esc to exit.",
    );
    game.run(GameState {});
}
