//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example window

use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        width: 800.0,
        height: 200.0,
        resize_constraints: WindowResizeConstraints {
            min_width: 700.0,
            min_height: 150.0,
            max_width: 900.0,
            max_height: 300.0,
        },
        title: "Custom Window Settings".into(),
        resizable: true,
        decorations: false,
        cursor_visible: false,
        ..Default::default() // for the rest of the options, see https://docs.rs/bevy/0.5.0/bevy/window/struct.WindowDescriptor.html
    });
    let _ = game.add_text(
        "message",
        "This is a heavily-customized window.\nYou may resize it a little bit.\nPress Esc to exit.",
    );
    game.run(());
}
