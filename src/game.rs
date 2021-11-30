use crate::{
    actor::{Actor, ActorPreset},
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MouseWheel},
    prelude::{CollisionEvent, KeyboardInput, KeyboardState, MouseState},
    text_actor::TextActor,
};
use bevy::{prelude::*, utils::HashMap};
use std::time::Duration;

pub type GameLogicFunction = fn(&mut GameState);
pub use bevy::window::{WindowDescriptor, WindowMode, WindowResizeConstraints};

/// GameState is the primary way that you will interact with Rusty Engine. Every frame this struct
/// is provided to the "game logic" function or closure that you provided to [`Game::run`]. The
/// fields in this struct are divided into two groups:
///
/// 1. `USER` fields
///
/// These fields are only used by you, the developer of the game, and are persistent across frames.
///
/// The names of the fields in this first group begin with the name of the type you can store (e.g.
/// `f32`), and end in either `_vec` or `_map` to indicate that the field is a vector or hash map,
/// respectively. All hash map keys are [`String`]s. These fields are intended for you to use if you
/// need to carry state across frames. For example, in one frame you may add a bunch of message
/// strings to `.string_vec`, and then in later frames you may take these strings and render
/// messages in the game once the player has had time to read them. Rusty Engine won't touch
/// anything in these fields, so how you use them is completely up to you.
///
/// 2. `SYNCED` fields.
///
/// These fields are marked with `SYNCED`. These fields are shared between you and the engine. Each
/// frame Rusty Engine will populate these fields, then provide them to the user's game logic
/// function, and then examine any changes the user made and take action on what changed. This is
/// your primary way to interact with the engine.
///
/// 3. `INFO` fields
///
/// INFO fields are provided as fresh, readable information to you each. Since information in these
/// fields are overwritten every frame, any changes are ignored. This makes it convenient to ////
/// you can feel free to, e.g. consume all the events out of the `.mouse_butten_events` vector.
#[derive(Default, Debug)]
pub struct GameState {
    // Empty collections for users
    /// USER -- hash map of [`String`] to [`bool`]
    pub bool_map: HashMap<String, bool>,
    /// USER -- hash map of [`String`] to [`f32`]
    pub f32_map: HashMap<String, f32>,
    /// USER -- hash map of [`String`] to [`i32`]
    pub i32_map: HashMap<String, i32>,
    /// USER -- hash map of [`String`] to [`u8`]
    pub u8_map: HashMap<String, u8>,
    /// USER -- hash map of [`String`] to [`u32`]
    pub u32_map: HashMap<String, u32>,
    /// USER -- hash map of [`String`] to [`usize`]
    pub usize_map: HashMap<String, usize>,
    /// USER -- hash map of [`String`] to [`String`]
    pub string_map: HashMap<String, String>,
    /// USER -- hash map of [`String`] to [`Timer`]
    pub timer_map: HashMap<String, Timer>,
    /// USER -- hash map of [`String`] to [`Vec2`]
    pub vec2_map: HashMap<String, Vec2>,
    /// USER -- vector of [`bool`]
    pub bool_vec: Vec<bool>,
    /// USER -- vector of [`f32`]
    pub f32_vec: Vec<f32>,
    /// USER -- vector of [`i32`]
    pub i32_vec: Vec<i32>,
    /// USER -- vector of [`u8`]
    pub u8_vec: Vec<u8>,
    /// USER -- vector of [`u32`]
    pub u32_vec: Vec<u32>,
    /// USER -- vector of [`usize`]
    pub usize_vec: Vec<usize>,
    /// USER -- vector of [`String`]
    pub string_vec: Vec<String>,
    /// USER -- vector of [`Timer`]
    pub timer_vec: Vec<Timer>,
    /// USER -- vector of [`Vec2`]
    pub vec2_vec: Vec<Vec2>,
    /// SYNCED - The state of all actors this frame
    pub actors: HashMap<String, Actor>,
    /// SYNCED - The state of all text actors this frame
    pub text_actors: HashMap<String, TextActor>,
    /// INFO - All the collision events that occurred this frame. For collisions to be generated
    /// between actors, both actors must have [`Actor.collision`] set to `true`. Collision events
    /// are generated when two actors' colliders begin or end overlapping in 2D space.
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
    /// [`GameState::keyboard_state`] instead. For example, one pressed event will fire when you
    /// start holding down a key, and then after a short delay additional pressed events will occur
    /// at the same rate that additional letters would show up in a word processor. When the key is
    /// finally released, a single released event is emitted.
    pub keyboard_state: KeyboardState,
    /// INFO - The delta time (time between frames) for the current frame as a [`Duration`], perfect
    /// for use with [`Timer`]s
    pub keyboard_events: Vec<KeyboardInput>,
    /// INFO - The current state of all the keys on the keyboard. Use this to control movement in
    /// your games!  A [`KeyboardState`] has helper methods you should use to query the state of
    /// specific [`KeyCode`]s.
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
    /// INFO - Screen dimensions in pixels
    pub screen_dimensions: Vec2,
    // Used by internal methods
    #[doc(hidden)]
    pub should_exit: bool,
}

impl GameState {
    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    #[must_use]
    /// Add an [`Actor`]. Use the `&mut Actor` that is returned to set the translation, rotation,
    /// etc. Attempting to add two actors with the same label will crash.
    pub fn add_actor<T: Into<String>>(&mut self, label: T, preset: ActorPreset) -> &mut Actor {
        let label = label.into();
        self.actors
            .insert(label.clone(), preset.build(label.clone()));
        // Unwrap: Can't crash because we just inserted the actor
        self.actors.get_mut(&label).unwrap()
    }

    #[must_use]
    /// Add a [`TextActor`]. Use the `&mut TextActor` that is returned to set the translation,
    /// rotation, etc. Attempting to add two text actors with the same label will crash.
    pub fn add_text_actor<T, S>(&mut self, label: T, text: S) -> &mut TextActor
    where
        T: Into<String>,
        S: Into<String>,
    {
        let label = label.into();
        let text = text.into();
        let text_actor = TextActor {
            label: label.clone(),
            text,
            ..Default::default()
        };
        self.text_actors.insert(label.clone(), text_actor);
        // Unwrap: Can't crash because we just inserted the actor
        self.text_actors.get_mut(&label).unwrap()
    }
}

#[macro_export]
macro_rules! game_stuff {
    ($custom_state_type:ty) => {

use rusty_engine::{
    actor::{Actor, ActorPreset},
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MousePlugin, MouseWheel},
    prelude::{
        AudioManagerPlugin, CollisionEvent, KeyboardInput, KeyboardPlugin, KeyboardState,
        MouseState, PhysicsPlugin,
    },
    text_actor::TextActor,
};
use bevy::{app::AppExit, input::system::exit_on_esc_system, prelude::*, utils::HashMap};
use bevy_kira_audio::*;
use std::{sync::Mutex, time::Duration};


type LogicFunction = fn(&mut GameState, &mut $custom_state_type);

/// A [`Game`] represents the entire game and its data.
/// By default the game will spawn an empty window, and exit upon Esc or closing of the window.
/// Under the hood, Rusty Engine syncs the game data to Bevy to power most of the underlying
/// functionality.
pub struct Game {
    app_builder: AppBuilder,
    game_state: GameState,
    logic_functions: Vec<LogicFunction>,
    window_descriptor: WindowDescriptor,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            app_builder: App::build(),
            game_state: GameState::default(),
            logic_functions: vec![],
            window_descriptor: WindowDescriptor {
                title: "Rusty Engine".into(),
                ..Default::default()
            },
        }
    }
}

impl Game {
    /// Create an empty [`Game`] with an empty [`GameState`] and an empty vector of [`Actor`]s
    pub fn new() -> Self {
        if std::fs::read_dir("assets").is_err() {
            println!("FATAL: Could not find assets directory. Have you downloaded the assets?\nhttps://github.com/CleanCut/rusty_engine#you-must-download-the-assets-separately");
            std::process::exit(1);
        }
        Default::default()
    }

    #[must_use]
    /// Add an [`Actor`] before the game has begun. Use the `&mut Actor` that is returned to set
    /// the translation, rotation, etc.
    pub fn add_actor<T: Into<String>>(&mut self, label: T, preset: ActorPreset) -> &mut Actor {
        let label = label.into();
        self.game_state
            .actors
            .insert(label.clone(), preset.build(label.clone()));
        // Unwrap: Can't crash because we just inserted the actor
        self.game_state.actors.get_mut(&label).unwrap()
    }

    #[must_use]
    /// Add a [`TextActor`] before the game has begun. Use the `&mut TextActor` that is returned to
    /// set the translation, rotation, etc.
    pub fn add_text_actor<T, S>(&mut self, label: T, text: S) -> &mut TextActor
    where
        T: Into<String>,
        S: Into<String>,
    {
        let label = label.into();
        let text = text.into();
        let text_actor = TextActor {
            label: label.clone(),
            text,
            ..Default::default()
        };
        self.game_state
            .text_actors
            .insert(label.clone(), text_actor);
        // Unwrap: Can't crash because we just inserted the actor
        self.game_state.text_actors.get_mut(&label).unwrap()
    }

    /// Use to get a `&mut GameState` to set up game state before the game has begun.
    pub fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }

    /// Use this to set properties of the native OS window before running the game. See the
    /// [window](https://github.com/CleanCut/rusty_engine/blob/main/examples/window.rs) example for
    /// more information.
    pub fn window_settings(&mut self, window_descriptor: WindowDescriptor) -> &mut Self {
        self.window_descriptor = window_descriptor;
        log::debug!("window descriptor is: {:?}", self.window_descriptor);
        self
    }

    /// Start the game. This method never returns.  [`GameLogicFunction`] should be a function or
    /// closure which accepts one parameter, a `&mut GameState` and returns nothing.
    ///
    /// # Examples
    ///
    /// There are much more interesting and complete examples in [the `examples/` directory.](https://github.com/CleanCut/rusty_engine/tree/main/examples)
    ///
    /// You can write a function for your game logic.
    ///
    /// ```no_run
    /// use rusty_engine::prelude::*;
    ///
    /// fn game_logic(game_state: &mut GameState) {
    ///     println!("This game has been running for: {:.2}", game_state.time_since_startup_f64);
    /// }
    ///
    /// fn main() {
    ///     let mut game = Game::new();
    ///     game.run(game_logic);
    /// }
    /// ```
    ///
    /// You can write a closure for your game logic, if and only if your closure doesn't capture
    /// any variables.
    ///
    /// ```no_run
    /// # use rusty_engine::prelude::*;
    /// let mut game = Game::new();
    /// game.run(|game_state| {
    ///     println!("This game has been running for: {:.2}", game_state.time_since_startup_f64);
    /// });
    /// ```
    ///
    /// If you don't want to do anything, you can use the closure `|_| {}`
    ///
    /// ```no_run
    /// # use rusty_engine::prelude::*;
    /// let mut game = Game::new();
    /// game.run(|_| {});
    /// ```
    pub fn run(&mut self, custom_state: $custom_state_type) {
        self.app_builder
            .insert_resource::<WindowDescriptor>(self.window_descriptor.clone())
            .insert_resource::<$custom_state_type>(custom_state);
        self.app_builder
            // Built-ins
            .add_plugins_with(DefaultPlugins, |group| {
                group.disable::<bevy::audio::AudioPlugin>()
            })
            .add_system(exit_on_esc_system.system())
            // External Plugins
            .add_plugin(AudioPlugin) // kira_bevy_audio
            // Rusty Engine Plugins
            .add_plugin(AudioManagerPlugin)
            .add_plugin(KeyboardPlugin)
            .add_plugin(MousePlugin)
            .add_plugin(PhysicsPlugin)
            //.insert_resource(ReportExecutionOrderAmbiguities) // for debugging
            .add_system(game_logic_sync.system().label("game_logic_sync"))
            .add_startup_system(setup.system());
        // Unwrap: Can't crash, we're the only thread using the lock, so it can't be poisoned.
        //GAME_LOGIC_FUNCTIONS.lock().unwrap().push(func);
        let world = self.app_builder.world_mut();
        world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        let game_state = std::mem::take(&mut self.game_state);
        self.app_builder.insert_resource(game_state);
        let logic_functions = std::mem::take(&mut self.logic_functions);
        self.app_builder.insert_resource(logic_functions);
        self.app_builder.run();
    }

    pub fn add_logic(&mut self, logic_function: LogicFunction) {
        self.logic_functions.push(logic_function);
    }
}

// startup system - grab window settings, initialize all the starting actors
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mut game_state: ResMut<GameState>,
) {
    // Unwrap: If we can't access the primary window...there's no point to running Rusty Engine
    let window = windows.get_primary().unwrap();
    game_state.screen_dimensions = Vec2::new(window.width(), window.height());
    info!("Window dimensions: {}", game_state.screen_dimensions);
    add_actors(&mut commands, &asset_server, materials, &mut game_state);
    add_text_actors(&mut commands, &asset_server, &mut game_state);
}

// system - the magic that connects Rusty Engine to Bevy, frame by frame
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn game_logic_sync(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    mut custom_state: ResMut<$custom_state_type>,
    logic_functions: Res<Vec<LogicFunction>>,
    keyboard_state: Res<KeyboardState>,
    mouse_state: Res<MouseState>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
    mut collision_events: EventReader<CollisionEvent>,
    // mut actor_query: Query<(&mut Actor, &mut Transform)>,
    // mut text_actor_query: Query<(&mut TextActor, &mut Transform)>,
    mut query_set: QuerySet<(
        Query<&Actor>,
        Query<&TextActor>,
        Query<(Entity, &mut Actor, &mut Transform)>,
        Query<(Entity, &mut TextActor, &mut Transform, &mut Text)>,
    )>,
) {
    // Update this frame's timing info
    game_state.delta = time.delta();
    game_state.delta_f32 = time.delta_seconds();
    game_state.time_since_startup = time.time_since_startup();
    game_state.time_since_startup_f64 = time.seconds_since_startup();

    // TODO: Transfer any changes to the Bevy components by the physics system over to the Actors
    // for (mut actor, mut transform) in actor_query.iter_mut() {
    //     actor.translation = Vec2::from(transform.translation);
    //     actor.layer = transform.translation.z;
    //     // transform.rotation = Quat::from_axis_angle(Vec3::Z, actor.rotation);
    //     actor.rotation = ???
    //     actor.scale = transform.scale.x;
    // }

    // Copy keyboard state over to game_state to give to users
    game_state.keyboard_state = keyboard_state.clone();

    // Copy mouse state over to game_state to give to users
    game_state.mouse_state = mouse_state.clone();

    // Copy all collision events over to the game_state to give to users
    game_state.collision_events.clear();
    for collision_event in collision_events.iter() {
        game_state.collision_events.push(collision_event.clone());
    }

    // Copy all actors over to the game_state to give to users
    game_state.actors.clear();
    for actor in query_set.q0().iter() {
        let _ = game_state
            .actors
            .insert(actor.label.clone(), (*actor).clone());
    }

    // Copy all text_actors over to the game_state to give to users
    game_state.text_actors.clear();
    for text_actor in query_set.q1().iter() {
        let _ = game_state
            .text_actors
            .insert(text_actor.label.clone(), (*text_actor).clone());
    }

    // Perform all the user's game logic for this frame
    for func in logic_functions.iter() {
        func(&mut game_state, &mut custom_state);
    }

    // Transfer any changes in the user's Actor copies to the Bevy Actor and Transform components
    for (entity, mut actor, mut transform) in query_set.q2_mut().iter_mut() {
        if let Some(actor_copy) = game_state.actors.remove(&actor.label) {
            *actor = actor_copy;
            *transform = actor.bevy_transform();
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Transfer any changes in the user's TextActor copies to the Bevy TextActor and Transform components
    for (entity, mut text_actor, mut transform, mut text) in query_set.q3_mut().iter_mut() {
        if let Some(text_actor_copy) = game_state.text_actors.remove(&text_actor.label) {
            *text_actor = text_actor_copy;
            *transform = text_actor.bevy_transform();
            if text_actor.text != text.sections[0].value {
                text.sections[0].value = text_actor.text.clone();
            }
            #[allow(clippy::float_cmp)]
            if text_actor.font_size != text.sections[0].style.font_size {
                text.sections[0].style.font_size = text_actor.font_size;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Add Bevy components for any new actors remaining in game_state.actors
    add_actors(&mut commands, &asset_server, materials, &mut game_state);

    // Add Bevy components for any new text_actors remaining in game_state.text_actors
    add_text_actors(&mut commands, &asset_server, &mut game_state);

    if game_state.should_exit {
        app_exit_events.send(AppExit);
    }
}

// helper function: Add Bevy components for all the actors in game_state.actors
fn add_actors(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: &mut GameState,
) {
    for (_, actor) in game_state.actors.drain() {
        // let mut transform = Transform::from_translation(actor.translation.extend(actor.layer));
        // transform.rotation = Quat::from_axis_angle(Vec3::Z, actor.rotation);
        // transform.scale = Vec3::splat(actor.scale);
        let transform = actor.bevy_transform();
        let texture_handle = asset_server.load(actor.filename.as_str());
        commands.spawn().insert(actor).insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform,
            ..Default::default()
        });
    }
}

// helper function: Add Bevy components for all the actors in game_state.text_actors
fn add_text_actors(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_state: &mut GameState,
) {
    for (_, text_actor) in game_state.text_actors.drain() {
        let transform = text_actor.bevy_transform();
        let font_size = text_actor.font_size;
        let text = text_actor.text.clone();
        commands
            .spawn()
            .insert(text_actor)
            .insert_bundle(Text2dBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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

};
}
