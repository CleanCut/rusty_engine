use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();
    let msg = game.add_text_actor(
        "msg",
        "Press any key to advance to the next music selection.\n\nIf you are not running with \"--release\", it may take several seconds for each song to load!",
    );
    msg.translation.y = -200.0;

    game.run(logic);
}

fn logic(game_state: &mut EngineState) {
    let mut should_play_new_song = false;

    // Play a "new" song if nothing has been played before
    let current_music_index = game_state
        .usize_map
        .entry("current music index".into())
        .or_insert_with(|| {
            should_play_new_song = true;
            0
        });

    // Play a new song because a key was pressed
    for ev in game_state.keyboard_events.drain(..) {
        if ev.state != ElementState::Pressed {
            continue;
        }
        *current_music_index = (*current_music_index + 1) % MusicPreset::variant_iter().count();
        should_play_new_song = true;
        break;
    }

    if should_play_new_song {
        // Actually play the new song
        let music_preset = MusicPreset::variant_iter()
            .nth(*current_music_index)
            .unwrap();
        game_state.audio_manager.play_music(music_preset, 1.0);
        // And make a text actor saying what song we're playing
        let note1 = game_state.add_text_actor("note1", format!("Looping: {:?}", music_preset));
        note1.font_size = 75.0;
    }
}
