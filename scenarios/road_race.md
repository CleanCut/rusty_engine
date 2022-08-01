# Road Race
### Difficulty level: Easy

How far can you race your car without running into too many obstacles?

Race your car down the road.  Your car is on the left side of the screen facing towards the right side, and is continuously driving down a road. Hitting obstacles lowers your health. If your health reaches zero, the race ends.

- [Reference Code](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/road_race.rs)

## Common Setup

1. Follow the instructions in the [Common Setup](https://github.com/CleanCut/rusty_engine/tree/main/scenarios) section of the scenarios readme to set up the skeleton of the project.

## Health

1. We need to keep track of the player's health, so we'll store it in our `GameState` struct. Add a `health_amount: u8` field to the `GameState` struct definition.
1. We want to stop the game when the player loses. Add a `lost: bool` field to the `GameState` struct definition.
1. When you run the game, provide `5` for the initial value of the `health_amount` field, and `false` for the `lost` field. You can hit five obstacles before you lose the game, and you haven't lost yet.
1. Try it! At this point you should still be presented with a blank window, but it should compile and run!
    * `cargo run --release`

```rust
struct GameState {
    health_amount: u8,
    lost: bool,
}

// ...inside main()
game.run(GameState {
    health_amount: 5,
    lost: false,
});
```

## Create Player Sprite

In your `// game setup goes here` section of `main()`...

1. Create a sprite using the [`.add_sprite()`](https://cleancut.github.io/rusty_engine/55-sprite-creation.html) method of `Game`  (`.add_sprite()` returns a mutable reference to the sprite you can use to access its fields)
    1. For the first argument, use the label `"player1"`
    1. For the second argument, use the preset `SpritePreset::RacingCarBlue`
1. Set the following [attributes](https://cleancut.github.io/rusty_engine/60-sprite-placement.html) on the `player1` sprite via the mutable reference returned by `.add_sprite()`:
    1. Set `translation.x` to `-500.0` so the car will be near the left side of the screen
    1. Set `layer` to `10.0` so that the sprite will be on top of other sprites which we will put on lower layers.
    1. Set `collision` to `true` so that the player sprite will be able to collide with other sprites.
1. [Play some music](https://cleancut.github.io/rusty_engine/205-music.html) in the background of the game if you like!  The recommended music for this scenario is `MusicPreset::WhimsicalPopsicle` at 20% volume.
1. Try it! You should hear your music and see a blue race car on the left side of the screen.
    * `cargo run --release`

```rust
// Create the player sprite
let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
player1.translation.x = -500.0;
player1.layer = 10.0;
player1.collision = true;

// Start some background music
game.audio_manager
    .play_music(MusicPreset::WhimsicalPopsicle, 0.2);
```

<img width="1392" alt="screenshot1" src="https://user-images.githubusercontent.com/5838512/147838667-ea202119-77db-40b4-bdc1-404b27e9c5e8.png">

## Player input

Let's look at the player input and store it for using to move the player later. This section is all done in your [game logic function](https://cleancut.github.io/rusty_engine/25-game-logic-function.html), which is called once every frame.

1. Make a mutable variable `direction` of type `f32` and initialize it to `0.0`.
    1. `1.0` means up (positive `y` direction). `0.0` means not moving up or down. `-1.0` means down (negative `y` direction)
1. Time to collect some input from the [keyboard state](https://cleancut.github.io/rusty_engine/105-keyboard-state.html)!
    1. If `KeyCode::Up` is pressed, then add `1.0` to `direction`. (`engine.keyboard_state.pressed(KeyCode::Up)`)
    1. If `KeyCode::Down` is pressed, then subtract `1.0` from `direction`
    1. At this point, `direction` should be `1.0` if the up arrow is pressed, `-1.0` if the down arrow is pressed, and `0.0` if neither or both are pressed.
1. Now we can actually move the player!
    1. First, define a constant near the top of your file at module level named `PLAYER_SPEED` and set it to something like `250.0` (you can set it higher or lower depending on your preference for how fast you can move). This is our up-and-down movement speed in pixels per second.
    1. Back in `game_logic`, get a mutable reference to the player sprite. We used the label "player1", and it's in the `Engine.sprites` [hash map](https://doc.rust-lang.org/std/collections/struct.HashMap.html#).
    1. Change the player's `translation.y` by `direction * PLAYER_SPEED * engine.delta_f32`
        * Multiplying by `direction` will leave the result positive if we're moving up, reverse the sign to negative if we're moving down, or zero out the entire product if we're not moving.
        * Multiplying by `engine.delta_f32` will make it so we move smoothly even as frame rates jitter around.
    1. If the car is moving up (or down), let's rotate it a bit so it looks like it actually turned in that direction. Set the player sprite's `rotation` to `direction * 0.15`
    1. If the (center of the) player leaves the top or bottom of the screen, then the player dies. Set the game state's `health_amount` to `0` if the player's `translation.y` value is greater than `360.0` or less than `-360.0`
1. Try it out! Should be able to move the car up and down, seeing the car turn a bit while it does it.
    * `cargo run --release`
 
```rust
// At the top of your file...
const PLAYER_SPEED: f32 = 250.0;


// In your `game_logic` function from here on down...

// Collect keyboard input
let mut direction = 0.0;
if engine.keyboard_state.pressed(KeyCode::Up) {
    direction += 1.0;
}
if engine.keyboard_state.pressed(KeyCode::Down) {
    direction -= 1.0;
}

// Move the player sprite
let player1 = engine.sprites.get_mut("player1").unwrap();
player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
player1.rotation = direction * 0.15;
if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
    game_state.health_amount = 0;
}
```

<img width="1392" alt="screenshot2" src="https://user-images.githubusercontent.com/5838512/147838668-a6eb80ec-e0fe-465e-a0dc-f59185a9c11a.png">


## The Road

It doesn't really look like the car is driving down a road, yet. Let's fix that by adding painted lines on the road. Back up in the `main()` function in the `// game setup goes here` section:
1. Loop 10 times: `for i in 0..10 { }`, and each time through the loop:
    1. Add a sprite with the label `roadline{}` (substitute in `i` for the curly braces using the `format!()` macro), and the preset `SpritePreset::RacingBarrierWhite` and then set:
    1. the `scale` to `0.1`
    1. the `translation.x` to `-600.0 + 150.0 * i as f32`, where `i` is the index number of the road line.

```rust
// In your `main` function...

// Create the road lines
for i in 0..10 {
    let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
    roadline.scale = 0.1;
    roadline.translation.x = -600.0 + 150.0 * i as f32;
}
```

Now we need to make the lines move (yes, the lines will move to the left instead of the car moving to the right to create the illusion of moving down a road). 
1. Add a constant near the top of your `main.rs` file called `ROAD_SPEED` and set it to `400.0`. This represents the speed our car is moving horizontally (even though it's "the road" that we're going to move)
    * `const ROAD_SPEED: f32 = 400.0;`
1. Back at the bottom of the `game_logic` function, loop through all the `sprites` HashMap values with `engine.sprites.values_mut()` using a mutable reference, and:
    1. If the `sprite.label` starts with `"roadline"`, then:
       * Subtract `ROAD_SPEED * engine.delta_f32` from the sprite's `translation.x`
    1. If the sprite's `translation.x` is less than `-675.0` (meaning it has gone off the left side of the screen) then add `1500.0` to it  (moving it off the right side of the screen), so it can rush across the screen again.


```rust
// At the top of your file...
const ROAD_SPEED: f32 = 400.0;


// In `game_logic`

// Move road objects
for sprite in engine.sprites.values_mut() {
    if sprite.label.starts_with("roadline") {
        sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
        if sprite.translation.x < -675.0 {
            sprite.translation.x += 1500.0;
        }
    }
}
```

<img width="1392" alt="screenshot3" src="https://user-images.githubusercontent.com/5838512/147838670-6fa05611-7b97-4857-b787-316b9aa1fd7b.png">

## Obstacles

Now it's time to add some obstacles. Interesting obstacles will be in random locations, so first we need to:
1. Add `rand` to the `[dependencies]` section of `Cargo.toml`
1. Add `use rand::prelude::*` to the top of your `main.rs` file.
1. In the `main()` function, in your `// game setup goes here` section:
1. Make a vector of some `SpritePreset` variants to use as obstacles.
    * `let obstacle_presets = vec![SpritePreset::RacingBarrelBlue, SpritePreset::RacingBarrelRed, SpritePreset::RacingConeStraight];`
    * You can add more variants than that, depending how challenging you would like the game to be.
1. Loop through the presets, enumerating them so you have their index.
    * `for (i, preset) in obstacle_presets.into_iter().enumerate() { }`
    1. For each preset:
        1. Add a sprite with that preset and a label that starts with `"obstacle"`, and ends with the number value of `i`. (Use the `format!()` macro to construct the label string).
        1. Set the sprite's `layer` to `5.0` so that the obstacle will be on top of road lines, but underneath the player.
        1. set the sprite's `collision` to `true` so that it will generate collision events with the race car.
        1. Set the `x` location to a random value between `800.0` and `1600.0` using `thread_rng()`
        * `sprite.translation.x = thread_rng().gen_range(800.0..1600.0);`
        1. Do the same for `y`, only between `-300.0` and `300.0`

```rust
// in `main`
let obstacle_presets = vec![RacingBarrelBlue, RacingBarrelRed, RacingConeStraight];
for (i, preset) in obstacle_presets.into_iter().enumerate() {
    let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
    obstacle.layer = 5.0;
    obstacle.collision = true;
    obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
    obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
}
```

The obstacles need to move, too, so they appear to be on the road!  In the `game_logic()` function:
1. Inside the same loop that moves the road lines, add another `if` expression that does the following if the sprite's `label` starts with `"obstacle"`:
    1. Updates the sprite's `translation.x` the same as we did with the road lines
    1. If the `translation.x` value is less than `-800.0`, then set both the `x` and `y` values to new random values in the same ranges as before (you can copy-and-paste the same lines setting random values as in the last section)
1. Try it out! Obstacles should now appear, which the race car can avoid.
    * `cargo run --release`

```rust
// inside the same loop in `game_logic` that moves the road lines
if sprite.label.starts_with("obstacle") {
    sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
    if sprite.translation.x < -800.0 {
        sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
        sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
    }
}
```


<img width="1392" alt="with obstacles" src="https://user-images.githubusercontent.com/5838512/170616371-0833ef53-8aec-4b2d-8bf1-88adb64afe28.png">

## Health

Let's get ready to handle the player's health.
1. In `main`, add a new `Text` using [the `add_text()` method](https://cleancut.github.io/rusty_engine/155-text-creation.html) with the label `"health_message"` and the text `"Health: 5"`.
    * Set the text's `translation` to `Vec2::new(550.0, 320.0)`
1. If the player's health reaches `0`, we will pause the game and let the player consider their poor life choices.

```rust
// in `main`...

// Create the health message
let health_message = game.add_text("health_message", "Health: 5");
health_message.translation = Vec2::new(550.0, 320.0);
```

Now we need to actually handle the health.  At **_the bottom_** of the `game_logic` function we'll deal with collisions:
1. Get a mutable reference to the health message
    * `let health_message = engine.texts.get_mut("health_message").unwrap();`
1. Loop through all the collision events in a `for` loop using `engine.collision_events.drain(..)`
    1. Ignore events (by doing a `continue` in the `for` loop) that contain `"player1"` in the collision pair OR where the event state is the ending of a collision.
        * `if !event.pair.either_contains("player1") || event.state.is_end() { continue; }`
    1. If `game_state.health_amount` is greater than `0` (we don't want to try to subtract from an unsigned number without checking first)
        1. Subtract `1` from `game_state.health_amount`
        1. Set the `value` of the `health_message` to the string `"Health: {}"`, where `{}` is the value of `game_state.health_amount`.
        1. Use the `audio_manager` to play `SfxPreset::Impact3` at a volume of `0.5`.
1. Try it!  The game should mostly work, except that after you get to zero health, you can keep on driving forever.
    * `cargo run --release`

```rust
// Deal with collisions
let health_message = engine.texts.get_mut("health_message").unwrap();
for event in engine.collision_events.drain(..) {
    // We don't care if obstacles collide with each other or collisions end
    if !event.pair.either_contains("player1") || event.state.is_end() {
        continue;
    }
    if game_state.health_amount > 0 {
        game_state.health_amount -= 1;
        health_message.value = format!("Health: {}", game_state.health_amount);
        engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
    }
}
```

<img width="1392" alt="screenshot4" src="https://user-images.githubusercontent.com/5838512/147838672-f442f587-340a-4623-b6ca-f4cda37e0f86.png">


## Game Over

First, at the very _top_ of the `game_logic function let's stop the game if we have lost.
1. If `game_state.lost` is `true` then `return` from the game logic function. This will effectively "pause" everything, since none of the rest of our game logic will run.

```rust
// Don't run any more game logic if the game has ended
if game_state.lost {
    return;
}
```

Finally, at the very _bottom_ of the `game_logic` function we need to detect whether we lost and clean up a few things if we did.
1. If `game.health_amount` is `0`
    1. Set `game_state.lost` to `true`
    1. Create a `Text`, and set its value to `"Game Over"`
    1. Using the mutable reference from creating the text, set its `font_size` to `128.0` (if this crashes on your system, reduce the font size to a smaller number)
    1. Use the `audio_manager` to stop the music.
    1. Use the `audio_manager` to [play the sound effect](https://cleancut.github.io/rusty_engine/210-sfx.html) `SfxPreset::Jingle3` at half volume (it's a sad sound)
1. Try it!
    * `cargo run --release`

```rust
if game_state.health_amount == 0 {
    game_state.lost = true;
    let game_over = engine.add_text("game over", "Game Over");
    game_over.font_size = 128.0;
    engine.audio_manager.stop_music();
    engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
}
```

<img width="1392" alt="screenshot5" src="https://user-images.githubusercontent.com/5838512/147838675-a3d85aef-c2e5-4257-aef1-51ca983d3044.png">

That's it! You've done it!  At this point you should have a fully-functional game prototype.  Feel free to continue changing things and having some fun.  Below is a list of challenge ideas to get you thinking about other things you could do.

# Troubleshooting

Having trouble getting the scenario above to work?  Check out the [reference implementation](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/road_race.rs)

# Challenges

* Add a second player!
    * Variant A: The two players can overlap, harmlessly
    * Variant B: The two players are separated into their own lanes, and cannot cross to the same lane
    * Variant C: The two players crash if they touch each other. Requires implementing the "cars can move forward and backward a little".
* Powerups!  All powerups wear off after a short time, so you'll need to use `Timer`s
    * Powerup A: Boost car maneuverability
    * Powerup B: Armor - car can withstand more obstacle hits
    * Powerup C: Phase shift - car can move through obstacles
    * Powerup D: Explosion - all visible obstacles are cleared
* Hazards!  All hazards wear off after a short time, so you'll need to use `Timer`s
    * Hazard A: Oil Slick - car is unable to control movement
    * Hazard B: Anti-Powerup - hitting the anti-powerup causes a new type of obstacle to appear
    * Hazard C: Afterburners - road speed increases
* Polish
    * Make the car turn (rotate) smoothly instead of suddenly, and have the speed the car moves in the y direction vary proportianally to how much the car is rotated.
    * Randomize the rotation of the obstacles
    * Add support for driving the car via mouse input
    * Add controls to change or turn off the music
    * Add text indicators of things that happen (collisions, powerups, etc.)
    * Add the ability to pause and resume the game
    * Collect points for every obstacle that you pass, display the score in the corner during the game, add an end screen showing the final score.
    * Instead of ignoring obstacles that collide with each other, take one of the colliding items and set it to a new random starting position.
    * Make it so you can't hit the same obstacle twice (currently if you wiggle back and forth you can run into the same obstacle multiple times)

