# Text Value, Font & Font Size

Changing the string value, the chosen font, or the font size causes the `Text` to be re-rendered as a new image at the end of the frame. This is relatively expensive in terms of performance, so you should avoid changing these attributes except when you actually need to.

All existing text values can be accessed through the `Engine` struct's `texts` field, which is a vector of `Text`s.


### Value

The `Text` struct's `value` field is the actual string that gets rendered to the screen. If you change the value, then the `Text` will be re-rendered as a new image at the end of the frame with the new value.

```rust,ignored
let score_text = engine.texts.get_mut("score_text").unwrap();
score_text.value = format!("Score: {}", score); // The `format` macro produces a String.
```

### Font

If you change the font, then the `Text` will be re-rendered as a new image at the end of the frame with the new value.

The asset pack contains two fonts:

- `font/FiraMono-Medium.ttf`
- `font/FiraSans-Bold.ttf` (the default font if none is specified)


```rust,ignored
let mono = engine.add_text("mono", "This text is using a monospace font");
mono.font = "font/FiraMono-Medium.ttf".to_string();
```

To use a custom font, place a valid `otf` or `ttf` file in `assets/` and set it on your `Text`.

```rust,ignored
// After placing `party.otf` in the `assets/` directory...
let party = engine.add_text("party", "Let's Party!");
mono.font = "party.otf".to_string();
```

If you specify a font file which can't be loaded successfully, you will get an console error like this:

```text
Dec 30 15:15:20.624  WARN bevy_asset::asset_server: encountered an error while reading an asset: path not found: /Users/nathan/rust/rusty_engine/assets/font/nonexistent.ttf
```

### Font Size

If you change the font size, then the `Text` will be re-rendered as a new image at the end of the frame with the font size.

The default font size is `30.0`. Setting the font size doesn't require a lot of explanation:

```rust,ignored
let large = engine.add_text("large", "This is a large font size!");
mono.font_size = 96.0;
```
