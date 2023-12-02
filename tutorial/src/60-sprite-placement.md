# Sprite Placement

There are four different fields you can use to position and size your sprite:

### Translation

`Sprite.translation` is a [`Vec2`](https://docs.rs/glam/latest/glam/f32/struct.Vec2.html) containing the X and Y coordinates of your sprite's position on the screen. The coordinate system works just like it does in math class. `(0.0, 0.0)` is in the center of the screen. Positive X goes to the right side of the screen. Positive Y goes to the top of the screen. Every increment of `1.0` is one logical pixel on the screen. Hi-DPI screens may have more than one physical pixel per logical pixel. See the [`Engine`](400-engine.md) section for details on how to check the logical pixel dimensions of your window.

### Rotation

`Sprite.rotation` is an `f32` representing the angle in radians from the positive X axis. In other words, a rotation of `0.0` is facing to the right, so custom images you want to use in Rusty Engine should also be "facing" to the right in their raw form (whatever "to the right" means is up to you). `2 * PI` brings you in a full circle, so `0.5 * PI` is "up", `PI` is "left", and `1.5 * PI` is "down". There are a bunch of helpful constants defined for cardinal directions if you don't want to remember the numerical value yourself. These constants are all included in the prelude.

```
UP
DOWN
LEFT
RIGHT

NORTH
NORTH_EAST
EAST
SOUTH_EAST
SOUTH
SOUTH_WEST
WEST
NORTH_WEST
```

### Scale

`Sprite.scale` is an `f32`. `1.0` is the default, which means matching a pixel of the source image to a pixel on the screen. `2.0` makes the image twice as wide and tall, etc.

### Layer

`Sprite.layer` is an `f32` that affects what sprite or text is "on top" of another sprite or text when they overlap. `0.0` is the default layer and is on the "bottom", while `999.0` is the "top" layer. The order of sprites or text on the same layer is random and unstable (can change frame to frame), so you should make sure that sprites and text that will overlap are on different layers so they don't change their position unpredictably. A good practice is to choose a few layers and assign them to constants, and then don't let sprites on the same layer overlap. For example:

```rust,ignored
const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;
```


### Adjusting your newly-created sprite

When you create a sprite, you get a mutable reference to the newly-created sprite that you can use to adjust it.

```rust,ignored
let player = engine.add_sprite("my_player", SpritePreset::RacingCarBlue);
player.translation = Vec2::new(200.0, 100.0); // Move the car up and to the right
player.rotation = UP; // UP is one of the built-in constants you can use
player.scale = 2.5; // It's a BIG car!
player.layer = CHARACTER_LAYER; // as in previous code snippet
```

The `Vec2` type used for the `translation` field is from `glam`, and has [its own documentation](https://docs.rs/glam/latest/glam/f32/struct.Vec2.html) you can read up on if you're interested.  The thing you'll probably use the most are its `x` and `y` fields:

```rust,ignored
player.translation.x += 45.0 * engine.delta_f32;
player.translation.y -= 10.0 * engine.delta_f32;
```

NOTE: If you want to adjust your sprite smoothly, you will need to multiply it by the frame's delta value. See the [`Engine`](400-engine.md) section for more details.

### Adjusting an existing sprite

To adjust a sprite which already exists, you need to get a mutable reference to it.  This is where that "label" comes in.  The `sprites` field on the `Engine` struct is a hash map of labels to sprites. You get a mutable reference to a sprite with the `HashMap::get_mut` method:


```rust,ignored
// Be careful with unwrap()! If the entry isn't there, this will crash your game.
let player_reference = engine.sprites.get_mut("my_player").unwrap();
player_reference.rotation += TURN_SPEED_PER_SEC * engine.delta_f32;
```

### Deleting a sprite

To delete a sprite, remove it from the `Engine.sprites` hash map.

```rust,ignored
engine.sprites.remove("my_player");
```

