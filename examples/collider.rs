//! To run this code in your own project, first install it:
//!
//!     cargo install rusty_engine --example collider
//!
//! Then run it in your own project (with the asset pack present) with an image
//! placed somewhere in your `assets/` directory.
//!
//!     collider assets/some_image.png
//!
//! Of course, you could also clone the rusty_engine repository, place your image
//! in `assets/` somewhere, and run:
//!
//! cargo run --release --example collider assets/some_image.png

use std::path::PathBuf;

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    circle_radius: f32,
    scale: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            circle_radius: 16.0,
            scale: 1.0,
        }
    }
}

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
    // We need an image file to work with, so the user must pass in the path of an image
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        println!(
            "Please pass in the path of an image inside the `assets/` directory! For example:\n\
            cargo run --release --example collider assets/sprite/racing/car_green.png"
        );
        std::process::exit(1);
    }

    // Remove "./" or ".\" from the start of the argument if necessary.
    let mut path: PathBuf = args[0]
        .as_str()
        .trim_start_matches("./")
        .trim_start_matches(r".\")
        .into();

    // If the user passed in `assets/something...` then we need to strip `assets/` (the asset loader will prepend `assets/`)
    if path.starts_with("assets") {
        path = path.strip_prefix("assets").unwrap().to_path_buf();
    }
    if !(PathBuf::from("assets").join(&path)).exists() {
        println!("Couldn't find the file {}", path.to_string_lossy());
        std::process::exit(1);
    }

    // Start with the "game" part
    let mut game = Game::new();
    game.show_colliders = true;
    game.window_settings(Window {
        title: "Collider Creator".into(),
        ..Default::default()
    });
    let _ = game.add_sprite("sprite", path);

    // Print instructions to the console
    println!("\n\
    Collider Instructions:\n\
    \n\
    1-9: Set Zoom level (sprite scale) to this amount.\n\
    Del/Backspace: Delete existing collider.*\n\
    Mouse Click: Add a collider point. Add points in a CLOCKWISE direction. Must be a CONVEX polygon to work correctly!\n\
    - Hold SHIFT while clicking the mouse to change the LAST point added.\n\
    c: Generate a circle collider at the current radius (radius defaults to 16.0)*\n\
    +: Increase the radius by 0.5 and generate a circle collider*\n\
    -: Decrease the radius by 0.5 and generate a circle collider*\n\
    w: Write the collider file. NOTE: This will overwrite the existing collider file (if any), so make a backup if you need the old one!\n\
    \n\
    *This command deletes the current collider in memory, but only writing the collider file will affect the collider file on disk.");

    // Tell the user to look to the console for the instructions
    let msg = game.add_text("msg", "See console output for instructions.");
    msg.translation = Vec2::new(0.0, -325.0);

    // Text to let the user know whether or not their polygon is convex
    let convex = game.add_text("convex", "???");
    convex.translation = Vec2::new(0.0, 325.0);

    game.add_logic(game_logic);
    game.run(Default::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Zoom levels
    if engine.keyboard_state.just_pressed(KeyCode::Key1) {
        game_state.scale = 1.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key2) {
        game_state.scale = 2.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key3) {
        game_state.scale = 3.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key4) {
        game_state.scale = 4.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key5) {
        game_state.scale = 5.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key6) {
        game_state.scale = 6.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key7) {
        game_state.scale = 7.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key8) {
        game_state.scale = 8.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Key9) {
        game_state.scale = 9.0;
    }
    // Update scale
    let sprite = engine.sprites.get_mut("sprite").unwrap();
    sprite.scale = game_state.scale;

    // Rotate
    if engine.mouse_state.pressed(MouseButton::Right) {
        sprite.rotation += engine.delta_f32 * 6.0;
    }
    // Delete collider
    if engine.keyboard_state.just_pressed(KeyCode::Delete)
        || engine.keyboard_state.just_pressed(KeyCode::Back)
    {
        sprite.collider = Collider::NoCollider;
        sprite.collider_dirty = true;
    }
    // Modify a collider point
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(location) = engine.mouse_state.location() {
            let location = (((location / game_state.scale) * 2.0).round() * 0.5) * game_state.scale;
            if engine
                .keyboard_state
                .pressed_any(&[KeyCode::ShiftLeft, KeyCode::ShiftRight])
            {
                sprite.change_last_collider_point(location);
            } else {
                sprite.add_collider_point(location);
            }
        }
    }
    // Generate a circle collider
    if engine
        .keyboard_state
        .just_pressed_any(&[KeyCode::Plus, KeyCode::Equals, KeyCode::NumpadAdd])
    {
        game_state.circle_radius += 0.5;
    }
    if engine
        .keyboard_state
        .just_pressed_any(&[KeyCode::Minus, KeyCode::NumpadSubtract])
    {
        game_state.circle_radius -= 0.5;
    }
    if engine.keyboard_state.just_pressed_any(&[
        KeyCode::Plus,
        KeyCode::Equals,
        KeyCode::NumpadAdd,
        KeyCode::Minus,
        KeyCode::NumpadSubtract,
        KeyCode::C,
    ]) {
        sprite.collider = Collider::circle(game_state.circle_radius);
        sprite.collider_dirty = true;
    }
    // Let the user know whether or not their collider is currently convex
    let convex = engine.texts.get_mut("convex").unwrap();
    const CONVEX_MESSAGE: &str = "Convex!";
    const NOT_CONVEX_MESSAGE: &str = "Not a convex polygon. :-(";
    if sprite.collider.is_convex() {
        if convex.value != CONVEX_MESSAGE {
            convex.value = CONVEX_MESSAGE.into();
        }
    } else {
        if convex.value != NOT_CONVEX_MESSAGE {
            convex.value = NOT_CONVEX_MESSAGE.into();
        }
    }
    // Write the collider file
    if engine.keyboard_state.just_pressed(KeyCode::W) {
        if sprite.write_collider() {
            println!(
                "Successfully wrote the new collider file: {}",
                sprite.collider_filepath.to_string_lossy()
            );
        } else {
            eprintln!(
                "Error: unable to write the collider file: {}",
                sprite.collider_filepath.to_string_lossy()
            );
        }
    }
}
