use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut race_car = game.add_actor("Race Car", ActorPreset::RacingCarGreen);
    race_car.translation = Vec2::new(0.0, 0.0);
    race_car.rotation = UP;
    race_car.layer = 100.0;
    race_car.scale = 1.0;
    race_car.collision = true;
    race_car.collider = Collider::rect(Vec2::new(-60.0, 35.0), Vec2::new(60.0, -35.0));
    // race_car.collider = Collider::circle(15.0);

    let mut actor = game.add_actor("bluebarrel1", ActorPreset::RacingBarrelBlue);
    actor.translation = Vec2::new(-400.0, 200.0);
    actor.collision = true;
    // actor.collider = Collider::circle(15.0);
    actor.collider = Collider::rect(Vec2::new(-28.0, 28.0), Vec2::new(28.0, -28.0));

    let mut actor = game.add_actor("redbarrel1", ActorPreset::RacingBarrelRed);
    actor.translation = Vec2::new(400.0, 200.0);
    actor.collision = true;
    // actor.collider = Collider::circle(15.0);
    actor.collider = Collider::rect(Vec2::new(-28.0, 28.0), Vec2::new(28.0, -28.0));

    let mut actor = game.add_actor("bluebarrel2", ActorPreset::RacingBarrelBlue);
    actor.translation = Vec2::new(400.0, -200.0);
    actor.collision = true;
    // actor.collider = Collider::circle(15.0);
    actor.collider = Collider::rect(Vec2::new(-28.0, 28.0), Vec2::new(28.0, -28.0));

    let mut actor = game.add_actor("redbarrel2", ActorPreset::RacingBarrelRed);
    actor.translation = Vec2::new(-400.0, -200.0);
    actor.collision = true;
    // actor.collider = Collider::circle(15.0);
    actor.collider = Collider::rect(Vec2::new(-28.0, 28.0), Vec2::new(28.0, -28.0));

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    for event in game_state.collision_events.drain(..) {
        println!("{:?}", event);
        match event.state {
            CollisionState::Begin => game_state.audio_manager.play_sfx(SfxPreset::Click),
            CollisionState::End => game_state.audio_manager.play_sfx(SfxPreset::Switch1),
        }
    }

    if let Some(actor) = game_state.actors.get_mut("Race Car") {
        for mouse_button_input in &game_state.mouse_button_events {
            if mouse_button_input.state != ElementState::Pressed {
                break;
            }
            match mouse_button_input.button {
                MouseButton::Left => actor.rotation += std::f32::consts::FRAC_PI_4,
                MouseButton::Right => actor.rotation -= std::f32::consts::FRAC_PI_4,
                _ => {}
            }
        }
        for cursor_moved in &game_state.cursor_moved_events {
            actor.translation = cursor_moved.position;
        }
        for mouse_wheel in &game_state.mouse_wheel_events {
            if mouse_wheel.y > 0.0 {
                actor.scale *= 1.1;
            } else {
                actor.scale *= 0.9;
            }
            actor.scale = actor.scale.clamp(0.1, 3.0);
        }
    }
}
