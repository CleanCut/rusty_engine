use rusty_engine::prelude::*;

struct GameState {
    current_label: String,
    // Use an incrementing index (converted to a string) for the unique label of the actors
    // Start at 1 since the hard-coded initial actor is 0
    next_actor_index: u32,
    shift_pressed: bool,
    next_layer: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_label: "0".into(),
            next_actor_index: 1,
            shift_pressed: false,
            next_layer: 0.01,
        }
    }
}

rusty_engine::init!(GameState);

const MAX_LAYER: f32 = 900.0;

fn main() {
    let mut game = Game::new();

    println!(
        "
This example is a level creator that lets you place actors into a level, and then
generate the code you can copy-and-paste into a main.rs file to recreate that level.

Yes, it would be better to find a way to serialize/deserialize all the data to and
from a save file. But I haven't taken the time to do that. Feel free to open a pull
request contributing a nice feature like that! ;-)

Controls

Right / Left Click - Rotate actor by 45 degrees (add Shift to rotate by 1 degree)
Mousewheel - Scale actor by 10% (add Shift to scale by 1 percent)
Mouse location - Choose translation (location) of actor

Space - Place actor
Left/Up Arrow - Previous actor preset
Right/Down Arrow - Next actor preset

R - Reset actor to default scale & rotation
S - Print out status of current actor
Z - Print out Rust code of current level

"
    );

    let game_state = GameState::default();

    // Get our first actor onto the board
    let mut curr_actor = game.add_actor("0".to_string(), ActorPreset::RacingCarRed);
    //curr_actor.scale = 0.5;
    curr_actor.layer = MAX_LAYER;

    game.add_logic(logic);
    game.run(game_state);
}

fn logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {
    // Gather keyboard input
    let mut reset = false;
    let mut print_level = false;
    let mut print_status = false;
    let mut place_actor = false;
    let mut prev_preset = false;
    let mut next_preset = false;
    for keyboard_event in &engine_state.keyboard_events {
        if let KeyboardInput {
            scan_code: _,
            key_code: Some(key_code),
            state,
        } = keyboard_event
        {
            if *state == ElementState::Pressed {
                match key_code {
                    KeyCode::Z | KeyCode::Semicolon => {
                        print_level = true;
                    }
                    KeyCode::LShift | KeyCode::RShift => {
                        game_state.shift_pressed = true;
                    }
                    KeyCode::R | KeyCode::P => {
                        reset = true;
                    }
                    KeyCode::S | KeyCode::O => {
                        print_status = true;
                    }
                    KeyCode::Space | KeyCode::Back => {
                        place_actor = true;
                    }
                    KeyCode::Left | KeyCode::Up => {
                        prev_preset = true;
                    }
                    KeyCode::Right | KeyCode::Down => {
                        next_preset = true;
                    }
                    _ => {}
                }
            } else {
                match key_code {
                    KeyCode::LShift | KeyCode::RShift => {
                        game_state.shift_pressed = false;
                    }
                    _ => {}
                }
            }
        }
    }

    // Print out the level?
    if print_level {
        println!(
            "---------------\n\nuse rusty_engine::prelude::*;\n\nstruct GameState {{}}\n\nrusty_engine::init!(GameState);\n\nfn main() {{\n    let mut game = Game::new();\n"
        );
        for actor in engine_state.actors.values() {
            if actor.label == game_state.current_label {
                continue;
            }
            println!(
                "    let a = game.add_actor(\"{}\", ActorPreset::{:?}); a.translation = Vec2::new({:.1}, {:.1}); a.rotation = {:.8}; a.scale = {:.8}; a.layer = {:.8}; a.collision = true;",
                actor.label,
                actor.preset.unwrap(),
                actor.translation.x,
                actor.translation.y,
                actor.rotation,
                actor.scale,
                actor.layer,
            );
        }
        println!("\n    game.add_logic(logic);\n    game.run(GameState {{}});\n}}\n\nfn logic(engine_state: &mut EngineState, game_state: &mut GameState) -> bool {{\n    // Game Logic Goes Here\n    true\n}}")
    }

    // Handle current actor that has not yet been placed
    if let Some(actor) = engine_state.actors.get_mut(&game_state.current_label) {
        // Should we print out the status of the actor?
        if print_status {
            println!(
                "Actor Status:\n-----------\n{:?}\nt: ({:.1}, {:.1})\nr: {:.8}\ns: {:.8}",
                actor.preset.unwrap(),
                actor.translation.x,
                actor.translation.y,
                actor.rotation,
                actor.scale
            );
        }
        // Did the user ask for rotation scale to be reset?
        if reset {
            actor.rotation = 0.0;
            actor.scale = 1.0;
        }

        // Handle translation via mouse location
        for cursor_moved in &engine_state.mouse_location_events {
            actor.translation = cursor_moved.position;
        }
        // Handle rotation via mouse clicks
        for mouse_button_input in &engine_state.mouse_button_events {
            if mouse_button_input.state != ElementState::Pressed {
                break;
            }
            let rotate_amount = if game_state.shift_pressed {
                std::f32::consts::TAU / 360.0
            } else {
                std::f32::consts::FRAC_PI_4
            };
            match mouse_button_input.button {
                MouseButton::Left => actor.rotation += rotate_amount,
                MouseButton::Right => actor.rotation -= rotate_amount,
                _ => {}
            }
            println!("r: {:.8}", actor.rotation);
        }
        // Handle scale via mousewheel
        for mouse_wheel in &engine_state.mouse_wheel_events {
            let scale_amount = if game_state.shift_pressed { 0.01 } else { 0.1 };
            if mouse_wheel.y > 0.0 || mouse_wheel.x < 0.0 {
                actor.scale *= 1.0 + scale_amount;
            } else {
                actor.scale *= 1.0 - scale_amount;
            }
            actor.scale = actor.scale.clamp(0.1, 5.0);
            println!("s: {:.8}", actor.scale);
        }
    }

    // Change actor to prev/next preset
    if prev_preset || next_preset {
        let old_actor = {
            engine_state
                .actors
                .get_mut(&game_state.current_label)
                .unwrap()
                .clone()
        };
        let new_preset = {
            if prev_preset {
                old_actor.preset.unwrap().prev()
            } else {
                old_actor.preset.unwrap().next()
            }
        };

        let new_label = game_state.next_actor_index.to_string();
        game_state.next_actor_index += 1;
        let mut new_actor = new_preset.build(new_label.clone());

        game_state.current_label = new_label;
        new_actor.layer = MAX_LAYER;
        new_actor.translation = old_actor.translation;
        new_actor.rotation = old_actor.rotation;
        new_actor.scale = old_actor.scale;
        engine_state
            .actors
            .insert(new_actor.label.clone(), new_actor);
        engine_state.actors.remove::<str>(old_actor.label.as_ref());
        println!("{:?}", new_preset);
    }

    // Place an actor
    if place_actor {
        let mut actor = {
            engine_state
                .actors
                .get_mut(&game_state.current_label)
                .unwrap()
                .clone()
        };
        actor.layer = game_state.next_layer;
        game_state.next_layer += 0.01;
        actor.label = game_state.next_actor_index.to_string();
        game_state.next_actor_index += 1;
        engine_state.actors.insert(actor.label.clone(), actor);
    }
    true
}
