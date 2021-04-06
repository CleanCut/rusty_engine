use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    // Play sound effects every time the timer runs out

    game.add_game_logic(logic);
    game.game_state_mut()
        .timer_map
        .insert("music change timer".into(), Timer::from_seconds(40.0, true));
    game.run();
}

fn logic(game_state: &mut GameState, time: &Time) {
    // gain another life every time the timer goes off
    if game_state.bool_map.get("music started").is_none() {
        game_state.bool_map.insert("music started".into(), true);
        game_state
            .audio_manager
            .play_music(MusicPreset::MysteriousMagic);
    }
    println!("{}", time.time_since_startup().as_secs_f32());
    if game_state
        .timer_map
        .get_mut("music change timer")
        .unwrap()
        .tick(time.delta())
        .just_finished()
    {
        if game_state.bool_map.get("music changed").is_none() {
            game_state.bool_map.insert("music changed".into(), true);
            game_state.audio_manager.play_music(MusicPreset::Classy8Bit);
        }
    }
}
