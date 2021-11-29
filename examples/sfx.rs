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
        Timer::from_seconds((SfxPreset::variant_iter().len() as f32) * 2.0 + 1.0, false),
    );

    let mut msg = game.add_text_actor("msg", "Playing sound effects!");
    msg.translation = Vec2::new(0.0, 100.0);
    msg.font_size = 60.0;

    let mut sfx_label = game.add_text_actor("sfx_label", "");
    sfx_label.translation = Vec2::new(0.0, -100.0);
    sfx_label.font_size = 90.0;

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    for (i, timer) in game_state.timer_vec.iter_mut().enumerate() {
        // None of the timers repeat, and they're all set to different times, so when the timer in
        // index X goes off, play sound effect in index X
        if timer.tick(game_state.delta).just_finished() {
            // Play a new sound effect at full volume
            let sfx = SfxPreset::variant_iter().nth(i).unwrap();
            game_state.audio_manager.play_sfx(sfx, 1.0);
            // Update the text to show which sound effect we are playing
            let sfx_label = game_state.text_actors.get_mut("sfx_label").unwrap();
            sfx_label.text = format!("{:?}", sfx);
        }
    }

    // Are we all done?
    if game_state
        .timer_map
        .get_mut("quit_timer")
        .unwrap()
        .tick(game_state.delta)
        .just_finished()
    {
        let sfx_label = game_state.text_actors.get_mut("sfx_label").unwrap();
        sfx_label.text = "That's all! Press Esc to quit.".into();
    }
}
