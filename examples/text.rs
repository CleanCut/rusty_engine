use rusty_engine::prelude::*;

struct GameState {
    timer: Timer,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();
    let fps = game.add_text_actor("fps", "FPS: ");
    fps.translation = Vec2::new(0.0, 250.0);
    fps.font_size = 60.0;

    let mut msg = game.add_text_actor("msg", "Changing text and font size after creation is supported, but requires\nre-rendering the text image each time, so use it sparingly!\n\nChanging the text's translation, rotation*, and scale* is fast,\n so feel free to do that a lot.");
    msg.font_size = 24.0;

    let mut msg2 = game.add_text_actor("msg2", "*Changing rotation and scale will not work until Bevy 0.6 is released,\nbut changing the translation works great already!");
    msg2.font_size = 20.0;

    let mut msg3 = game.add_text_actor(
        "msg3",
        "Changing font size re-renders the text smoothly at a different size,\nbut using this technique for animation is both jittery (character kerning) and expensive.",
    );
    msg3.font_size = 35.0;
    msg3.translation = Vec2::new(0.0, 150.0);

    let game_state = GameState {
        timer: Timer::from_seconds(0.2, true),
    };
    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    if game_state.timer.tick(engine_state.delta).just_finished() {
        let mut fps = engine_state.text_actors.get_mut("fps").unwrap();
        fps.text = format!("FPS: {:.1}", 1.0 / engine_state.delta_f32);
    }

    let msg2 = engine_state.text_actors.get_mut("msg2").unwrap();
    msg2.translation.x = 75.0 * (engine_state.time_since_startup_f64 * 0.5).sin() as f32;
    msg2.translation.y = 75.0 * (engine_state.time_since_startup_f64 * 0.5).cos() as f32 - 200.0;

    let msg3 = engine_state.text_actors.get_mut("msg3").unwrap();
    msg3.font_size = 10.0 * (engine_state.time_since_startup_f64 * 0.5).cos() as f32 + 20.0;
    true
}
