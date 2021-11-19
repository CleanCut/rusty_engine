use rusty_engine::prelude::*;

fn place_barrier(game: &mut Game, prefix: String, location: Vec2) {
    for x in 0..5 {
        for y in 0..3 {
            if y == 2 && (x == 0 || x == 4) {
                continue;
            }
            let block = game.add_actor(
                format!("{}-{}-{}", prefix, x, y),
                ActorPreset::RollingBlockSmall,
            );
            block.translation = Vec2::new(x as f32 * 32.0, y as f32 * 32.0) + location;
        }
    }
}

fn main() {
    let mut game = Game::new();

    // create and position the player
    let player = game.add_actor("player", ActorPreset::RollingBlockCorner);
    player.translation.y = -335.0;
    player.rotation = SOUTH_WEST;
    player.scale = 0.75;
    player.collision = true;

    for i in 0..5 {
        place_barrier(
            &mut game,
            format!("barrier{}", i),
            Vec2::new(-624.0 + (i * (160 + 96)) as f32 + 32.0, -230.0),
        );
    }
    // pre-populate laser labels
    for i in 0..2 {
        game.game_state_mut().string_vec.push(format!("laser{}", i));
    }

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    // Player movement
    let player = game_state.actors.get_mut("player").unwrap();
    if let Some(location) = game_state.mouse_state.location() {
        player.translation.x = player
            .translation
            .lerp(Vec2::new(location.x, player.translation.y), 0.1)
            .x;
    }

    // Lasers!!!
    let player_translation = player.translation;
    if game_state.mouse_state.just_pressed(MouseButton::Left)
        || game_state.keyboard_state.just_pressed(KeyCode::Space)
    {
        if let Some(label) = game_state.string_vec.pop() {
            let laser =
                game_state.add_actor(format!("laser{}", label), ActorPreset::RacingBarrierWhite);
            laser.rotation = UP;
            laser.scale = 0.25;
            laser.translation = player_translation;
        }
    }
}
