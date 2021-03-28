// Public prelude
pub mod prelude {
    pub use crate::{actor::Actor, consts::*, preset::ActorPreset, Game, GameState};
    pub use bevy::{
        self,
        prelude::{Time, Timer, Vec2},
    };
}

pub mod actor;
pub mod consts {
    use std::f32::consts::{FRAC_PI_2, PI};
    pub const RIGHT: f32 = 0.0;
    pub const LEFT: f32 = PI;
    pub const UP: f32 = FRAC_PI_2;
    pub const DOWN: f32 = PI + FRAC_PI_2;
}
pub mod physics;
pub mod preset;

use std::collections::HashMap;

use crate::actor::{Actor, ActorPlugin, LogicFunction, LOGICS};
use crate::preset::ActorPreset;
use bevy::{input::system::exit_on_esc_system, prelude::*};

#[derive(Default)]
pub struct Game {
    actors: HashMap<String, Actor>,
    app_builder: AppBuilder,
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut app_builder = App::build();
        app_builder
            .add_plugins(DefaultPlugins)
            .add_system(exit_on_esc_system.system())
            .add_plugin(ActorPlugin::default());

        Self {
            app_builder,
            actors: HashMap::default(),
            game_state: GameState::default(),
        }
    }

    pub fn add_actor(&mut self, name: String, preset: ActorPreset) -> &mut Actor {
        if self.actors.contains_key(&name) {
            panic!("An actor named \"{}\" already exists!", name);
        }
        self.actors.insert(name.clone(), preset.build(name.clone()));
        self.actors.get_mut(&name).unwrap() // Unwrap: Can't crash because we just inserted the actor
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
        for (_name, actor) in self.actors.drain() {
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
