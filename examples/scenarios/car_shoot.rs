//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example car_shoot

use rand::prelude::*;
use rusty_engine::prelude::*;
use SpritePreset::*; // The SpritePreset enum was imported from rusty_engine::prelude

rusty_engine::init!(GameState);

#[derive(Default)]
struct GameState {
    marbles_left: Vec<String>,
    cars_left: i32,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();

    // Set the title of the window to be Car Shooter
    game.window_settings(WindowDescriptor {
        title: "Car Shoot".into(),
        ..Default::default()
    });

    // Create the player
    let player = game.add_sprite("player", RacingBarrierRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    // Start the music
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    let mut game_state = GameState::default();

    // Marbles left. We'll use these strings as labels for sprites. If they are present in the
    // vector, then they are available to be shot out of the marble gun. If they are not present,
    // then they are currently in play.
    game_state.marbles_left = vec!["marble1".into(), "marble2".into(), "marble3".into()];

    // Cars left in level - each integer represents a car that will be spawned
    game_state.cars_left = 25;
    let cars_left = game.add_text("cars left", "Cars left: 25");
    cars_left.translation = Vec2::new(540.0, -320.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

const MARBLE_SPEED: f32 = 600.0;
const CAR_SPEED: f32 = 300.0;

fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    // Handle marble gun movement
    let player = engine_state.sprites.get_mut("player").unwrap();
    if let Some(location) = engine_state.mouse_state.location() {
        player.translation.x = location.x;
    }
    let player_x = player.translation.x;

    // Shoot marbles!
    if engine_state.mouse_state.just_pressed(MouseButton::Left) {
        // Create the marble
        if let Some(label) = game_state.marbles_left.pop() {
            let marble = engine_state.add_sprite(label, RollingBallBlue);
            marble.translation.y = -275.0;
            marble.translation.x = player_x;
            marble.layer = 5.0;
            marble.collision = true;
            engine_state.audio_manager.play_sfx(SfxPreset::Impact2, 0.7);
        }
    }

    // Move marbles
    for marble in engine_state
        .sprites
        .values_mut()
        .filter(|marble| marble.label.starts_with("marble"))
    {
        marble.translation.y += MARBLE_SPEED * engine_state.delta_f32;
    }

    // Move cars across the screen
    for car in engine_state
        .sprites
        .values_mut()
        .filter(|car| car.label.starts_with("car"))
    {
        car.translation.x += CAR_SPEED * engine_state.delta_f32;
    }

    // Clean up sprites that have gone off the screen
    let mut labels_to_delete = vec![];
    for sprite in engine_state.sprites.values() {
        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            labels_to_delete.push(sprite.label.clone());
        }
    }
    for label in labels_to_delete {
        engine_state.sprites.remove(&label);
        if label.starts_with("marble") {
            game_state.marbles_left.push(label);
        }
    }

    // Spawn cars
    if game_state
        .spawn_timer
        .tick(engine_state.delta)
        .just_finished()
    {
        // Reset the timer to a new value
        game_state.spawn_timer = Timer::from_seconds(thread_rng().gen_range(0.1..1.25), false);
        // Get the next car
        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;
            let label = format!("car{}", game_state.cars_left);
            let cars_left_text = engine_state.texts.get_mut("cars left").unwrap();
            cars_left_text.value = format!("Cars left: {}", game_state.cars_left);
            let car_choices = vec![
                RacingCarBlack,
                RacingCarBlue,
                RacingCarGreen,
                RacingCarRed,
                RacingCarYellow,
            ];
            let car = engine_state.add_sprite(
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
    for event in engine_state.collision_events.drain(..) {
        if event.state.is_end() {
            continue;
        }
        if !event.pair.one_starts_with("marble") {
            continue;
        }
        engine_state.sprites.remove(&event.pair.0);
        engine_state.sprites.remove(&event.pair.1);
        if event.pair.0.starts_with("marble") {
            game_state.marbles_left.push(event.pair.0);
        }
        if event.pair.1.starts_with("marble") {
            game_state.marbles_left.push(event.pair.1);
        }
        engine_state
            .audio_manager
            .play_sfx(SfxPreset::Confirmation1, 0.5);
    }

    true
}
