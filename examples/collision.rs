use rusty_engine::prelude::*;

rusty_engine::init!();

const ROTATION_SPEED: f32 = 3.0;

fn main() {
    let mut game = Game::new();

    let msg2 = game.add_text_actor(
        "instructions",
        "Move the car around with your mouse. Rotate it by holding left/right mouse buttons. Scale it with the mousewheel.",
    );
    msg2.font_size = 20.0;
    msg2.translation.y = 340.0;

    let mut race_car = game.add_actor("Player", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.layer = 100.0;
    race_car.collision = true;

    let mut actor_presets_iter = ActorPreset::variant_iter().peekable();
    'outer: for y in (-265..=400).step_by(175) {
        for x in (-550..=550).step_by(275) {
            if actor_presets_iter.peek().is_none() {
                break 'outer;
            }
            let actor_preset = actor_presets_iter.next().unwrap();
            let mut actor = game.add_actor(format!("{:?}", actor_preset), actor_preset);
            actor.translation = Vec2::new(x as f32, (-y) as f32);
            actor.collision = true;
        }
    }

    let mut text_actor = game.add_text_actor("collision text", "");
    text_actor.translation = Vec2::new(0.0, -200.0);

    game.add_logic(logic);
    game.run(());
}

fn logic(engine_state: &mut EngineState, _: &mut ()) -> bool {
    // If a collision event happened last frame, print it out and play a sound
    for event in engine_state.collision_events.drain(..) {
        let text_actor = engine_state.text_actors.get_mut("collision text").unwrap();
        match event.state {
            CollisionState::Begin => {
                text_actor.text = format!("{:?}", event.pair);
                engine_state.audio_manager.play_sfx(SfxPreset::Switch1)
            }
            CollisionState::End => {
                text_actor.text = "".into();
                engine_state.audio_manager.play_sfx(SfxPreset::Switch2)
            }
        }
    }

    if let Some(actor) = engine_state.actors.get_mut("Player") {
        // Use the latest state of the mouse buttons to rotate the actor
        let mut rotation_amount = 0.0;
        if engine_state.mouse_state.pressed(MouseButton::Left) {
            rotation_amount += ROTATION_SPEED * engine_state.delta_f32;
        }
        if engine_state.mouse_state.pressed(MouseButton::Right) {
            rotation_amount -= ROTATION_SPEED * engine_state.delta_f32;
        }
        actor.rotation += rotation_amount;

        // Use the latest state of the mouse wheel to scale the actor
        if let Some(location) = engine_state.mouse_state.location() {
            actor.translation = location
        }

        // Mousewheel scales the car
        for mouse_wheel in &engine_state.mouse_wheel_events {
            actor.scale *= 1.0 + (0.05 * mouse_wheel.y);
            actor.scale = actor.scale.clamp(0.1, 4.0);
        }
    }
    true
}
