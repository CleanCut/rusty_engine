use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();
    game.game_state_mut()
        .timer_map
        .insert("music change timer".into(), Timer::from_seconds(30.0, true));
    let _ = game.add_text_actor(
        "msg",
        "If you are not running with \"--release\", it may take several seconds for each song to load.",
    );

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    // gain another life every time the timer goes off
    if game_state.bool_map.get("music started").is_none() {
        println!("Playing MysteriousMagic for about 30 seconds.");
        game_state.bool_map.insert("music started".into(), true);
        game_state
            .audio_manager
            .play_music(MusicPreset::MysteriousMagic, 1.0);
    }
    if game_state
        .timer_map
        .get_mut("music change timer")
        .unwrap()
        .tick(game_state.delta)
        .just_finished()
    {
        if game_state.bool_map.get("music changed").is_none() {
            println!("Switching to Classy8Bit...forever. Press Esc on the GUI Window or Ctrl-C in the terminal to quit.");
            game_state.bool_map.insert("music changed".into(), true);
            game_state
                .audio_manager
                .play_music(MusicPreset::Classy8Bit, 1.0);
        }
    }
}
