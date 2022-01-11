use bevy::prelude::{
    info, AssetServer, Color, Commands, HorizontalAlign, Res, ResMut, SpriteBundle,
    Text as BevyText, Text2dBundle, TextAlignment, TextStyle, Vec2, VerticalAlign, Windows,
};
use bevy::utils::HashMap;
pub use bevy::window::{WindowDescriptor, WindowMode, WindowResizeConstraints};
//use bevy_prototype_debug_lines::*;
use std::path::PathBuf;
use std::time::Duration;

use crate::{
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MouseWheel},
    prelude::{CollisionEvent, KeyboardInput, KeyboardState, MouseState},
    sprite::Sprite,
    text::Text,
};

/// EngineState is the primary way that you will interact with Rusty Engine. Every frame this struct
/// is provided to the "logic" function (or closure) that you provided to [`Game::run`]. The
/// fields in this struct are divided into two groups:
///
/// 1. `SYNCED` fields.
///
/// These fields are marked with `SYNCED`. These fields are shared between you and the engine. Each
/// frame Rusty Engine will populate these fields, then provide them to the user's game logic
/// function, and then examine any changes the user made and sync those changes back to the engine.
/// There are dedicated methods to create items for these fields.
///
/// 2. `INFO` fields
///
/// INFO fields are provided as fresh, readable information to you each frame. Since information in
/// these fields are overwritten every frame, any changes are ignored. Thus, you can feel free to,
/// e.g. consume all the events out of the `collision_events` vector.
#[derive(Default, Debug)]
pub struct EngineState {
    /// SYNCED - The state of all sprites this frame. To add a sprite, use the
    /// [`add_sprite`](EngineState::add_sprite) method. Modify & remove sprites as you like.
    pub sprites: HashMap<String, Sprite>,
    /// SYNCED - The state of all texts this frame. For convenience adding a text, use the
    /// [`add_text`](EngineState::add_text) method. Modify & remove text as you like.
    pub texts: HashMap<String, Text>,
    /// SYNCED - If set to `true`, the game exits. Note: the current frame will run to completion first.
    pub should_exit: bool,
    /// SYNCED - If set to `true`, then debug lines are shown depicting sprite colliders
    pub debug_sprite_colliders: bool,
    /// INFO - All the collision events that occurred this frame. For collisions to be generated
    /// between sprites, both sprites must have [`Sprite.collision`] set to `true`. Collision events
    /// are generated when two sprites' colliders begin or end overlapping in 2D space.
    pub collision_events: Vec<CollisionEvent>,
    /// INFO - The current state of mouse location and buttons. Useful for input handling that only
    /// cares about the final state of the mouse each frame, and not the intermediate states.
    pub mouse_state: MouseState,
    /// INFO - All the mouse button events that occurred this frame.
    pub mouse_button_events: Vec<MouseButtonInput>,
    /// INFO - All the mouse location events that occurred this frame. The events are Bevy
    /// [`CursorMoved`] structs, but despite the name they represent the _location_ of the mouse
    /// during this frame.
    pub mouse_location_events: Vec<CursorMoved>,
    /// INFO - All the mouse motion events that occurred this frame. These represent the relative
    /// movements of the mouse, not the location of the mouse.
    pub mouse_motion_events: Vec<MouseMotion>,
    /// INFO - All the mouse wheel events that occurred this frame.
    pub mouse_wheel_events: Vec<MouseWheel>,
    /// INFO - All the keyboard input events. These are text-processor-like events. If you are
    /// looking for keyboard events to control movement in a game character, you should use
    /// [`EngineState::keyboard_state`] instead. For example, one pressed event will fire when you
    /// start holding down a key, and then after a short delay additional pressed events will occur
    /// at the same rate that additional letters would show up in a word processor. When the key is
    /// finally released, a single released event is emitted.
    pub keyboard_state: KeyboardState,
    /// INFO - The delta time (time between frames) for the current frame as a [`Duration`], perfect
    /// for use with [`Timer`](crate::prelude::Timer)s
    pub keyboard_events: Vec<KeyboardInput>,
    /// INFO - The current state of all the keys on the keyboard. Use this to control movement in
    /// your games!  A [`KeyboardState`] has helper methods you should use to query the state of
    /// specific [`KeyCode`](crate::prelude::KeyCode)s.
    pub delta: Duration,
    /// INFO - The delta time (time between frames) for the current frame as an [`f32`], perfect for
    /// use in math with other `f32`'s. A cheap and quick way to approximate smooth movement
    /// (velocity, accelleration, etc.) is to multiply it by `delta_f32`.
    pub delta_f32: f32,
    /// INFO - The amount of time the game has been running since startup as a [`Duration`]
    pub time_since_startup: Duration,
    /// INFO - The amount of time the game has been running as an [`f64`]. This needs to be an f64,
    /// since it gets to be large enough that an f32 would lose precision. For best results, do your
    /// math on the `f64` and get it to a smaller value _before_ casting it to an `f32`.
    pub time_since_startup_f64: f64,
    /// A struct with methods to play sound effects and music
    pub audio_manager: AudioManager,
    /// INFO - Window dimensions in logical pixels
    pub window_dimensions: Vec2,
}

impl EngineState {
    #[must_use]
    /// Add an [`Sprite`]. Use the `&mut Sprite` that is returned to set the translation, rotation,
    /// etc. Use a unique label for each sprite. Attempting to add two sprites with the same label
    /// will crash.
    pub fn add_sprite<T: Into<String>, P: Into<PathBuf>>(
        &mut self,
        label: T,
        file_or_preset: P,
    ) -> &mut Sprite {
        let label = label.into();
        self.sprites
            .insert(label.clone(), Sprite::new(label.clone(), file_or_preset));
        // Unwrap: Can't crash because we just inserted the sprite
        self.sprites.get_mut(&label).unwrap()
    }

    #[must_use]
    /// Add a [`Text`]. Use the `&mut Text` that is returned to set the translation, rotation, etc.
    /// Use a unique label for each text. Attempting to add two texts with the same label will
    /// crash.
    pub fn add_text<T, S>(&mut self, label: T, text: S) -> &mut Text
    where
        T: Into<String>,
        S: Into<String>,
    {
        let label = label.into();
        let text = text.into();
        let curr_text = Text {
            label: label.clone(),
            value: text,
            ..Default::default()
        };
        self.texts.insert(label.clone(), curr_text);
        // Unwrap: Can't crash because we just inserted the text
        self.texts.get_mut(&label).unwrap()
    }
}

// startup system - grab window settings, initialize all the starting sprites
#[doc(hidden)]
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut engine_state: ResMut<EngineState>,
) {
    add_sprites(&mut commands, &asset_server, &mut engine_state);
    add_texts(&mut commands, &asset_server, &mut engine_state);
}

// helper function: Add Bevy components for all the sprites in engine_state.sprites
#[doc(hidden)]
pub fn add_sprites(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    engine_state: &mut EngineState,
) {
    for (_, sprite) in engine_state.sprites.drain() {
        let transform = sprite.bevy_transform();
        let texture_path = PathBuf::from("sprite").join(&sprite.filepath);
        commands.spawn().insert(sprite).insert_bundle(SpriteBundle {
            texture: asset_server.load(texture_path),
            transform,
            ..Default::default()
        });
    }
}

/// Bevy system which adds any needed Bevy components to correspond to the texts in
/// `engine_state.texts`
#[doc(hidden)]
pub fn add_texts(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    engine_state: &mut EngineState,
) {
    for (_, text) in engine_state.texts.drain() {
        let transform = text.bevy_transform();
        let font_size = text.font_size;
        let text_string = text.value.clone();
        let font_path = format!("font/{}", text.font);
        commands.spawn().insert(text).insert_bundle(Text2dBundle {
            text: BevyText::with_section(
                text_string,
                TextStyle {
                    font: asset_server.load(font_path.as_str()),
                    font_size,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform,
            ..Default::default()
        });
    }
}

// system - update current window dimensions in the engine state, because people resize windows
#[doc(hidden)]
pub fn update_window_dimensions(windows: Res<Windows>, mut engine_state: ResMut<EngineState>) {
    // Unwrap: If we can't access the primary window...there's no point to running Rusty Engine
    let window = windows.get_primary().unwrap();
    let screen_dimensions = Vec2::new(window.width(), window.height());
    if screen_dimensions != engine_state.window_dimensions {
        engine_state.window_dimensions = screen_dimensions;
        info!("Set window dimensions: {}", engine_state.window_dimensions);
    }
}

/*
// system - draw sprite colliders
#[doc(hidden)]
pub fn draw_sprite_colliders(
    engine_state: Res<EngineState>,
    mut lines: ResMut<DebugLines>,
    sprite_query: Query<&Sprite>,
) {
    if !engine_state.debug_sprite_colliders {
        return;
    }
    for sprite in sprite_query.iter() {
        let points = sprite.collider.relative_to(sprite); // will be empty vector if NoCollider
        let length = points.len();
        if length < 2 {
            continue;
        }
        let mut curr = 0;
        let mut next = 1;
        while curr < length {
            lines.line(points[curr].extend(0.0), points[next].extend(0.0), 0.0);
            curr += 1;
            next = (next + 1) % length;
        }
    }
}
*/

/// A [`Game`] represents the entire game and its data.
/// By default the game will spawn an empty window, and exit upon Esc or closing of the window.
/// Under the hood, Rusty Engine syncs the game data to Bevy to power most of the underlying
/// functionality.
///
/// [`Game`] forwards method calls to [`EngineState`] when it can, so you should be able to use all
/// of the methods in [`EngineState`] on [`Game`] during your game setup in your `main()` function.
///
/// *Note:* YOU NEED TO USE THE VERSION OF `Game` GENERATED BY THE `rusty_engine::init!( ... )`
/// MACRO CALL!  _This_ version is a dummy just so we can document how to use the generated version!
pub struct Game {}

impl Game {
    /// Create an empty [`Game`] with an empty [`EngineState`]
    ///
    /// *Note:* YOU NEED TO USE THE VERSION OF `Game` GENERATED BY THE `rusty_engine::init!( ... )`
    /// MACRO CALL!  _This_ version is a dummy just so we can document how to use the generated version!
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        panic!("Use the `Game` struct generated by rusty_engine::init!( ... ). See https://github.com/CleanCut/rusty_engine/#quick-start")
    }

    /// Use this to set properties of the native OS window before running the game. See the
    /// [window](https://github.com/CleanCut/rusty_engine/blob/main/examples/window.rs) example for
    /// more information.
    ///
    /// *Note:* YOU NEED TO USE THE VERSION OF `Game` GENERATED BY THE `rusty_engine::init!( ... )`
    /// MACRO CALL!  _This_ version is a dummy just so we can document how to use the generated version!
    #[allow(unused_variables)]
    pub fn window_settings(&mut self, window_descriptor: WindowDescriptor) -> &mut Self {
        panic!("Use the `Game` struct generated by rusty_engine::init!( ... ). See https://github.com/CleanCut/rusty_engine/#quick-start")
    }

    /// Start the game.
    ///
    /// # Examples
    ///
    /// There are much more interesting and complete examples in [the `examples/` directory.](https://github.com/CleanCut/rusty_engine/tree/main/examples)
    ///
    /// The type of `initial_game_state` is the type you pass into `rusty_engine::init!()`, or a unit struct `()` if you don't pass anything in.
    #[allow(unused_variables)]
    pub fn run(&mut self, initial_game_state: ()) {
        panic!("Use the `Game` struct generated by rusty_engine::init!( ... ). See https://github.com/CleanCut/rusty_engine/#quick-start")
    }

    #[allow(unused_variables)]
    /// `logic_function` is a function or closure that takes two parameters:
    ///
    /// - `engine_state: &mut EngineState`
    /// - `game_state`, which is a mutable reference (`&mut`) to the struct type you passed to `rusty_engine::init!()`, or `&mut ()` if you don't pass anything in.
    ///
    /// and returns a `bool`. If `false` is returned, no more logic functions are processed this frame after this one.
    pub fn add_logic(&mut self, logic_function: ()) {
        panic!("Use the `Game` struct generated by rusty_engine::init!( ... ). See https://github.com/CleanCut/rusty_engine/#quick-start")
    }
}

#[macro_export]
macro_rules! init {
    () => {
        // If the user doesn't pass in a type, pass in the unit struct
        rusty_engine::init!{()}
    };
    ($game_state_type:ty) => {

use rusty_engine::{
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MousePlugin, MouseWheel},
    prelude::{
        AudioManagerPlugin, CollisionEvent, KeyboardInput, KeyboardPlugin, KeyboardState,
        MouseState, PhysicsPlugin,
    },
    game::/*{draw_sprite_colliders, */update_window_dimensions/*}*/,
    sprite::{Sprite, SpritePreset},
    text::Text,
};
use bevy::{app::AppExit, input::system::exit_on_esc_system,
    prelude::{
        App, Assets, AssetServer, Color, ColorMaterial, Commands, DefaultPlugins,
        Entity, EventReader, EventWriter, IntoSystem, OrthographicCameraBundle,
        ParallelSystemDescriptorCoercion, Query, QuerySet, QueryState, Res, ResMut,
        Text as BevyText, Transform, Vec3,
    }, utils::HashMap};
use bevy_kira_audio::*;
//use bevy_prototype_debug_lines::*;
use std::{sync::Mutex, time::Duration, ops::{Deref, DerefMut}};


type LogicFunction = fn(&mut EngineState, &mut $game_state_type) -> bool;

/// A [`Game`] represents the entire game and its data.
/// By default the game will spawn an empty window, and exit upon Esc or closing of the window.
/// Under the hood, Rusty Engine syncs the game data to Bevy to power most of the underlying
/// functionality.
struct Game {
    app: App,
    engine_state: EngineState,
    logic_functions: Vec<LogicFunction>,
    window_descriptor: WindowDescriptor,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            app: App::new(),
            engine_state: EngineState::default(),
            logic_functions: vec![],
            window_descriptor: WindowDescriptor {
                title: "Rusty Engine".into(),
                ..Default::default()
            },
        }
    }
}

impl Game {
    /// documented in the public stub
    fn new() -> Self {
        if std::fs::read_dir("assets").is_err() {
            println!("FATAL: Could not find assets directory. Have you downloaded the assets?\nhttps://github.com/CleanCut/rusty_engine#you-must-download-the-assets-separately");
            std::process::exit(1);
        }
        Default::default()
    }

    /// documented in the public stub
    fn window_settings(&mut self, window_descriptor: WindowDescriptor) -> &mut Self {
        self.window_descriptor = window_descriptor;
        log::debug!("window descriptor is: {:?}", self.window_descriptor);
        self
    }

    /// documented in the public stub
    fn run(&mut self, initial_game_state: $game_state_type) {
        self.app
            .insert_resource::<WindowDescriptor>(self.window_descriptor.clone())
            .insert_resource::<$game_state_type>(initial_game_state);
        self.app
            // Built-ins
            .add_plugins_with(DefaultPlugins, |group| {
                group.disable::<bevy::audio::AudioPlugin>()
            })
            .add_system(exit_on_esc_system)
            // External Plugins
            .add_plugin(AudioPlugin) // kira_bevy_audio
            //.add_plugin(DebugLinesPlugin) // bevy_prototype_debug_lines, for debugging sprite colliders
            // Rusty Engine Plugins
            .add_plugin(AudioManagerPlugin)
            .add_plugin(KeyboardPlugin)
            .add_plugin(MousePlugin)
            .add_plugin(PhysicsPlugin)
            //.insert_resource(ReportExecutionOrderAmbiguities) // for debugging
            .add_system(update_window_dimensions.label("update_window_dimensions").before("game_logic_sync"))
            .add_system(game_logic_sync.label("game_logic_sync"))
            //.add_system(draw_sprite_colliders.label("draw_sprite_colliders").after("game_logic_sync"))
            .add_startup_system(rusty_engine::game::setup);
        self.app.world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        let engine_state = std::mem::take(&mut self.engine_state);
        self.app.insert_resource(engine_state);
        let logic_functions = std::mem::take(&mut self.logic_functions);
        self.app.insert_resource(logic_functions);
        self.app.run();
    }

    /// documented in the public stub
    fn add_logic(&mut self, logic_function: LogicFunction) {
        self.logic_functions.push(logic_function);
    }
}

// system - the magic that connects Rusty Engine to Bevy, frame by frame
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn game_logic_sync(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut engine_state: ResMut<EngineState>,
    mut game_state: ResMut<$game_state_type>,
    logic_functions: Res<Vec<LogicFunction>>,
    keyboard_state: Res<KeyboardState>,
    mouse_state: Res<MouseState>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_set: QuerySet<(
        QueryState<&Sprite>,
        QueryState<&Text>,
        QueryState<(Entity, &mut Sprite, &mut Transform)>,
        QueryState<(Entity, &mut Text, &mut Transform, &mut BevyText)>,
    )>,
) {
    // Update this frame's timing info
    engine_state.delta = time.delta();
    engine_state.delta_f32 = time.delta_seconds();
    engine_state.time_since_startup = time.time_since_startup();
    engine_state.time_since_startup_f64 = time.seconds_since_startup();

    // TODO: Transfer any changes to the Bevy components by the physics system over to the Sprites
    // for (mut sprite, mut transform) in sprite_query.iter_mut() {
    //     sprite.translation = Vec2::from(transform.translation);
    //     sprite.layer = transform.translation.z;
    //     // transform.rotation = Quat::from_axis_angle(Vec3::Z, sprite.rotation);
    //     sprite.rotation = ???
    //     sprite.scale = transform.scale.x;
    // }

    // Copy keyboard state over to engine_state to give to users
    engine_state.keyboard_state = keyboard_state.clone();

    // Copy mouse state over to engine_state to give to users
    engine_state.mouse_state = mouse_state.clone();

    // Copy all collision events over to the engine_state to give to users
    engine_state.collision_events.clear();
    for collision_event in collision_events.iter() {
        engine_state.collision_events.push(collision_event.clone());
    }

    // Copy all sprites over to the engine_state to give to users
    engine_state.sprites.clear();
    for sprite in query_set.q0().iter() {
        let _ = engine_state
            .sprites
            .insert(sprite.label.clone(), (*sprite).clone());
    }

    // Copy all texts over to the engine_state to give to users
    engine_state.texts.clear();
    for text in query_set.q1().iter() {
        let _ = engine_state
            .texts
            .insert(text.label.clone(), (*text).clone());
    }

    // Perform all the user's game logic for this frame
    for func in logic_functions.iter() {
        // If the user returns false, abort the rest of the game logic
        if !func(&mut engine_state, &mut game_state) {
            break;
        }
    }

    // Transfer any changes in the user's Sprite copies to the Bevy Sprite and Transform components
    for (entity, mut sprite, mut transform) in query_set.q2().iter_mut() {
        if let Some(sprite_copy) = engine_state.sprites.remove(&sprite.label) {
            *sprite = sprite_copy;
            *transform = sprite.bevy_transform();
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Transfer any changes in the user's Texts to the Bevy Text and Transform components
    for (entity, mut text, mut transform, mut bevy_text_component) in query_set.q3().iter_mut() {
        if let Some(text_copy) = engine_state.texts.remove(&text.label) {
            *text = text_copy;
            *transform = text.bevy_transform();
            if text.value != bevy_text_component.sections[0].value {
                bevy_text_component.sections[0].value = text.value.clone();
            }
            #[allow(clippy::float_cmp)]
            if text.font_size != bevy_text_component.sections[0].style.font_size {
                bevy_text_component.sections[0].style.font_size = text.font_size;
            }
            let font_path = format!("font/{}", text.font);
            let font = asset_server.load(font_path.as_str());
            if bevy_text_component.sections[0].style.font != font {
                bevy_text_component.sections[0].style.font = font;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Add Bevy components for any new sprites remaining in engine_state.sprites
    rusty_engine::game::add_sprites(&mut commands, &asset_server, &mut engine_state);

    // Add Bevy components for any new texts remaining in engine_state.texts
    rusty_engine::game::add_texts(&mut commands, &asset_server, &mut engine_state);

    if engine_state.should_exit {
        app_exit_events.send(AppExit);
    }
}


// The Deref and DerefMut implementations make it so that you can call all the `EngineState` methods
// on a `Game`, which is much more straightforward for game setup in `main()`
impl Deref for Game {
    type Target = EngineState;

    fn deref(&self) -> &Self::Target {
        &self.engine_state
    }
}

impl DerefMut for Game {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine_state
    }
}

};
}
