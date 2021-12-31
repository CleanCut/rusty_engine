use rusty_engine::prelude::*;

struct GameState {
    timer: Timer,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();
    let fps = game.add_text("fps", "FPS: ");
    fps.translation = Vec2::new(0.0, 250.0);
    fps.font = "FiraMono-Medium.ttf".to_string();
    fps.font_size = 60.0;

    let zoom_msg = game.add_text(
        "zoom_msg",
        "Changing font size re-renders the text smoothly at a different size,\nbut using this technique for animation is both jittery (character kerning) and expensive.",
    );
    zoom_msg.font_size = 35.0;
    zoom_msg.translation = Vec2::new(0.0, 150.0);

    let font_msg = game.add_text(
        "font_msg",
        "You can choose a font at creation time by providing the filename of a font stored in assets/font.\n\"FiraSans-Bold.ttf\" is the default. \"FiraMono-Medium.ttf\" is also included in the asset pack."
    );
    font_msg.font_size = 20.0;
    font_msg.font = "FiraMono-Medium.ttf".to_string();
    font_msg.translation.y = 0.0;

    let msg = game.add_text("msg", "Changing the text's translation, rotation*, and scale* is fast,\n so feel free to do that a lot.");
    msg.font_size = 24.0;
    msg.translation.y = -150.0;

    let msg2 = game.add_text("msg2", "*Changing rotation and scale will not work until Bevy 0.6 is released,\nbut changing the translation works great already!");
    msg2.font_size = 20.0;

    let game_state = GameState {
        timer: Timer::from_seconds(0.2, true),
    };
    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    if game_state.timer.tick(engine_state.delta).just_finished() {
        let mut fps = engine_state.texts.get_mut("fps").unwrap();
        fps.value = format!("FPS: {:.1}", 1.0 / engine_state.delta_f32);
    }

    let msg2 = engine_state.texts.get_mut("msg2").unwrap();
    msg2.translation.x = 50.0 * (engine_state.time_since_startup_f64 * 0.5).sin() as f32;
    msg2.translation.y = 50.0 * (engine_state.time_since_startup_f64 * 0.5).cos() as f32 - 275.0;

    let msg3 = engine_state.texts.get_mut("zoom_msg").unwrap();
    msg3.font_size = 10.0 * (engine_state.time_since_startup_f64 * 0.5).cos() as f32 + 25.0;
    true
}
