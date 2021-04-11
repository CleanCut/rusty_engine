// Used locally
use crate::{
    actor::{Actor, ActorLogicFunction, ActorPlugin, ActorPreset, ACTOR_LOGIC_FUNCTIONS},
    audio::AudioManager,
    prelude::AudioManagerPlugin,
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
#[derive(Default)]
pub struct Game {
    actors: Vec<Actor>,
    app_builder: AppBuilder,
    game_state: GameState,
}

impl Game {
    /// Create an empty [`Game`] with an empty [`GameState`] and an empty vector of [`Actor`]s
    pub fn new() -> Self {
        let mut app_builder = App::build();
        app_builder
            .add_plugins_with(DefaultPlugins, |group| {
                group.disable::<bevy::audio::AudioPlugin>()
            })
            .add_system(exit_on_esc_system.system())
            .add_plugin(ActorPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(AudioManagerPlugin)
            //.insert_resource(ReportExecutionOrderAmbiguities)
            .add_system(game_logic_system.system());

        Self {
            app_builder,
            actors: Vec::default(),
            game_state: GameState::default(),
        }
    }

    pub fn add_actor(&mut self, label: String, preset: ActorPreset) -> &mut Actor {
        self.actors.push(preset.build(label));
        // Unwrap: Can't crash because we just inserted the actor
        self.actors.last_mut().unwrap()
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

    pub fn run(&mut self) {
        let world = self.app_builder.world_mut();
        world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        for actor in self.actors.drain(..) {
            world.spawn().insert(actor);
        }
        let game_state = std::mem::take(&mut self.game_state);
        self.app_builder.insert_resource(game_state);
        self.app_builder.run();
    }
}

fn game_logic_system(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    // Unwrap: We're the only system that uses GAME_LOGIC_FUNCTIONS after the game is run
    for func in GAME_LOGIC_FUNCTIONS.lock().unwrap().iter() {
        func(&mut game_state, &time);
    }

    if game_state.should_exit {
        app_exit_events.send(AppExit);
    }
}

#[derive(Default)]
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
    // Used by internal methods
    should_exit: bool,
}

impl GameState {
    pub fn exit(&mut self) {
        self.should_exit = true;
    }
}
