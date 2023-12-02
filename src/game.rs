use bevy::{
    app::AppExit,
    prelude::{Text as BevyText, *},
    time::Time,
    utils::HashMap,
    window::{close_on_esc, PrimaryWindow, WindowPlugin},
};
use bevy_prototype_lyon::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    time::Duration,
};

use crate::{
    audio::AudioManager,
    mouse::{CursorMoved, MouseButtonInput, MouseMotion, MousePlugin, MouseWheel},
    prelude::{
        AudioManagerPlugin, CollisionEvent, KeyboardInput, KeyboardPlugin, KeyboardState,
        MouseState, PhysicsPlugin,
    },
    sprite::Sprite,
    text::Text,
};

// Public re-export
pub use bevy::window::{Cursor, Window, WindowMode, WindowResolution};

/// Engine is the primary way that you will interact with Rusty Engine. Each frame this struct
/// is provided to the "logic" functions (or closures) that you provided to [`Game::add_logic`]. The
/// fields in this struct are divided into two groups:
///
/// 1. `SYNCED` fields.
///
/// These fields are marked with `SYNCED`. These fields are shared between you and the engine. Each
/// frame Rusty Engine will populate these fields, then provide them to the user's game logic
/// function, and then examine any changes the user made and sync those changes back to the engine.
///
/// 2. `INFO` fields
///
/// INFO fields are provided as fresh, readable information to you each frame. Since information in
/// these fields are overwritten every frame, any changes to them are ignored. Thus, you can feel
/// free to, e.g. consume all the events out of the `collision_events` vector.
#[derive(Default, Debug, Resource)]
pub struct Engine {
    /// SYNCED - The state of all sprites this frame. To add a sprite, use the
    /// [`add_sprite`](Engine::add_sprite) method. Modify & remove sprites as you like.
    pub sprites: HashMap<String, Sprite>,
    /// SYNCED - The state of all texts this frame. For convenience adding a text, use the
    /// [`add_text`](Engine::add_text) method. Modify & remove text as you like.
    pub texts: HashMap<String, Text>,
    /// SYNCED - If set to `true`, the game exits. Note: the current frame will run to completion first.
    pub should_exit: bool,
    /// SYNCED - If set to `true`, then debug lines are shown depicting sprite colliders
    pub show_colliders: bool,
    // so we can tell if the value changed this frame
    last_show_colliders: bool,
    /// INFO - All the collision events that occurred this frame. For collisions to be generated
    /// between sprites, both sprites must have [`Sprite.collision`] set to `true` and both sprites
    /// must have colliders (use the collider example to create a collider for your own images).
    /// Collision events are generated when two sprites' colliders begin or end overlapping in 2D
    /// space.
    pub collision_events: Vec<CollisionEvent>,
    /// INFO - The current state of mouse location and buttons. Useful for input handling that only
    /// cares about the final state of the mouse each frame, and not the intermediate states.
    pub mouse_state: MouseState,
    /// INFO - All the mouse button events that occurred this frame.
    pub mouse_button_events: Vec<MouseButtonInput>,
    /// INFO - All the mouse location events that occurred this frame. The events are Bevy
    /// [`CursorMoved`] structs, but despite the name they represent the _location_ of the mouse
    /// during this frame.
    pub mouse_location_events: Vec<CursorMoved>,
    /// INFO - All the mouse motion events that occurred this frame. These represent the relative
    /// movements of the mouse, not the location of the mouse.
    pub mouse_motion_events: Vec<MouseMotion>,
    /// INFO - All the mouse wheel events that occurred this frame.
    pub mouse_wheel_events: Vec<MouseWheel>,
    /// INFO - The current state of all the keys on the keyboard. Use this to control movement in
    /// your games!  A [`KeyboardState`] has helper methods you should use to query the state of
    /// specific [`KeyCode`](crate::prelude::KeyCode)s.
    pub keyboard_state: KeyboardState,
    /// INFO - All the keyboard input events. These are text-processor-like events. If you are
    /// looking for keyboard events to control movement in a game character, you should use
    /// [`Engine::keyboard_state`] instead. For example, one pressed event will fire when you
    /// start holding down a key, and then after a short delay additional pressed events will occur
    /// at the same rate that additional letters would show up in a word processor. When the key is
    /// finally released, a single released event is emitted.
    pub keyboard_events: Vec<KeyboardInput>,
    /// INFO - The delta time (time between frames) for the current frame as a [`Duration`], perfect
    /// for use with [`Timer`](crate::prelude::Timer)s
    pub delta: Duration,
    /// INFO - The delta time (time between frames) for the current frame as an [`f32`], perfect for
    /// use in math with other `f32`'s. A cheap and quick way to approximate smooth movement
    /// (velocity, accelleration, etc.) is to multiply it by `delta_f32`.
    pub delta_f32: f32,
    /// INFO - The amount of time the game has been running since startup as a [`Duration`]
    pub time_since_startup: Duration,
    /// INFO - The amount of time the game has been running as an [`f64`]. This needs to be an f64,
    /// since it gets to be large enough that an f32 would lose precision. For best results, do your
    /// math on the `f64` and get it to a smaller value _before_ casting it to an `f32`.
    pub time_since_startup_f64: f64,
    /// A struct with methods to play sound effects and music
    pub audio_manager: AudioManager,
    /// INFO - Window dimensions in logical pixels. On high DPI screens, there will often be four
    /// physical pixels per logical pixel. On low DPI screens, one logical pixel is one physical
    /// pixel.
    pub window_dimensions: Vec2,
}

impl Engine {
    #[must_use]
    /// Create and add a [`Sprite`] to the game. Use the `&mut Sprite` that is returned to adjust
    /// the translation, rotation, etc. Use a *unique* label for each sprite. Attempting to add two
    /// sprites with the same label will cause a crash.
    pub fn add_sprite<T: Into<String>, P: Into<PathBuf>>(
        &mut self,
        label: T,
        file_or_preset: P,
    ) -> &mut Sprite {
        let label = label.into();
        self.sprites
            .insert(label.clone(), Sprite::new(label.clone(), file_or_preset));
        // Unwrap: Can't crash because we just inserted the sprite
        self.sprites.get_mut(&label).unwrap()
    }

    #[must_use]
    /// Create and add a [`Text`] to the game. Use the `&mut Text` that is returned to adjust the
    /// translation, rotation, etc. Use a *unique* label for each text. Attempting to add two texts
    /// with the same label will cause a crash.
    pub fn add_text<T, S>(&mut self, label: T, text: S) -> &mut Text
    where
        T: Into<String>,
        S: Into<String>,
    {
        let label = label.into();
        let text = text.into();
        let curr_text = Text {
            label: label.clone(),
            value: text,
            ..Default::default()
        };
        self.texts.insert(label.clone(), curr_text);
        // Unwrap: Can't crash because we just inserted the text
        self.texts.get_mut(&label).unwrap()
    }
}

/// startup system - grab window settings, initialize all the starting sprites
#[doc(hidden)]
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut engine: ResMut<Engine>) {
    add_sprites(&mut commands, &asset_server, &mut engine);
    add_texts(&mut commands, &asset_server, &mut engine);
}

/// Add visible lines representing a collider
fn add_collider_lines(commands: &mut Commands, sprite: &mut Sprite) {
    // Add the collider lines, a visual representation of the sprite's collider
    let points = sprite.collider.points(); // will be empty vector if NoCollider
    if points.len() >= 2 {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(points[0]);
        for point in &points[1..] {
            path_builder.line_to(*point);
        }
        path_builder.close(); // draws the line from the last point to the first point
        let line = path_builder.build();
        let transform = sprite.bevy_transform();
        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::new().add(&line).build(),
                    spatial: SpatialBundle::from_transform(transform),
                    ..Default::default()
                },
                Stroke::new(Color::WHITE, 1.0 / transform.scale.x),
            ))
            // .spawn(GeometryBuilder::build_as(
            //     &line,
            //     DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.0 / transform.scale.x)),
            //     transform,
            // ))
            .insert(ColliderLines {
                sprite_label: sprite.label.clone(),
            });
    }
    sprite.collider_dirty = false;
}

/// helper function: Add Bevy components for all the sprites in engine.sprites
#[doc(hidden)]
pub fn add_sprites(commands: &mut Commands, asset_server: &Res<AssetServer>, engine: &mut Engine) {
    for (_, sprite) in engine.sprites.drain() {
        // Create the sprite
        let transform = sprite.bevy_transform();
        let texture_path = sprite.filepath.clone();
        commands.spawn((
            sprite,
            SpriteBundle {
                texture: asset_server.load(texture_path),
                transform,
                ..Default::default()
            },
        ));
    }
}

/// Bevy system which adds any needed Bevy components to correspond to the texts in
/// `engine.texts`
#[doc(hidden)]
pub fn add_texts(commands: &mut Commands, asset_server: &Res<AssetServer>, engine: &mut Engine) {
    for (_, text) in engine.texts.drain() {
        let transform = text.bevy_transform();
        let font_size = text.font_size;
        let text_string = text.value.clone();
        let font_path = text.font.clone();
        commands.spawn((
            text,
            Text2dBundle {
                text: BevyText::from_section(
                    text_string,
                    TextStyle {
                        font: asset_server.load(font_path),
                        font_size,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform,
                ..Default::default()
            },
        ));
    }
}

/// system - update current window dimensions in the engine, because people resize windows
#[doc(hidden)]
pub fn update_window_dimensions(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut engine: ResMut<Engine>,
) {
    // It's possible to not have a window for the first frame or two
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let screen_dimensions = Vec2::new(window.width(), window.height());
    if screen_dimensions != engine.window_dimensions {
        engine.window_dimensions = screen_dimensions;
        debug!("Set window dimensions: {}", engine.window_dimensions);
    }
}

/// Component to add to the collider lines visualizations to link them to the sprite they represent
#[derive(Component)]
#[doc(hidden)]
pub struct ColliderLines {
    sprite_label: String,
}

/// A [`Game`] represents the entire game and its data.
/// By default the game will spawn an empty window, and exit upon Esc or closing of the window.
/// Under the hood, Rusty Engine syncs the game data to [Bevy](https://bevyengine.org/) to power
/// most of the underlying functionality.
///
/// [`Game`] forwards method calls to [`Engine`] when it can, so you should be able to use all
/// of the methods from [`Engine`] on [`Game`] during your game setup in your `main()` function.
pub struct Game<S: Resource + Send + Sync + 'static> {
    app: App,
    engine: Engine,
    logic_functions: LogicFuncVec<S>,
    window: Window,
}

impl<S: Resource + Send + Sync + 'static> Default for Game<S> {
    fn default() -> Self {
        Self {
            app: App::new(),
            engine: Engine::default(),
            logic_functions: LogicFuncVec(vec![]),
            window: Window {
                title: "Rusty Engine".into(),
                ..Default::default()
            },
        }
    }
}

impl<S: Resource + Send + Sync + 'static> Game<S> {
    /// Create an new, empty [`Game`] with an empty [`Engine`]
    pub fn new() -> Self {
        if std::fs::read_dir("assets").is_err() {
            println!("FATAL: Could not find assets directory. Have you downloaded the assets?\nhttps://github.com/CleanCut/rusty_engine#you-must-download-the-assets-separately");
            std::process::exit(1);
        }
        Default::default()
    }

    /// Use this to set properties of the native OS window before running the game. See the
    /// [window](https://github.com/CleanCut/rusty_engine/blob/main/examples/window.rs) example for
    /// more information.
    pub fn window_settings(&mut self, window: Window) -> &mut Self {
        self.window = window;
        self
    }

    /// Start the game.
    pub fn run(&mut self, initial_game_state: S) {
        self.app.insert_resource::<S>(initial_game_state);
        self.app
            // TODO: Remove this to use the new, darker default color once the videos have been remastered
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            // Built-ins
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(self.window.clone()),
                        ..Default::default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add_systems(
                Update,
                (
                    (close_on_esc),
                    (update_window_dimensions, game_logic_sync::<S>),
                ),
            )
            // External Plugins
            .add_plugins(ShapePlugin) // bevy_prototype_lyon, for displaying sprite colliders
            // Rusty Engine Plugins
            .add_plugins((
                AudioManagerPlugin,
                KeyboardPlugin,
                MousePlugin,
                PhysicsPlugin,
            ))
            //.insert_resource(ReportExecutionOrderAmbiguities) // for debugging
            .add_systems(Startup, setup);
        self.app.world.spawn(Camera2dBundle::default());
        let engine = std::mem::take(&mut self.engine);
        self.app.insert_resource(engine);
        let mut logic_functions = LogicFuncVec(vec![]);
        std::mem::swap(&mut self.logic_functions, &mut logic_functions);
        self.app.insert_resource(logic_functions);
        self.app.run();
    }

    /// `logic_function` is a function or closure that takes two parameters and returns nothing:
    ///
    /// - `engine: &mut Engine`
    /// - `game_state`, which is a mutable reference (`&mut`) to the game state struct you defined,
    ///    or `&mut ()` if you didn't define one.
    pub fn add_logic(&mut self, logic_function: fn(&mut Engine, &mut S)) {
        self.logic_functions.0.push(logic_function);
    }
}

/// system - the magic that connects Rusty Engine to Bevy, frame by frame
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn game_logic_sync<S: Resource + Send + Sync + 'static>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut engine: ResMut<Engine>,
    mut game_state: ResMut<S>,
    logic_functions: Res<LogicFuncVec<S>>,
    keyboard_state: Res<KeyboardState>,
    mouse_state: Res<MouseState>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_set: ParamSet<(
        Query<(Entity, &mut Sprite, &mut Transform)>,
        Query<(Entity, &mut Text, &mut Transform, &mut BevyText)>,
        Query<(Entity, &mut Stroke, &mut Transform, &ColliderLines)>,
    )>,
) {
    // Update this frame's timing info
    engine.delta = time.delta();
    engine.delta_f32 = time.delta_seconds();
    engine.time_since_startup = time.elapsed();
    engine.time_since_startup_f64 = time.elapsed_seconds_f64();

    // Copy keyboard state over to engine to give to users
    engine.keyboard_state = keyboard_state.clone();

    // Copy mouse state over to engine to give to users
    engine.mouse_state = mouse_state.clone();

    // Copy all collision events over to the engine to give to users
    engine.collision_events.clear();
    for collision_event in collision_events.read() {
        engine.collision_events.push(collision_event.clone());
    }

    // Copy all sprites over to the engine to give to users
    engine.sprites.clear();
    for (_, sprite, _) in query_set.p0().iter() {
        let _ = engine
            .sprites
            .insert(sprite.label.clone(), (*sprite).clone());
    }

    // Copy all texts over to the engine to give to users
    engine.texts.clear();
    for (_, text, _, _) in query_set.p1().iter() {
        let _ = engine.texts.insert(text.label.clone(), (*text).clone());
    }

    // Perform all the user's game logic for this frame
    for func in logic_functions.0.iter() {
        func(&mut engine, &mut game_state);
    }

    if !engine.last_show_colliders && engine.show_colliders {
        // Just turned on show_colliders -- create collider lines for all sprites
        for sprite in engine.sprites.values_mut() {
            add_collider_lines(&mut commands, sprite);
        }
    } else if engine.last_show_colliders && !engine.show_colliders {
        // Just turned off show_colliders -- delete collider lines for all sprites
        for (entity, _, _, _) in query_set.p2().iter_mut() {
            commands.entity(entity).despawn();
        }
    }
    // Update transform & line width of all collider lines
    if engine.show_colliders {
        // Delete collider lines for sprites which are missing, or whose colliders are dirty
        for (entity, _, _, collider_lines) in query_set.p2().iter_mut() {
            if let Some(sprite) = engine.sprites.get(&collider_lines.sprite_label) {
                if sprite.collider_dirty {
                    commands.entity(entity).despawn();
                }
            } else {
                commands.entity(entity).despawn();
            }
        }
        // Add collider lines for sprites whose colliders are dirty
        for sprite in engine.sprites.values_mut() {
            if sprite.collider_dirty {
                add_collider_lines(&mut commands, sprite);
            }
        }
        // Update transform & line width
        for (_, mut stroke, mut transform, collider_lines) in query_set.p2().iter_mut() {
            if let Some(sprite) = engine.sprites.get(&collider_lines.sprite_label) {
                *transform = sprite.bevy_transform();
                // We want collider lines to appear on top of the sprite they are for, so they need a
                // slightly higher z value. We tell users to only use up to 999.0.
                transform.translation.z = (transform.translation.z + 0.1).clamp(0.0, 999.1);
            }
            // Stroke line width gets scaled with the transform, but we want it to appear to be the same
            // regardless of scale, so we have to counter the scale.
            stroke.options.line_width = 1.0 / transform.scale.x;
        }
    }
    engine.last_show_colliders = engine.show_colliders;

    // Transfer any changes in the user's Sprite copies to the Bevy Sprite and Transform components
    for (entity, mut sprite, mut transform) in query_set.p0().iter_mut() {
        if let Some(sprite_copy) = engine.sprites.remove(&sprite.label) {
            *sprite = sprite_copy;
            *transform = sprite.bevy_transform();
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Add Bevy components for any new sprites remaining in engine.sprites
    add_sprites(&mut commands, &asset_server, &mut engine);

    // Transfer any changes in the user's Texts to the Bevy Text and Transform components
    for (entity, mut text, mut transform, mut bevy_text_component) in query_set.p1().iter_mut() {
        if let Some(text_copy) = engine.texts.remove(&text.label) {
            *text = text_copy;
            *transform = text.bevy_transform();
            if text.value != bevy_text_component.sections[0].value {
                bevy_text_component.sections[0].value = text.value.clone();
            }
            #[allow(clippy::float_cmp)]
            if text.font_size != bevy_text_component.sections[0].style.font_size {
                bevy_text_component.sections[0].style.font_size = text.font_size;
            }
            let font = asset_server.load(text.font.clone());
            if bevy_text_component.sections[0].style.font != font {
                bevy_text_component.sections[0].style.font = font;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Add Bevy components for any new texts remaining in engine.texts
    add_texts(&mut commands, &asset_server, &mut engine);

    if engine.should_exit {
        app_exit_events.send(AppExit);
    }
}

// The Deref and DerefMut implementations make it so that you can call all the `Engine` methods
// on a `Game`, which is much more straightforward for game setup in `main()`
impl<S: Resource + Send + Sync + 'static> Deref for Game<S> {
    type Target = Engine;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

impl<S: Resource + Send + Sync + 'static> DerefMut for Game<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine
    }
}

#[derive(Resource)]
struct LogicFuncVec<S: Resource + Send + Sync + 'static>(Vec<fn(&mut Engine, &mut S)>);
