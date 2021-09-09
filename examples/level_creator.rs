use rusty_engine::prelude::*;

const MAX_LAYER: f32 = 900.0;

fn main() {
    println!(
        "
This is an example of how you could write a level creater. This example lets you place actors, and
generate the code you can copy-and-paste into a main.rs file to recreate that level.

Controls

R - Reset actor to default scale & rotation
S - Print out status of current actor
Z - Print out Rust code of current level

Left/Up - Previous actor preset
Right/Down - Next actor preset

Right / Left Click - Rotate actor by 45 degrees (add Shift to rotate by 1 degree)
Mousewheel - Scale actor by 10% (add Shift to scale by 1 percent)
Mouse location - Choose translation (location) of actor
"
    );

    let mut game = Game::new();

    // Use an incrementing index (converted to a string) for the unique label of the actors
    // Start at 1 since the hard-coded initial actor is 0
    game.game_state_mut().u32_vec.push(1);

    // Get our first actor onto the board
    let initial_label = "0".to_string();
    game.game_state_mut().string_vec.push(initial_label.clone());
    let mut curr_actor = game.add_actor(initial_label, ActorPreset::RacingCarRed);
    //curr_actor.scale = 0.5;
    curr_actor.layer = MAX_LAYER;

    // Use a bool to track whether or not the shift key is currently pressed
    game.game_state_mut().bool_vec.push(false);

    // Use an f32 to track the current layer (so newer actors will always be on top of older ones)
    game.game_state_mut().f32_vec.push(0.01);

    game.run(logic);
}

fn logic(game_state: &mut GameState) {
    // Extract values we're tracking
    let current_label = game_state.string_vec.get_mut(0).unwrap();
    let next_actor_index = game_state.u32_vec.get_mut(0).unwrap();
    let shift_pressed = game_state.bool_vec.get_mut(0).unwrap();
    let next_layer = game_state.f32_vec.get_mut(0).unwrap();

    // Gather keyboard input
    let mut reset = false;
    let mut print_level = false;
    let mut print_status = false;
    let mut place_actor = false;
    let mut prev_preset = false;
    let mut next_preset = false;
    for keyboard_event in &game_state.keyboard_events {
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
                        *shift_pressed = true;
                    }
                    KeyCode::R => {
                        reset = true;
                    }
                    KeyCode::S => {
                        print_status = true;
                    }
                    KeyCode::Space | KeyCode::Delete => {
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
                        *shift_pressed = false;
                    }
                    _ => {}
                }
            }
        }
    }

    // Print out the level?
    if print_level {
        println!(
            "---------------\n\nuse rusty_engine::prelude::*;\n\nfn main() {{\n    let mut game = Game::new();\n"
        );
        for actor in game_state.actors.values() {
            if actor.label == *current_label {
                continue;
            }
            println!(
                "    let a = game.game_state_mut().add_actor(\"{}\", ActorPreset::{:?}); a.translation = Vec2::new({:.1}, {:.1}); a.rotation = {:.8}; a.scale = {:.8}; a.layer = {:.8}; a.collision = true;",
                actor.label,
                actor.preset.unwrap(),
                actor.translation.x,
                actor.translation.y,
                actor.rotation,
                actor.scale,
                actor.layer,
            );
        }
        println!("\n    game.run(logic);\n}}\n\nfn logic(game_state: &mut GameState) {{\n    // Game Logic Goes Here\n}}")
    }

    // Handle current actor that has not yet been placed
    if let Some(actor) = game_state.actors.get_mut(current_label) {
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
        for cursor_moved in &game_state.mouse_location_events {
            actor.translation = cursor_moved.position;
        }
        // Handle rotation via mouse clicks
        for mouse_button_input in &game_state.mouse_button_events {
            if mouse_button_input.state != ElementState::Pressed {
                break;
            }
            let rotate_amount = if *shift_pressed {
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
        for mouse_wheel in &game_state.mouse_wheel_events {
            let scale_amount = if *shift_pressed { 0.01 } else { 0.1 };
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
        let old_actor = { game_state.actors.get_mut(current_label).unwrap().clone() };
        let new_preset = {
            if prev_preset {
                old_actor.preset.unwrap().prev()
            } else {
                old_actor.preset.unwrap().next()
            }
        };

        let new_label = next_actor_index.to_string();
        *next_actor_index += 1;
        let mut new_actor = new_preset.build(new_label.clone());

        *current_label = new_label;
        new_actor.layer = MAX_LAYER;
        new_actor.translation = old_actor.translation;
        new_actor.rotation = old_actor.rotation;
        new_actor.scale = old_actor.scale;
        game_state.actors.insert(new_actor.label.clone(), new_actor);
        game_state.actors.remove::<str>(old_actor.label.as_ref());
        println!("{:?}", new_preset);
    }

    // Place an actor
    if place_actor {
        let mut actor = { game_state.actors.get_mut(current_label).unwrap().clone() };
        actor.layer = *next_layer;
        *next_layer += 0.01;
        actor.label = next_actor_index.to_string();
        *next_actor_index += 1;
        game_state.actors.insert(actor.label.clone(), actor);
    }
}
