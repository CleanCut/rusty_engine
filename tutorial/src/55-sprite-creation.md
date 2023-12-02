# Sprite Creation

Sprites are created through the [`Engine`](400-engine.md).  Since `Game` implements `DerefMut<Engine>`, you can also call all of `Engine`'s creation methods through `Game` in your `main()` function. In either case, it looks something like this when you create a sprite with a preset:

```rust,ignored
// Through your `Game` in `main()`
let _ = game.add_sprite("my_player", SpritePreset::RacingCarBlue);

// Or later in a game logic function
let _ = engine.add_sprite("my_player", SpritePreset::RacingCarBlue);
```

All sprites in the asset pack have a "preset", which is just a fancy `enum` that makes it easy for you as a user to select one of sprite image files that are included in the default assets pack. You could also specify the image filepath, relative to the `assets/` directory, which you would do if you add your own images.  For example, the full filepath of the blue racing car is `assets/sprite/racing/car_blue.png`, so to create it by filepath you would do:

```rust,ignored
let _ = engine.add_sprite("my_player", "sprite/racing/car_blue.png");
```

`add_sprite` returns a mutable reference to a `Sprite` (`&mut Sprite`). Since it will emit a warning to silently ignore the reference, you should explicitly ignore it if you are not going to use it by doing `let _ = ...` as in the examples above. However, most of the time you will want to use the mutable reference to immediately adjust your sprite.
