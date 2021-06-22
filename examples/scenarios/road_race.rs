use rusty_engine::prelude::*;

const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();
    // setup goes here
    let player1 = game.add_actor("player1", ActorPreset::RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 100.0;
    player1.collision = true;
    game.game_state_mut()
        .audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);
    for i in 0..10 {
        let roadline = game.add_actor(format!("roadline{}", i), ActorPreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }
    game.run(game_logic);
}

fn game_logic(game_state: &mut GameState) {
    // Direction player1 is moving vertically. 1 is up, 0 is not moving, -1 is down.
    let direction = game_state.i32_map.entry("direction".into()).or_insert(0);
    // Respond to keyboard events and set the deriction
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
    }

    // Move road objects
    for actor in game_state.actors.values_mut() {
        if actor.label.starts_with("roadline") {
            actor.translation.x -= ROAD_SPEED * game_state.delta_seconds;
            if actor.translation.x < -675.0 {
                actor.translation.x += 1500.0;
            }
        }
    }
}
