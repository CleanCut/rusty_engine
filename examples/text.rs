use rusty_engine::prelude::*;

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
        "Changing font size is expensive, but it can be done.",
    );
    msg3.font_size = 20.0;
    msg3.translation = Vec2::new(0.0, 150.0);

    game.game_state_mut()
        .timer_vec
        .push(Timer::from_seconds(0.2, true));
    game.run(game_logic);
}

fn game_logic(game_state: &mut GameState) {
    let timer = game_state.timer_vec.get_mut(0).unwrap();
    if timer.tick(game_state.delta).just_finished() {
        let mut fps = game_state.text_actors.get_mut("fps").unwrap();
        fps.text = format!("FPS: {:.1}", 1.0 / game_state.delta_seconds);
    }

    let msg2 = game_state.text_actors.get_mut("msg2").unwrap();
    msg2.translation.x = 75.0 * (game_state.seconds_since_startup * 0.5).sin() as f32;
    msg2.translation.y = 75.0 * (game_state.seconds_since_startup * 0.5).cos() as f32 - 200.0;

    let msg3 = game_state.text_actors.get_mut("msg3").unwrap();
    msg3.font_size = 10.0 * (game_state.seconds_since_startup * 0.5).cos() as f32 + 20.0;
}