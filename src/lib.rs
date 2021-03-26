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
            .add_system(actor_system.system())
            .add_system(actor_spawner.system());
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
        let world = self.app_builder.world_mut();
        world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        for (_name, actor) in self.actors.drain() {
            world.spawn().insert(actor);
        }
        self.app_builder.run();
    }
}

fn actor_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    new_actor_query: Query<(Entity, &Actor), Without<Transform>>,
) {
    for (entity, actor) in new_actor_query.iter() {
        println!("Spawning {}", actor.name);
        let transform = Transform::from_translation(actor.translation.extend(0.0));
        let texture_handle = asset_server.load(actor.filename.as_str());
        commands.entity(entity).insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform,
            ..Default::default()
        });
    }
}

fn actor_system(mut actor_query: Query<(&mut Actor, &mut Transform)>) {
    for (mut actor, mut transform) in actor_query.iter_mut() {
        // Perform the user-specified logic on the Actor, which has a bunch of proxy data
        for logic in LOGICS.lock().unwrap().iter() {
            logic(&mut actor);
        }
        // Transfer any changes to the proxies over to the real components
        transform.translation = actor.translation.extend(0.0);
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    // must be unique
    pub name: String,
    // preset
    pub preset: Option<ActorPreset>,
    // filename
    pub filename: String,
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

impl Default for Actor {
    fn default() -> Self {
        Self {
            name: String::default(),
            preset: None,
            filename: String::default(),
            translation: Vec2::default(),
            rotation: f32::default(),
            scale: 1.0,
            collision: true,
            collider: Collider::default(),
        }
    }
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

#[derive(Clone, Debug)]
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
        let (filename, _): (&str, i32) = match self {
            ActorPreset::RacingBarrelBlue => ("sprite/racing/barrel_blue.png", 0),
            ActorPreset::RacingBarrelRed => ("sprite/racing/barrel_red.png", 0),
            ActorPreset::RacingBarrierRed => ("sprite/racing/barrier_red.png", 0),
            ActorPreset::RacingBarrierWhite => ("sprite/racing/barrier_white.png", 0),
            ActorPreset::RacingCarBlack => ("sprite/racing/car_black.png", 0),
            ActorPreset::RacingCarBlue => ("sprite/racing/car_blue.png", 0),
            ActorPreset::RacingCarGreen => ("sprite/racing/car_green.png", 0),
            ActorPreset::RacingCarRed => ("sprite/racing/car_red.png", 0),
            ActorPreset::RacingCarYellow => ("sprite/racing/car_yellow.png", 0),
            ActorPreset::RacingConeStraight => ("sprite/racing/cone_straight.png", 0),
            ActorPreset::RollingBallBlue => ("sprite/rolling/ball_blue.png", 0),
            ActorPreset::RollingBallBlueAlt => ("sprite/rolling/ball_blue_alt.png", 0),
            ActorPreset::RollingBallRed => ("sprite/rolling/ball_red.png", 0),
            ActorPreset::RollingBallRedAlt => ("sprite/rolling/ball_red_alt.png", 0),
            ActorPreset::RollingBlockCorner => ("sprite/rolling/block_corner.png", 0),
            ActorPreset::RollingBlockNarrow => ("sprite/rolling/block_narrow.png", 0),
            ActorPreset::RollingBlockSmall => ("sprite/rolling/block_small.png", 0),
            ActorPreset::RollingBlockSquare => ("sprite/rolling/block_square.png", 0),
            ActorPreset::RollingHoleEnd => ("sprite/rolling/hole_end.png", 0),
            ActorPreset::RollingHoleStart => ("sprite/rolling/hole_start.png", 0),
        };

        let filename = filename.to_string();

        Actor {
            name,
            preset: Some(self),
            filename,
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
