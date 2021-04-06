use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    // Play sound effects every time the timer runs out
    game.game_state_mut()
        .timer_map
        .insert("sfx timer".into(), Timer::from_seconds(2.0, true));

    game.add_game_logic(logic);

    game.run();
}

fn logic(game_state: &mut GameState, time: &Time) {
    // gain another life every time the timer goes off
    if game_state
        .timer_map
        .get_mut("sfx timer")
        .unwrap()
        .tick(time.delta())
        .just_finished()
    {
        game_state.audio_manager.play_sfx(SfxPreset::Confirmation1);
    }
}
