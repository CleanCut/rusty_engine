use rusty_engine::prelude::*;

#[derive(Default)]
struct GameState {
    laser_labels: Vec<String>,
}

fn main() {
    let mut game = Game::new();

    // create and position the player
    let player = game.add_sprite("player", SpritePreset::RollingBlockCorner);
    player.translation.y = -335.0;
    player.rotation = SOUTH_WEST;
    player.scale = 0.75;
    player.collision = true;

    //car.translation.y = car.translation.y.clamp(-360.0, 360.0);

    for i in 0..5 {
        place_barrier(
            &mut game,
            format!("barrier{}", i),
            Vec2::new(-624.0 + (i * (160 + 96)) as f32 + 32.0, -230.0),
        );
    }
    // pre-populate laser labels
    let mut game_state = GameState::default();
    for i in 0..2 {
        game_state.laser_labels.push(format!("laser{}", i));
    }

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Player movement
    let player = engine.sprites.get_mut("player").unwrap();
    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = player
            .translation
            .lerp(Vec2::new(location.x, player.translation.y), 0.1)
            .x;
    }

    // Lasers!!!
    let player_translation = player.translation;
    if engine.mouse_state.just_pressed(MouseButton::Left)
        || engine.keyboard_state.just_pressed(KeyCode::Space)
    {
        if let Some(label) = game_state.laser_labels.pop() {
            let laser =
                engine.add_sprite(format!("laser{}", label), SpritePreset::RacingBarrierWhite);
            laser.rotation = UP;
            laser.scale = 0.25;
            laser.translation = player_translation;
        }
    }
}

fn place_barrier(game: &mut Game<GameState>, prefix: String, location: Vec2) {
    for x in 0..5 {
        for y in 0..3 {
            if y == 2 && (x == 0 || x == 4) {
                continue;
            }
            let block = game.add_sprite(
                format!("{}-{}-{}", prefix, x, y),
                SpritePreset::RollingBlockSmall,
            );
            block.translation = Vec2::new(x as f32 * 32.0, y as f32 * 32.0) + location;
        }
    }
}
