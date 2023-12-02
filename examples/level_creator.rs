//! To run this code in your own project, first install it:
//!
//!     cargo install rusty_engine --example level_creator
//!
//! Then run it in your own project (with the asset pack present).
//!
//!     level_creator
//!
//! Alterantely, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example level_creator

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    current_label: String,
    // Use an incrementing index (converted to a string) for the unique label of the sprites
    // Start at 1 since the hard-coded initial sprite is 0
    next_sprite_num: u32,
    shift_pressed: bool,
    next_layer: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_label: "0".into(),
            next_sprite_num: 1,
            shift_pressed: false,
            next_layer: 0.01,
        }
    }
}

const MAX_LAYER: f32 = 900.0;

fn main() {
    // Some trickiness to make assets load relative to the current working directory, which
    // makes using it from `cargo install rusty_engine --example collider` possible.
    // This takes advantage of bevy's hard-coded asset loading behavior, and may break in future
    // bevy versions.
    std::env::set_var(
        "CARGO_MANIFEST_DIR",
        std::env::var("PWD").unwrap_or_default(),
    );
    // Make engine logging a bit quieter since we've got console instructions we want folks to see.
    std::env::set_var("RUST_LOG", "error");

    let mut game = Game::new();

    println!(
        "
This example is a level creator that lets you place sprites into a level, and then
generate the code you can copy-and-paste into a main.rs file to recreate that level.

Yes, it would be better to find a way to serialize/deserialize all the data to and
from a save file. But I haven't taken the time to do that. Feel free to open a pull
request contributing a nice feature like that! ;-)

Controls

Right / Left Click - Rotate sprite by 45 degrees (add Shift to rotate by 1 degree)
Mousewheel - Scale sprite by 10% (add Shift to scale by 1 percent)
Mouse location - Choose translation (location) of sprite

Space - Place sprite
Left/Up Arrow - Previous sprite preset
Right/Down Arrow - Next sprite preset

R - Reset sprite to default scale & rotation
S - Print out status of current sprite
Z - Print out Rust code of current level

"
    );

    let game_state = GameState::default();

    // Get our first sprite onto the board
    let curr_sprite = game.add_sprite("0".to_string(), SpritePreset::RacingCarRed);
    //curr_sprite.scale = 0.5;
    curr_sprite.layer = MAX_LAYER;

    game.add_logic(logic);
    game.run(game_state);
}

fn logic(engine: &mut Engine, game_state: &mut GameState) {
    // Gather keyboard input
    let mut reset = false;
    let mut print_level = false;
    let mut print_status = false;
    let mut place_sprite = false;
    let mut prev_preset = false;
    let mut next_preset = false;
    for keyboard_event in &engine.keyboard_events {
        if let KeyboardInput {
            scan_code: _,
            key_code: Some(key_code),
            state,
            window: _,
        } = keyboard_event
        {
            if *state == ButtonState::Pressed {
                match key_code {
                    KeyCode::Z | KeyCode::Semicolon => {
                        print_level = true;
                    }
                    KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                        game_state.shift_pressed = true;
                    }
                    KeyCode::R | KeyCode::P => {
                        reset = true;
                    }
                    KeyCode::S | KeyCode::O => {
                        print_status = true;
                    }
                    KeyCode::Space | KeyCode::Back => {
                        place_sprite = true;
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
                    KeyCode::ShiftLeft | KeyCode::ShiftRight => {
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
            "---------------\n\nuse rusty_engine::prelude::*;\n\n#[derive(Resource)]\nstruct GameState {{}}\n\nfn main() {{\n    let mut game = Game::new();\n"
        );
        for sprite in engine.sprites.values() {
            if sprite.label == game_state.current_label {
                continue;
            }
            println!(
                "    let a = game.add_sprite(\"{}\", \"{}\"); a.translation = Vec2::new({:.1}, {:.1}); a.rotation = {:.8}; a.scale = {:.8}; a.layer = {:.8}; a.collision = true;",
                sprite.label,
                sprite.filepath.to_string_lossy(),
                sprite.translation.x,
                sprite.translation.y,
                sprite.rotation,
                sprite.scale,
                sprite.layer,
            );
        }
        println!("\n    game.add_logic(logic);\n    game.run(GameState {{}});\n}}\n\nfn logic(engine: &mut Engine, game_state: &mut GameState) {{\n    // Game Logic Goes Here\n}}")
    }

    // Handle current sprite that has not yet been placed
    if let Some(sprite) = engine.sprites.get_mut(&game_state.current_label) {
        // Should we print out the status of the sprite?
        if print_status {
            println!(
                "Sprite Status:\n-----------\n{:?}\nt: ({:.1}, {:.1})\nr: {:.8}\ns: {:.8}",
                sprite.filepath.to_string_lossy(),
                sprite.translation.x,
                sprite.translation.y,
                sprite.rotation,
                sprite.scale
            );
        }
        // Did the user ask for rotation scale to be reset?
        if reset {
            sprite.rotation = 0.0;
            sprite.scale = 1.0;
        }

        // Handle translation via mouse location
        for cursor_moved in &engine.mouse_location_events {
            sprite.translation = cursor_moved.position;
        }
        // Handle rotation via mouse clicks
        for mouse_button_input in &engine.mouse_button_events {
            if mouse_button_input.state != ButtonState::Pressed {
                break;
            }
            let rotate_amount = if game_state.shift_pressed {
                std::f32::consts::TAU / 360.0
            } else {
                std::f32::consts::FRAC_PI_4
            };
            match mouse_button_input.button {
                MouseButton::Left => sprite.rotation += rotate_amount,
                MouseButton::Right => sprite.rotation -= rotate_amount,
                _ => {}
            }
            println!("r: {:.8}", sprite.rotation);
        }
        // Handle scale via mousewheel
        for mouse_wheel in &engine.mouse_wheel_events {
            let scale_amount = if game_state.shift_pressed { 0.01 } else { 0.1 };
            if mouse_wheel.y > 0.0 || mouse_wheel.x < 0.0 {
                sprite.scale *= 1.0 + scale_amount;
            } else {
                sprite.scale *= 1.0 - scale_amount;
            }
            sprite.scale = sprite.scale.clamp(0.1, 5.0);
            println!("s: {:.8}", sprite.scale);
        }
    }

    // Change sprite to prev/next preset
    if prev_preset || next_preset {
        let old_sprite = {
            engine
                .sprites
                .get_mut(&game_state.current_label)
                .unwrap()
                .clone()
        };
        let (idx, _) = SpritePreset::variant_iter()
            .enumerate()
            .find(|(_, preset)| preset.filepath() == old_sprite.filepath)
            .unwrap();
        let new_idx = if next_preset {
            (idx + 1) % SpritePreset::variant_iter().count()
        } else {
            if idx == 0 {
                SpritePreset::variant_iter().count() - 1
            } else {
                idx - 1
            }
        };
        let new_preset = SpritePreset::variant_iter().nth(new_idx).unwrap();

        let new_label = game_state.next_sprite_num.to_string();
        game_state.next_sprite_num += 1;
        let mut new_sprite = Sprite::new(new_label.clone(), new_preset);

        game_state.current_label = new_label;
        new_sprite.layer = MAX_LAYER;
        new_sprite.translation = old_sprite.translation;
        new_sprite.rotation = old_sprite.rotation;
        new_sprite.scale = old_sprite.scale;
        engine.sprites.insert(new_sprite.label.clone(), new_sprite);
        engine.sprites.remove::<str>(old_sprite.label.as_ref());
        println!("{:?}", new_preset);
    }

    // Place an sprite
    if place_sprite {
        let mut sprite = {
            engine
                .sprites
                .get_mut(&game_state.current_label)
                .unwrap()
                .clone()
        };
        sprite.layer = game_state.next_layer;
        game_state.next_layer += 0.01;
        sprite.label = game_state.next_sprite_num.to_string();
        game_state.next_sprite_num += 1;
        engine.sprites.insert(sprite.label.clone(), sprite);
    }
}
