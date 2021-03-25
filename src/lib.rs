// Public prelude
pub mod prelude {
    pub use super::{Actor, ActorPreset, Game};
    pub use bevy::{self, prelude::Vec2};
}

use bevy::{input::system::exit_on_esc_system, prelude::*, utils::HashMap};
use lazy_static::lazy_static;
use std::sync::Mutex;

// TODO: Find a way to connect outside logic with the Bevy system in a more elegant way if possible
lazy_static! {
    static ref LOGICS: Mutex<Vec<LogicFunction>> = Mutex::new(vec![]);
}

type LogicFunction = fn(&mut Actor);

#[derive(Default)]
pub struct Game {
    actors: HashMap<String, Actor>,
    app_builder: AppBuilder,
}

impl Game {
    pub fn new() -> Self {
        let mut app_builder = App::build();
        app_builder
            .add_plugins(DefaultPlugins)
            .add_system(exit_on_esc_system.system())
            .add_system(actor_system.system());
        Self {
            app_builder,
            actors: HashMap::default(),
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
        for (_name, actor) in self.actors.drain() {
            self.app_builder
                .world_mut()
                .spawn()
                .insert_bundle((actor, Transform::default()));
        }
        self.app_builder.run();
    }
}

fn actor_system(mut actor_query: Query<(&mut Actor, &mut Transform)>) {
    for (mut actor, mut transform) in actor_query.iter_mut() {
        println!("doing it! {:?}", actor);
        // Perform the user-specified logic on the Actor, which has a bunch of proxy data
        for logic in LOGICS.lock().unwrap().iter() {
            logic(&mut actor);
        }
        // Transfer any changes to the proxies over to the real components
        transform.translation = actor.translation.extend(0.0);
        println!("did it! {:?}", transform.translation);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Actor {
    // must be unique
    pub name: String,
    // Where you are
    pub translation: Vec2,
    // Direction you face in degrees. 0 - right. 90 - up. 180 - left. 270 - down.
    pub rotation: f32,
    // 1.0 is "normal"
    pub scale: f32,
    // Whether or not to calculate collisions
    pub collision: bool,
    // Relative to translation
    pub collider: Collider,
}

impl Actor {
    pub fn set_translation(&mut self, t: Vec2) -> &mut Self {
        self.translation = t;
        self
    }
    pub fn set_collision(&mut self, value: bool) -> &mut Self {
        self.collision = value;
        self
    }
}

pub enum ActorPreset {
    RacingBarrelBlue,
    RacingBarrelRed,
    RacingBarrierRed,
    RacingBarrierWhite,
    RacingCarBlack,
    RacingCarBlue,
    RacingCarGreen,
    RacingCarRed,
    RacingCarYellow,
    RacingConeStraight,
    RollingBallBlue,
    RollingBallBlueAlt,
    RollingBallRed,
    RollingBallRedAlt,
    RollingBlockCorner,
    RollingBlockNarrow,
    RollingBlockSmall,
    RollingBlockSquare,
    RollingHoleEnd,
    RollingHoleStart,
}

impl ActorPreset {
    pub fn build(self, name: String) -> Actor {
        let (_filename, collider): (&str, Collider) = match self {
            _ => ("a", Collider::new(-1.0, 1.0, 1.0, -1.0)),
            //ActorPreset::RacingBarrelBlue => ("a", Collider::new(-1.0, 1.0, 1.0, -1.0)),
            // ActorPreset::RacingBarrelRed => {}
            // ActorPreset::RacingBarrierRed => {}
            // ActorPreset::RacingBarrierWhite => {}
            // ActorPreset::RacingCarBlack => {}
            // ActorPreset::RacingCarBlue => {}
            // ActorPreset::RacingCarGreen => {}
            // ActorPreset::RacingCarRed => {}
            // ActorPreset::RacingCarYellow => {}
            // ActorPreset::RacingConeStraight => {}
            // ActorPreset::RollingBallBlue => {}
            // ActorPreset::RollingBallBlueAlt => {}
            // ActorPreset::RollingBallRed => {}
            // ActorPreset::RollingBallRedAlt => {}
            // ActorPreset::RollingBlockCorner => {}
            // ActorPreset::RollingBlockNarrow => {}
            // ActorPreset::RollingBlockSmall => {}
            // ActorPreset::RollingBlockSquare => {}
            // ActorPreset::RollingHoleEnd => {}
            // ActorPreset::RollingHoleStart => {}
        };

        Actor {
            name,
            collision: true,
            collider,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Collider {
    pub topleft: Vec2,
    pub bottomright: Vec2,
}

impl Collider {
    fn new(tlx: f32, tly: f32, brx: f32, bry: f32) -> Self {
        Self {
            topleft: Vec2::new(tlx, tly),
            bottomright: Vec2::new(brx, bry),
        }
    }
}
