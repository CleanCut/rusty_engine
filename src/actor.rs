use std::sync::Mutex;

use crate::game::GameState;
use crate::physics::Collider;
use bevy::prelude::*;
use lazy_static::lazy_static;

#[derive(Default)]
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            actor_spawner
                .system()
                .label("actor_spawner")
                .before("game_logic_sync"),
        )
        .add_system(
            actor_sync
                .system()
                .after("actor_spawner")
                .before("game_logic_sync"),
        );
    }
}

pub type ActorLogicFunction = fn(&mut GameState, &mut Actor, &Time);

// TODO: Find a way to connect outside logic with the Bevy system in a more elegant way if possible
lazy_static! {
    pub(crate) static ref ACTOR_LOGIC_FUNCTIONS: Mutex<Vec<ActorLogicFunction>> =
        Mutex::new(vec![]);
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
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut actor_query: Query<(&mut Actor, &mut Transform)>,
) {
    for (mut actor, mut transform) in actor_query.iter_mut() {
        // Perform the user-specified logic on the Actor, which has a bunch of proxy data
        // Unwrap: We're the only system that uses ACTOR_LOGIC_FUNCTIONS after the game runs.
        for func in ACTOR_LOGIC_FUNCTIONS.lock().unwrap().iter() {
            func(&mut game_state, &mut actor, &time);
        }
        // Transfer any changes to the proxies over to the real components
        transform.translation = actor.translation.extend(actor.layer);
        transform.rotation = Quat::from_axis_angle(Vec3::Z, actor.rotation);
        transform.scale = Vec3::splat(actor.scale);
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    /// READONLY: A way to identify an actor.
    pub label: String,
    /// READONLY: Which preset was used to create this actor
    pub preset: Option<ActorPreset>,
    /// READONLY: File used for this actor's sprite
    pub filename: String,
    /// SYNCED: Where you are in 2D game space. Positive x is right. Positive y is up. (0.0, 0.0) is the
    /// center of the screen.
    pub translation: Vec2,
    /// SYNCED: Depth of the sprite. 0.0 (back) to 999.0 (front)
    pub layer: f32,
    /// SYNCED: Direction you face in radians. See constants UP, DOWN, LEFT, RIGHT
    pub rotation: f32,
    /// SYNCED: 1.0 is the normal 100%
    pub scale: f32,
    /// TODO: Whether or not to calculate collisions
    pub collision: bool,
    /// TODO: Relative to translation
    pub collider: Collider,
}

/// An [`Actor`] is the basic abstraction for something that can be seen and interacted with.
/// Players, obstacles, etc. are all actors.
impl Default for Actor {
    fn default() -> Self {
        Self {
            label: String::default(),
            preset: None,
            filename: String::default(),
            translation: Vec2::default(),
            layer: f32::default(),
            rotation: f32::default(),
            scale: 1.0,
            collision: true,
            collider: Collider::default(),
        }
    }
}

impl Actor {
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.label = name;
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
    pub fn set_layer(&mut self, layer: f32) -> &mut Self {
        self.layer = layer;
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

use std::array::IntoIter;

#[derive(Copy, Clone, Debug)]
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
    pub fn build(self, label: String) -> Actor {
        let filename = self.filename();
        Actor {
            label,
            preset: Some(self),
            filename,
            ..Default::default()
        }
    }
    pub fn filename(&self) -> String {
        match self {
            ActorPreset::RacingBarrelBlue => "sprite/racing/barrel_blue.png",
            ActorPreset::RacingBarrelRed => "sprite/racing/barrel_red.png",
            ActorPreset::RacingBarrierRed => "sprite/racing/barrier_red.png",
            ActorPreset::RacingBarrierWhite => "sprite/racing/barrier_white.png",
            ActorPreset::RacingCarBlack => "sprite/racing/car_black.png",
            ActorPreset::RacingCarBlue => "sprite/racing/car_blue.png",
            ActorPreset::RacingCarGreen => "sprite/racing/car_green.png",
            ActorPreset::RacingCarRed => "sprite/racing/car_red.png",
            ActorPreset::RacingCarYellow => "sprite/racing/car_yellow.png",
            ActorPreset::RacingConeStraight => "sprite/racing/cone_straight.png",
            ActorPreset::RollingBallBlue => "sprite/rolling/ball_blue.png",
            ActorPreset::RollingBallBlueAlt => "sprite/rolling/ball_blue_alt.png",
            ActorPreset::RollingBallRed => "sprite/rolling/ball_red.png",
            ActorPreset::RollingBallRedAlt => "sprite/rolling/ball_red_alt.png",
            ActorPreset::RollingBlockCorner => "sprite/rolling/block_corner.png",
            ActorPreset::RollingBlockNarrow => "sprite/rolling/block_narrow.png",
            ActorPreset::RollingBlockSmall => "sprite/rolling/block_small.png",
            ActorPreset::RollingBlockSquare => "sprite/rolling/block_square.png",
            ActorPreset::RollingHoleEnd => "sprite/rolling/hole_end.png",
            ActorPreset::RollingHoleStart => "sprite/rolling/hole_start.png",
        }
        .into()
    }
    pub fn variant_iter() -> IntoIter<ActorPreset, 20> {
        static ACTOR_PRESETS: [ActorPreset; 20] = [
            ActorPreset::RacingBarrelBlue,
            ActorPreset::RacingBarrelRed,
            ActorPreset::RacingBarrierRed,
            ActorPreset::RacingBarrierWhite,
            ActorPreset::RacingCarBlack,
            ActorPreset::RacingCarBlue,
            ActorPreset::RacingCarGreen,
            ActorPreset::RacingCarRed,
            ActorPreset::RacingCarYellow,
            ActorPreset::RacingConeStraight,
            ActorPreset::RollingBallBlueAlt,
            ActorPreset::RollingBallBlue,
            ActorPreset::RollingBallRedAlt,
            ActorPreset::RollingBallRed,
            ActorPreset::RollingBlockCorner,
            ActorPreset::RollingBlockNarrow,
            ActorPreset::RollingBlockSmall,
            ActorPreset::RollingBlockSquare,
            ActorPreset::RollingHoleEnd,
            ActorPreset::RollingHoleStart,
        ];
        std::array::IntoIter::new(ACTOR_PRESETS)
    }
}
