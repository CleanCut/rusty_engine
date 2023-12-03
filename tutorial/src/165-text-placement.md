# Text Placement

Text is rendered as an image. This rendering (or re-rendering) happens at the end of the frame after any of the [Value, Font & Font Size](160-text-attributes.md) attributes are changed.  However, when values such as translation, rotation, scale, or layer are changed, the image remains the same and its representation on screen is manipulated in the GPU, which is high performance.

In short, feel free to change your text's placement attributes every frame without any big hit in performance.

### Translation

`Text.translation` is a [`Vec2`](https://docs.rs/glam/latest/glam/f32/struct.Vec2.html) containing the `x` and `y` coordinates of your text's position on the screen. This `Vec2` location is in the exact center of the text, both vertically and horizontally. In other words, text is always rendered with "centered" alignment on both axes.

The coordinate system works just like it does in math class. `(0.0, 0.0)` is in the center of the screen. Positive X goes to the right side of the screen. Positive Y goes to the top of the screen. Every increment of `1.0` is one logical pixel on the screen. Hi-DPI screens may have more than one physical pixel per logical pixel. See the [`Engine`](400-engine.md) section for details on how to check the logical pixel dimensions of your window.

```rust,ignored
let score_text = game.add_text("score_text", "Score: 0");
score_text.translation = Vec2::new(400.0, -325.0);
```

### Rotation

`Text.rotation` is an `f32` representing the angle in radians from the positive X axis. In other words, a rotation of `0.0` results in normal, horizontal text along the X axis. A rotation of `PI` would result in upside-down text.

```rust,ignored
let angled = engine.add_text("angled", "This text is at an angle.");
score_text.rotation = std::f32::consts::PI / 4.0;
```

### Scale

`Text.scale` is an `f32`. `1.0` means matching a pixel of the source image to a pixel on the screen. `2.0` makes the image twice as wide and tall, etc.

Usually, you will want to leave text at a scale of `1.0`, but if you wish to have text zoom or shrink, modifying the scale has two important advantages compared to changing the font size:

- Changing the scale is _fast_. The text image does not need to be re-rendered, and the size change is handled all in GPU hardware.
- Changing the scale doesn't cause weird re-rendering inconsistencies, so animating scale changes looks smooth.

The main drawback of changing the scale is that since the font is not re-rendered, it looks pixellated when scaled up. Though, this could be considered as a stylistic plus as well.

```rust,ignored
let zoomed = engine.add_text("zoomed", "This text is twice as big as normal.");
score_text.scale = 2.0;
```

### Layer

`Text.layer` is an `f32` that affects what sprite or text is "on top" of another sprite or text when they overlap. `0.0` is the default layer and is on "bottom", and `999.0` is the "top" layer. The order of sprites or text on the same layer is random and unstable (can change frame to frame), so you should make sure that sprites and text that will overlap are on different layers. A good practice is to choose a few layers and assign them to constants. For example:

```rust,ignored
const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;
```

### Adjusting your newly-created text

When you create a `Text`, you get a mutable reference to the newly-created text that you can use to adjust it.

```rust,ignored
let text = engine.add_text("msg", "This is an important message.");
text.translation = Vec2::new(0.0, -300.0);
text.layer = UI_TOP_LAYER; // as in previous code snippet
```

The `Vec2` type used for the `translation` field is from `glam`, and has [its own documentation](https://docs.rs/glam/latest/glam/f32/struct.Vec2.html) you can read up on if you're interested.  The thing you'll probably use the most are its `x` and `y` fields. The code below is the same as setting `text.translation = Vec2::new(0.0, -300.0);`

```rust,ignored
text.translation.x = 0.0;
player.translation.y = -300.0;
```

NOTE: If you want to adjust your text's placement smoothly, you will need to multiply your change by the frame's delta value. See the [`Engine`](400-engine.md) section for more details.

### Adjusting an existing text

To adjust a text which already exists, you need to get a mutable reference to it.  This is where that "label" comes in.  The `Engine.texts` field is a hash map of labels to texts. You get a mutable reference to a text with the `HashMap::get_mut` method:


```rust,ignored
// Be careful with unwrap()! If the entry isn't there, this will crash your game.
let spinning_message = engine.texts.get_mut("spinning_message").unwrap();
spinning_message.rotation += TURN_SPEED_PER_SEC * engine.delta_f32;
```

### Deleting a text

To delete a text, remove it from the `Engine.texts` hash map.

```rust,ignored
engine.texts.remove("old_message");
```

