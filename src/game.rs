use crate::{
    actor::{Actor, ActorPreset},
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MousePlugin, MouseWheel},
    prelude::{AudioManagerPlugin, KeyboardInput, KeyboardPlugin, PhysicsPlugin},
    text_actor::TextActor,
};
use bevy::{app::AppExit, input::system::exit_on_esc_system, prelude::*};
use bevy_kira_audio::*;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex, time::Duration};

pub type GameLogicFunction = fn(&mut GameState);

// TODO: Find a way to connect outside logic with the Bevy system in a more elegant way if possible
lazy_static! {
    pub(crate) static ref GAME_LOGIC_FUNCTIONS: Mutex<Vec<GameLogicFunction>> = Mutex::new(vec![]);
}

/// A [`Game`] represents the entire game and its data.
/// By default the game will spawn an empty window, and exit upon Esc or closing of the window.
/// Under the hood, Rusty Engine syncs the game data to Bevy to power most of the underlying
/// functionality.
pub struct Game {
    app_builder: AppBuilder,
    game_state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        let mut app_builder = App::build();
        app_builder
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
            //.insert_resource(ReportExecutionOrderAmbiguities)
            .add_system(game_logic_sync.system().label("game_logic_sync"))
            .add_startup_system(setup.system());

        Self {
            app_builder,
            game_state: GameState::default(),
        }
    }
}

impl Game {
    /// Create an empty [`Game`] with an empty [`GameState`] and an empty vector of [`Actor`]s
    pub fn new() -> Self {
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
    ///     println!("This game has been running for: {:.2}", game_state.seconds_since_startup);
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
    /// let mut game = Game::new();
    /// game.run(|game_state| {
    ///     println!("This game has been running for: {:.2}", game_state.seconds_since_startup);
    /// });
    /// ```
    ///
    /// If you don't want to do anything, you can use the closure `|_| {}`
    ///
    /// ```no_run
    /// let mut game = Game::new();
    /// game.run(|_| {});
    /// ```
    pub fn run(&mut self, func: GameLogicFunction) {
        // Unwrap: Can't crash, we're the only thread using the lock, so it can't be poisoned.
        GAME_LOGIC_FUNCTIONS.lock().unwrap().push(func);
        let world = self.app_builder.world_mut();
        world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        let game_state = std::mem::take(&mut self.game_state);
        self.app_builder.insert_resource(game_state);
        self.app_builder.run();
    }
}

#[derive(Default, Debug)]
pub struct GameState {
    // Empty collections for users
    pub bool_map: HashMap<String, bool>,
    pub i32_map: HashMap<String, i32>,
    pub u8_map: HashMap<String, u8>,
    pub u32_map: HashMap<String, u32>,
    pub usize_map: HashMap<String, usize>,
    pub string_map: HashMap<String, String>,
    pub timer_map: HashMap<String, Timer>,
    pub bool_vec: Vec<bool>,
    pub i32_vec: Vec<i32>,
    pub u8_vec: Vec<u8>,
    pub u32_vec: Vec<u32>,
    pub usize_vec: Vec<usize>,
    pub string_vec: Vec<String>,
    pub timer_vec: Vec<Timer>,
    // Built-in stuff
    pub audio_manager: AudioManager,
    pub screen_dimensions: Vec2,
    // Updated every frame
    pub actors: HashMap<String, Actor>,
    pub text_actors: HashMap<String, TextActor>,
    pub mouse_button_events: Vec<MouseButtonInput>,
    pub cursor_moved_events: Vec<CursorMoved>,
    pub mouse_motion_events: Vec<MouseMotion>,
    pub mouse_wheel_events: Vec<MouseWheel>,
    pub keyboard_events: Vec<KeyboardInput>,
    pub delta: Duration,
    pub delta_seconds: f32,
    pub time_since_startup: Duration,
    pub seconds_since_startup: f64,
    // Used by internal methods
    should_exit: bool,
}

impl GameState {
    pub fn exit(&mut self) {
        self.should_exit = true;
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
    add_actors(&mut commands, &asset_server, materials, &mut game_state);
    add_text_actors(&mut commands, &asset_server, &mut game_state);
}

// system - the magic that connects Rusty Engine to Bevy, frame by frame
#[allow(clippy::type_complexity)]
fn game_logic_sync(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
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
    game_state.delta_seconds = time.delta_seconds();
    game_state.time_since_startup = time.time_since_startup();
    game_state.seconds_since_startup = time.seconds_since_startup();

    // TODO: Transfer any changes to the Bevy components by the physics system over to the Actors
    // for (mut actor, mut transform) in actor_query.iter_mut() {
    //     actor.translation = Vec2::from(transform.translation);
    //     actor.layer = transform.translation.z;
    //     // transform.rotation = Quat::from_axis_angle(Vec3::Z, actor.rotation);
    //     actor.rotation = ???
    //     actor.scale = transform.scale.x;
    // }

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
    // Unwrap: Can't crash, we're the only thread using the lock, so it can't be poisoned.
    for func in GAME_LOGIC_FUNCTIONS.lock().unwrap().iter() {
        func(&mut game_state);
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
