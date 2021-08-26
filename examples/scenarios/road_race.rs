use rand::prelude::*;
use rusty_engine::prelude::{ActorPreset::*, *};

const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();

    // Play some background music
    game.game_state_mut()
        .audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Create the player actor
    let player1 = game.add_actor("player1", RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 100.0;
    player1.collision = true;

    // Create the road line actors
    for i in 0..10 {
        let roadline = game.add_actor(format!("roadline{}", i), ActorPreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Create the obstacle actors
    let obstacle_presets = vec![
        RacingBarrelBlue,
        RacingBarrelRed,
        RacingBarrelRed,
        RacingConeStraight,
        RacingConeStraight,
        RacingConeStraight,
        RollingBlockCorner,
        RollingBlockSquare,
        RollingBlockSmall,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let actor = game.add_actor(format!("obstacle{}", i), preset);
        actor.layer = 50.0;
        actor.collision = true;
        actor.translation.x = thread_rng().gen_range(800.0..1600.0);
        actor.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // Create the health amount and health message
    game.game_state_mut()
        .u8_map
        .insert("health_amount".into(), 5);
    let health_message = game.add_text_actor("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    // Run the game, which will run our game logic once every frame
    game.run(game_logic);
}

fn game_logic(game_state: &mut GameState) {
    // Pause if we have already lost
    let health_amount = game_state.u8_map.get_mut("health_amount").unwrap();
    if *health_amount == 0 {
        return;
    }

    // Direction player1 is moving vertically. 1 is up, 0 is not moving, -1 is down.
    let direction = game_state.i32_map.entry("direction".into()).or_insert(0);

    // Respond to keyboard events and set the direction
    for event in game_state.keyboard_events.drain(..) {
        match event.state {
            ElementState::Pressed => {
                if let Some(key_code) = event.key_code {
                    match key_code {
                        KeyCode::Up => *direction = 1,
                        KeyCode::Down => *direction = -1,
                        _ => {}
                    }
                }
            }
            ElementState::Released => *direction = 0,
        }
    }

    // Move player1
    let speed = 250.0;
    if let Some(player1) = game_state.actors.get_mut("player1") {
        if *direction > 0 {
            player1.translation.y += speed * game_state.delta_seconds;
            player1.rotation = 0.15;
        } else if *direction < 0 {
            player1.translation.y -= speed * game_state.delta_seconds;
            player1.rotation = -0.15;
        } else {
            player1.rotation = 0.0;
        }
        if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
            *health_amount = 0;
        }
    }

    // Move road objects
    for actor in game_state.actors.values_mut() {
        if actor.label.starts_with("roadline") {
            actor.translation.x -= ROAD_SPEED * game_state.delta_seconds;
            if actor.translation.x < -675.0 {
                actor.translation.x += 1500.0;
            }
        }
        if actor.label.starts_with("obstacle") {
            actor.translation.x -= ROAD_SPEED * game_state.delta_seconds;
            if actor.translation.x < -800.0 {
                actor.translation.x = thread_rng().gen_range(800.0..1600.0);
                actor.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Deal with collisions
    let health_message = game_state.text_actors.get_mut("health_message").unwrap();
    for event in game_state.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if *health_amount > 0 {
            *health_amount -= 1;
            health_message.text = format!("Health: {}", *health_amount);
            game_state.audio_manager.play_sfx(SfxPreset::Impact3);
        }
    }
    if *health_amount == 0 {
        let game_over = game_state.add_text_actor("game over", "Game Over");
        game_over.font_size = 128.0;
        game_state.audio_manager.stop_music();
        game_state.audio_manager.play_sfx(SfxPreset::Jingle3);
    }
}
