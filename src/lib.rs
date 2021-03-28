// Public prelude
pub mod prelude {
    pub use crate::{
        actor::{Actor, ActorPreset},
        consts::*,
        Game,
    };
    pub use bevy::{
        self,
        prelude::{Time, Vec2},
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

use crate::actor::{Actor, ActorPlugin, ActorPreset, LogicFunction, LOGICS};
use bevy::{input::system::exit_on_esc_system, prelude::*, utils::HashMap};

#[derive(Default)]
pub struct Game<T>
where
    T: Default + Sync + Send + 'static,
{
    actors: HashMap<String, Actor>,
    app_builder: AppBuilder,
    game_state: T,
}

impl<T> Game<T>
where
    T: Default + Sync + Send,
{
    pub fn new(game_state: T) -> Self {
        let mut app_builder = App::build();
        app_builder
            .add_plugins(DefaultPlugins)
            .add_system(exit_on_esc_system.system())
            .add_plugin(ActorPlugin::default());

        Self {
            app_builder,
            actors: HashMap::default(),
            game_state,
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
