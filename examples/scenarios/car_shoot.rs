use rand::prelude::*;
use rusty_engine::prelude::*;
use ActorPreset::*;

fn main() {
    let mut game = Game::new();

    // Create the player
    let player = game.add_actor("player", RacingBarrierRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    // Set the Window Settings
    game.window_settings(WindowDescriptor {
        title: "Car Shooter".into(),
        ..Default::default()
    });

    // Music!
    game.game_state_mut()
        .audio_manager
        .play_music(MusicPreset::Classy8Bit, 0.1);

    // Marbles left
    for i in 0..3 {
        game.game_state_mut()
            .string_vec
            .push(format!("marble{}", i));
    }

    // Cars left in level
    for i in 0..25 {
        game.game_state_mut().u32_vec.push(i);
    }
    let cars_left = game.add_text_actor("cars left", "Cars left: 25");
    cars_left.translation = Vec2::new(540.0, -320.0);

    game.run(game_logic);
}

const MARBLE_SPEED: f32 = 600.0;
const CAR_SPEED: f32 = 300.0;

fn game_logic(game_state: &mut GameState) {
    // Handle marble gun movement
    for event in game_state.mouse_location_events.drain(..) {
        let player = game_state.actors.get_mut("player").unwrap();
        player.translation.x = event.position.x;
    }

    // Shoot marbles!
    for event in game_state.mouse_button_events.clone() {
        if !matches!(event.state, ElementState::Pressed) {
            continue;
        }
        // Create the marble
        if let Some(label) = game_state.string_vec.pop() {
            let player_x = game_state.actors.get_mut("player").unwrap().translation.x;
            let marble = game_state.add_actor(label, RollingBallBlue);
            marble.translation.y = -275.0;
            marble.translation.x = player_x;
            marble.layer = 5.0;
            marble.collision = true;
            game_state.audio_manager.play_sfx(SfxPreset::Impact2, 0.7);
        }
    }

    // Move marbles
    for marble in game_state
        .actors
        .values_mut()
        .filter(|marble| marble.label.starts_with("marble"))
    {
        marble.translation.y += MARBLE_SPEED * game_state.delta_f32;
    }

    // Clean up actors that have gone off the screen
    let mut labels_to_delete = vec![];
    for actor in game_state.actors.values_mut() {
        if actor.translation.y > 400.0 || actor.translation.x > 750.0 {
            labels_to_delete.push(actor.label.clone());
        }
    }
    for label in labels_to_delete {
        game_state.actors.remove(&label);
        if label.starts_with("marble") {
            game_state.string_vec.push(label);
        }
    }

    // Move cars across the screen
    for car in game_state
        .actors
        .values_mut()
        .filter(|car| car.label.starts_with("car"))
    {
        car.translation.x += CAR_SPEED * game_state.delta_f32;
    }

    // Spawn cars
    let spawn_timer = game_state
        .timer_map
        .entry("spawn_timer".into())
        .or_insert(Timer::from_seconds(0.0, false));
    if spawn_timer.tick(game_state.delta).just_finished() {
        // Reset the timer to a new value
        *spawn_timer = Timer::from_seconds(thread_rng().gen_range(0.1..1.25), false);
        // Get the next car
        if let Some(i) = game_state.u32_vec.pop() {
            let cars_left = game_state.text_actors.get_mut("cars left").unwrap();
            cars_left.text = format!("Cars left: {}", i);
            let label = format!("car{}", i);
            let car_choices = vec![
                RacingCarBlack,
                RacingCarBlue,
                RacingCarGreen,
                RacingCarRed,
                RacingCarYellow,
            ];
            let car = game_state.add_actor(
                label,
                car_choices
                    .iter()
                    .choose(&mut thread_rng())
                    .unwrap()
                    .clone(),
            );
            car.translation.x = -740.0;
            car.translation.y = thread_rng().gen_range(-100.0..325.0);
            car.collision = true;
        }
    }

    // Collide with things
    for event in game_state.collision_events.drain(..) {
        if event.state.is_end() {
            continue;
        }
        if !event.pair.one_starts_with("marble") {
            // it's two cars spawning on top of each other, take one out
            game_state.actors.remove(&event.pair.0);
            continue;
        }
        game_state.actors.remove(&event.pair.0);
        game_state.actors.remove(&event.pair.1);
        if event.pair.0.starts_with("marble") {
            game_state.string_vec.push(event.pair.0);
        }
        if event.pair.1.starts_with("marble") {
            game_state.string_vec.push(event.pair.1);
        }
        game_state
            .audio_manager
            .play_sfx(SfxPreset::Confirmation1, 0.5);
    }
}
