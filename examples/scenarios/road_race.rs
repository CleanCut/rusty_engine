use rand::prelude::*;
use rusty_engine::prelude::*;
use ActorPreset::*; // The ActorPreset enum was imported from rusty_engine::prelude

const ROAD_SPEED: f32 = 400.0;
const PLAYER_SPEED: f32 = 250.0;

struct GameState {
    health_amount: u8,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();

    // Start some background music
    game.audio_manager
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

    // Create the health message
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_ends);
    game.add_logic(game_logic);

    // Run the game, which will run our game logic functions once every frame
    game.run(GameState { health_amount: 5 });
}

fn game_ends(_: &mut EngineState, game_state: &mut GameState) -> bool {
    // Don't continue to game logic if the game has ended
    game_state.health_amount > 0
}

fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    // Respond to keyboard events and set the direction
    let mut direction = 0;
    if engine_state
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W, KeyCode::Comma])
    {
        direction += 1;
    }
    if engine_state
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S, KeyCode::O])
    {
        direction -= 1;
    }

    // Move player1
    if let Some(player1) = engine_state.actors.get_mut("player1") {
        if direction > 0 {
            player1.translation.y += PLAYER_SPEED * engine_state.delta_f32;
            player1.rotation = 0.15;
        } else if direction < 0 {
            player1.translation.y -= PLAYER_SPEED * engine_state.delta_f32;
            player1.rotation = -0.15;
        } else {
            player1.rotation = 0.0;
        }
        if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
            game_state.health_amount = 0;
        }
    }

    // Move road objects
    for actor in engine_state.actors.values_mut() {
        if actor.label.starts_with("roadline") {
            actor.translation.x -= ROAD_SPEED * engine_state.delta_f32;
            if actor.translation.x < -675.0 {
                actor.translation.x += 1500.0;
            }
        }
        if actor.label.starts_with("obstacle") {
            actor.translation.x -= ROAD_SPEED * engine_state.delta_f32;
            if actor.translation.x < -800.0 {
                actor.translation.x = thread_rng().gen_range(800.0..1600.0);
                actor.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Deal with collisions
    let health_message = engine_state.texts.get_mut("health_message").unwrap();
    for event in engine_state.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine_state.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }
    if game_state.health_amount == 0 {
        let game_over = engine_state.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine_state.audio_manager.stop_music();
        engine_state.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }

    true
}
