use rusty_engine::prelude::*;

#[derive(Default)]
struct GameState {
    sfx_timers: Vec<Timer>,
    end_timer: Timer,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();
    let mut game_state = GameState::default();

    // One timer to launch each sound effect
    for (i, _sfx) in SfxPreset::variant_iter().enumerate() {
        game_state
            .sfx_timers
            .push(Timer::from_seconds((i as f32) * 2.0, false));
    }
    // One timer to end them all
    game_state.end_timer =
        Timer::from_seconds((SfxPreset::variant_iter().len() as f32) * 2.0 + 1.0, false);

    let mut msg = game.add_text_actor("msg", "Playing sound effects!");
    msg.translation = Vec2::new(0.0, 100.0);
    msg.font_size = 60.0;

    let mut sfx_label = game.add_text_actor("sfx_label", "");
    sfx_label.translation = Vec2::new(0.0, -100.0);
    sfx_label.font_size = 90.0;

    game.add_logic(logic);
    game.run(game_state);
}

fn logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    for (i, timer) in game_state.sfx_timers.iter_mut().enumerate() {
        // None of the timers repeat, and they're all set to different times, so when the timer in
        // index X goes off, play sound effect in index X
        if timer.tick(engine_state.delta).just_finished() {
            // Play a new sound effect
            let sfx = SfxPreset::variant_iter().nth(i).unwrap();
            engine_state.audio_manager.play_sfx(sfx);
            // Update the text to show which sound effect we are playing
            let sfx_label = engine_state.text_actors.get_mut("sfx_label").unwrap();
            sfx_label.text = format!("{:?}", sfx);
        }
    }

    // Are we all done?
    if game_state
        .end_timer
        .tick(engine_state.delta)
        .just_finished()
    {
        let sfx_label = engine_state.text_actors.get_mut("sfx_label").unwrap();
        sfx_label.text = "That's all! Press Esc to quit.".into();
    }
    true
}
