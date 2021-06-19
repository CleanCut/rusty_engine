// Used locally
use crate::{
    actor::{Actor, ActorLogicFunction, ActorPlugin, ActorPreset, ACTOR_LOGIC_FUNCTIONS},
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MousePlugin, MouseWheel},
    prelude::{AudioManagerPlugin, KeyboardInput, KeyboardPlugin},
};
use bevy::{app::AppExit, input::system::exit_on_esc_system, prelude::*};
use bevy_kira_audio::*;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub type GameLogicFunction = fn(&mut GameState, &Time);

// TODO: Find a way to connect outside logic with the Bevy system in a more elegant way if possible
lazy_static! {
    pub(crate) static ref GAME_LOGIC_FUNCTIONS: Mutex<Vec<GameLogicFunction>> = Mutex::new(vec![]);
}

/// A [`Game`] represents the entire game, the entire program that Rusty Engine is aware of.
/// By default the game will spawn an empty window, and exit upon Esc or closing of the window.
pub struct Game {
    app_builder: AppBuilder,
    game_state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        let mut app_builder = App::build();
        app_builder
            .add_plugins_with(DefaultPlugins, |group| {
                group.disable::<bevy::audio::AudioPlugin>()
            })
            .add_system(exit_on_esc_system.system())
            .add_plugin(ActorPlugin)
            .add_plugin(AudioPlugin) // kira_bevy_audio
            .add_plugin(AudioManagerPlugin)
            .add_plugin(KeyboardPlugin)
            .add_plugin(MousePlugin)
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

    pub fn add_actor(&mut self, label: String, preset: ActorPreset) -> &mut Actor {
        self.game_state
            .actors
            .insert(label.clone(), preset.build(label.clone()));
        // Unwrap: Can't crash because we just inserted the actor
        self.game_state.actors.get_mut(&label).unwrap()
    }

    pub fn add_logic(&self, func: ActorLogicFunction) {
        // Unwrap: The only way this could crash is for another thread to take the lock and crash.
        ACTOR_LOGIC_FUNCTIONS.lock().unwrap().push(func);
    }

    pub fn add_game_logic(&self, func: GameLogicFunction) {
        // Unwrap: The only way this could crash is for another thread to take the lock and crash.
        GAME_LOGIC_FUNCTIONS.lock().unwrap().push(func);
    }

    pub fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }

    pub fn run(&mut self, func: GameLogicFunction) {
        // Unwrap: The only way this could crash is for another thread to take the lock and crash.
        GAME_LOGIC_FUNCTIONS.lock().unwrap().push(func);
        let world = self.app_builder.world_mut();
        world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        for actor in self.game_state.actors.values() {
            world.spawn().insert(actor.clone());
        }
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
    pub mouse_button_events: Vec<MouseButtonInput>,
    pub cursor_moved_events: Vec<CursorMoved>,
    pub mouse_motion_events: Vec<MouseMotion>,
    pub mouse_wheel_events: Vec<MouseWheel>,
    pub keyboard_events: Vec<KeyboardInput>,
    // Used by internal methods
    should_exit: bool,
}

impl GameState {
    pub fn exit(&mut self) {
        self.should_exit = true;
    }
}

// startup system
fn setup(windows: Res<Windows>, mut game_state: ResMut<GameState>) {
    // Unwrap: If we can't access the primary window...there's no point to running Rusty Engine
    let window = windows.get_primary().unwrap();
    game_state.screen_dimensions = Vec2::new(window.width(), window.height());
}

// system
fn game_logic_sync(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
    mut actor_query: Query<(&mut Actor, &mut Transform)>,
) {
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
    for (actor, _) in actor_query.iter_mut() {
        let _ = game_state
            .actors
            .insert(actor.label.clone(), (*actor).clone());
    }

    // Perform all the user's game logic for this frame
    // Unwrap: We're the only system that uses GAME_LOGIC_FUNCTIONS after the game is run
    for func in GAME_LOGIC_FUNCTIONS.lock().unwrap().iter() {
        func(&mut game_state, &time);
    }

    // Transfer any changes in the user's Actor copies to the Bevy Actor and Transform components
    for (mut actor, mut transform) in actor_query.iter_mut() {
        if let Some(actor_copy) = game_state.actors.remove(&actor.label) {
            *actor = actor_copy;
            transform.translation = actor.translation.extend(actor.layer);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, actor.rotation);
            transform.scale = Vec3::splat(actor.scale);
        } else {
            // TODO: Remove Bevy entity corresponding to missing Actor
        }
    }

    // TODO: Add Bevy components for any new actors remaining in game_state.actors

    if game_state.should_exit {
        app_exit_events.send(AppExit);
    }
}
