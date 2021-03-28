use crate::physics::Collider;
use crate::preset::ActorPreset;
use crate::GameState;
use bevy::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Default)]
pub struct ActorPlugin {}

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(actor_spawner.system())
            .add_system(actor_sync.system());
    }
}

pub type LogicFunction = fn(&mut GameState, &mut Actor, &Time);

// TODO: Find a way to connect outside logic with the Bevy system in a more elegant way if possible
lazy_static! {
    pub(crate) static ref LOGICS: Mutex<Vec<LogicFunction>> = Mutex::new(vec![]);
}

fn actor_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    new_actor_query: Query<(Entity, &Actor), Without<Transform>>,
) {
    for (entity, actor) in new_actor_query.iter() {
        let transform = Transform::from_translation(actor.translation.extend(0.0));
        let texture_handle = asset_server.load(actor.filename.as_str());
        commands.entity(entity).insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform,
            ..Default::default()
        });
    }
}

fn actor_sync(
    mut game_state: ResMut<GameState>, // todo: AAArgh. How do we make this the same type as in Game<T>???
    time: Res<Time>,
    mut actor_query: Query<(&mut Actor, &mut Transform)>,
) {
    for (mut actor, mut transform) in actor_query.iter_mut() {
        // Perform the user-specified logic on the Actor, which has a bunch of proxy data
        for logic in LOGICS.lock().unwrap().iter() {
            logic(&mut game_state, &mut actor, &time);
        }
        // Transfer any changes to the proxies over to the real components
        transform.translation = actor.translation.extend(0.0);
        transform.rotation = Quat::from_axis_angle(Vec3::Z, actor.rotation);
        transform.scale = Vec3::splat(actor.scale);
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
    // Direction you face in radians. See constants UP, DOWN, LEFT, RIGHT
    pub rotation: f32,
    // 1.0 is "normal"
    pub scale: f32,
    // Whether or not to calculate collisions
    pub collision: bool,
    // Relative to translation
    pub collider: Collider,
}

/// An [`Actor`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all actors.
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
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn set_preset(&mut self, preset: ActorPreset) -> &mut Self {
        self.preset = Some(preset);
        self
    }
    pub fn set_translation(&mut self, translation: Vec2) -> &mut Self {
        self.translation = translation;
        self
    }
    pub fn set_rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
        self
    }
    pub fn set_collision(&mut self, value: bool) -> &mut Self {
        self.collision = value;
        self
    }
    pub fn set_collider(&mut self, collider: Collider) -> &mut Self {
        self.collider = collider;
        self
    }
}
