// Used locally
use crate::actor::{Actor, ActorPlugin, LogicFunction, LOGICS};
use crate::preset::ActorPreset;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use std::collections::HashMap;

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
            .add_plugins(DefaultPlugins)
            .add_system(exit_on_esc_system.system())
            .add_plugin(ActorPlugin::default());

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

    pub fn add_logic(&mut self, logic: LogicFunction) {
        // Unwrap: The only way this could crash is for another thread to take the lock and crash.
        LOGICS.lock().unwrap().push(logic);
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

#[derive(Default)]
pub struct GameState {
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
}
