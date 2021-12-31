//! This is an example of playing sound by path. For playing music or sound effect presets, please
//! see the `music` or `sfx` examples.

use rusty_engine::prelude::*;

rusty_engine::init!();

fn main() {
    let mut game = Game::new();
    let msg = game.add_text(
        "msg",
        "You can add your own sound files to the assets/audio directory (or its\nsubdirectories) and play them by relative path. For example:",
    );
    msg.translation.y = 100.0;

    let msg2 = game.add_text(
        "msg2",
        "engine_state.audio_manager.play_sfx(\"sfx/congratulations.ogg\", 1.0);",
    );
    msg2.translation.y = -100.0;
    msg2.font = "FiraMono-Medium.ttf".to_string();

    game.audio_manager.play_sfx("sfx/congratulations.ogg", 1.0);

    game.run(());
}
