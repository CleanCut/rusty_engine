use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    for (i, _sfx) in SfxPreset::variant_iter().enumerate() {
        game.game_state_mut()
            .timer_vec
            .push(Timer::from_seconds((i as f32) * 2.0, false));
    }
    game.game_state_mut().timer_map.insert(
        "quit_timer".into(),
        Timer::from_seconds((SfxPreset::variant_iter().len() as f32) * 2.0 + 3.0, false),
    );

    game.add_game_logic(logic);

    game.run();
}

fn logic(game_state: &mut GameState, time: &Time) {
    for (i, timer) in game_state.timer_vec.iter_mut().enumerate() {
        // gain another life every time the timer goes off
        if timer.tick(time.delta()).just_finished() {
            let sfx = SfxPreset::variant_iter().nth(i).unwrap();
            println!("Playing {:?}", sfx);
            game_state.audio_manager.play_sfx(sfx);
        }
    }
    if game_state
        .timer_map
        .get_mut("quit_timer")
        .unwrap()
        .tick(time.delta())
        .just_finished()
    {
        println!("All done!");
        game_state.exit();
    }
}
