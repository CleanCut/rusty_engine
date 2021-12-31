use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

rusty_engine::init!(GameState);

fn main() {
    let mut game = Game::new();
    let game_state = GameState {
        high_score: 2345,
        current_score: 0,
        enemy_labels: Vec::new(),
        spawn_timer: Timer::from_seconds(10.0, false),
    };
    game.add_logic(game_logic); // Don't forget to add the logic function to the game!
    game.run(game_state);
}

fn game_logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    game_state.current_score += 1;
    println!("Current score: {}", game_state.current_score);
    true
}
