//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example sfx

//! This is an example of playing a sound effect preset. For playing your own sound effect file,
//! please see the `sound` example.

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();
    let msg = game.add_text(
        "msg",
        "You can play sound effect presets that are included in the asset pack. For example:",
    );
    msg.translation.y = 50.0;

    let msg2 = game.add_text(
        "msg2",
        "engine.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);",
    );
    msg2.translation.y = -50.0;
    msg2.font = "font/FiraMono-Medium.ttf".to_string();

    game.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);

    game.run(GameState {});
}
