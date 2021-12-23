//! This is an example of playing a sound effect preset. For playing your own sound effect file,
//! please see the `sound` example.

use rusty_engine::prelude::*;

rusty_engine::init!();

fn main() {
    let mut game = Game::new();
    let msg = game.add_text_actor(
        "msg",
        "You can play sound effect presets that are included in the asset pack. For example:",
    );
    msg.translation.y = 50.0;

    let msg2 = game.add_text_actor_with_font(
        "msg2",
        "engine_state.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);",
        "FiraMono-Medium.ttf",
    );
    msg2.translation.y = -50.0;

    game.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);

    game.run(());
}
