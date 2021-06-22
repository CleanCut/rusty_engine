# Road Race
### Difficulty level: Easy

How far can you race without running into obstacles?

Race your car (or cars) down the road.  Your car (or cars) are on the left side of the screen facing towards the right side, and are continuously driving down a road. The road sways back and forth, indicated by the borders of the road going up or down. Hitting side barriers or obstacles in the road ends your race.

This scenario can be extended to 2 players.

## Common Setup

1. Follow the instructions in the [Common Setup]() section of the scenarios readme to set up the skeleton of the project.

## Game Initialization

In your `// setup goes here` section of `main()`...

1. Create an actor using the `.add_actor()` method of `Game`  (`.add_actor()` returns a mutable reference to the actor you can use to access its fields)
  * Label it `"player1"`
  * Use the preset `ActorPreset::RacingCarBlue`
  * `let player1 = game.add_actor("player1", ActorPreset::RacingCarBlue);`
1. Set the following attributes on the `player1` actor via the mutable reference:
  * Set `translation.x` to `-500.0` so the car will be near the left side of the screen
    * `player1.translation.x = -500.0;`
  * Set the player's `layer` to `100.0` so it will be on top of other sprites by default (higher layers are rendered on top of lower layers)
    * `player1.layer = 100.0;`
  * Set the player's `collision` to `true` so that `player1` will detect collisions with other actors.
1. Play some music in the background of the game if you like!  The recommended music for this scenario is `MusicPreset::WhimsicalPopsicle` at 20% volume.
  * `game.game_state_mut().audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);`
1. Try it! You should hear your music and see a blue race car on the left side of the screen.

## Gameplay Logic

In your `game_logic(...)` function...

Make it so you can move the player up and down
1. We need a variable in our game_state to represent which direction the race car is moving: up, down, or straight.
  1. We will add an integer entry named `"direction"` to `i32_map: HashMap<String, i32>`
  1. Here's how to get a mutable reference to that entry, initializing the entry to `0` if it doesn't exist:
    * `let direction: &mut i32 = game_state.i32_map.entry("direction".into()).or_insert(0);`
    * Now we can use `*direction` to read or write our direction integer.
  1. `1` means up (positive `y` direction). `0` means not moving up or down. `-1` means down (negative `y` direction)
  1. Try it! If you hard-code a direction (`*direction = 1;`) you should see the car move off of the screen.
    * Don't forget to remove the hard-coded direction when you are done testing!
1. Time for keyboard input! We will move the car up and down with the arrow keys (or you can pick some other keys)
  1. Add a `for` loop that loops over all the keyboard events using `.drain(..)`, which empties out the `Vec` but doesn't consume it. (We couldn't consume that vector even if we wanted to because we don't own it!)
    * `for event in game_state.keyboard_events.drain(..) {  }`
  1. Inside the for loop, do a `match` expression on `event.state` (if this block is too hard, skip down to the block of code below this step)
    * `match event.state { }`
    1. Inside the match expression, add an arm that resets `*direction` to 0 if a key was released.
      * `ElementState::Released => *direction = 0,`
    1. If a key was pressed, add a block of code for our logic to figure out whether we should go up or down.
      * `ElementState::Pressed => { },`
    1. Inside of the _pressed_ block, use an `if let` expression extract a `key_code` out of `event.key_code`, which is an `Option<KeyCode>`.
      * `if let Some(key_code) = event.key_code { }`
    1. Inside _that_ block, do a `match` on the value of `key_code`.
    1. The `KeyCode::Up` arm should set `*direction = 1`
    1. The `KeyCode::Down` arm should set `*direction = -1`
    1. A wildcard arm should do nothing (just use an empty block `{}`)
  1. Test it out by adding a `println!("{}", *direction);` to the bottom of `game_logic()` and running the game. You should see the numbers change between `0`, `1`, and `-1` as you press and release the up and down arrows.
    * Don't forget to remove this line after you are done testing!!!

```rust
// Whew! That was a complicated step. Here's all the code in game_logic() so far in case you got lost.

// Direction player1 is moving vertically. 1 is up, 0 is not moving, -1 is down.
let direction = game_state.i32_map.entry("direction".into()).or_insert(0);
// Respond to keyboard events and set the deriction
for event in game_state.keyboard_events.drain(..) {
    match event.state {
        ElementState::Pressed => {
            if let Some(key_code) = event.key_code {
                match key_code {
                    KeyCode::Up => *direction = 1,
                    KeyCode::Down => *direction = -1,
                    _ => {}
                }
            }
        }
        ElementState::Released => *direction = 0,
    }
}
```
  
1. Now let's add the movement code for `player1`
  1. Set a local variable named `speed` to some amount like `250.0`. It represents the car's vertical speed in pixels-per-second.
  1. Use an `if let` pattern to get a mutable reference to the `player1` actor.
    * `if let Some(player1) = game_state.actors.get_mut("player1") { }`
  1. Inside the `if let` block, use an `if` expression to move the player up if `*direction > 0`, or down if `*direction < 0`.
    * Don't forget to multiply your `speed` by `delta_seconds`, the amount of time that elapsed since the last frame.
    * For example, to move up you could do `player1.translation.y += speed * game_state.delta_seconds;`
  1. Try it out!
1. It would look better if the car turned when it was moving up or down. Let's set the car's rotation to a different value when it moves up down. `.rotation` is specified in radians, with `0.0` facing right, and `std::f32::consts::PI` facing left. See also `rusty_engine::consts`.
  1. Inside the `if` expression that moves the car up, add a line that sets the car's rotation slightly upwards.
    * `player1.rotation = 0.15;`
  1. Inside the `if` (or `else if`) that moves the car down, add a line that sets the car's rotation slightly downwards.
    * `player1.rotation = -0.15;`
  1. Add an `else` branch that sets the car's rotation to `0.0` if the car is not moving up or down.
    * `player1.rotation = 0.0;`
  1. Try it out!

```rust
// Did you catch all that? Here's a cheat sheet just in case:

// Move player1
let speed = 250.0;
if let Some(player1) = game_state.actors.get_mut("player1") {
    if *direction > 0 {
        player1.translation.y += speed * game_state.delta_seconds;
        player1.rotation = 0.15;
    } else if *direction < 0 {
        player1.translation.y -= speed * game_state.delta_seconds;
        player1.rotation = -0.15;
    } else {
        player1.rotation = 0.0;
    }
}
```

It doesn't really look like the car is driving down a road, yet. Let's fix that by adding painted lines on the road. Back up in the `main()` function in the `// setup goes here` section:
1. Loop 10 times: `for i in 0..10 { }`, and each time through the loop:
  1. Add an actor with the label `roadline{}` (substitute in `i` for the curly braces), and preset `ActorPreset::RacingBarrierWhite` and set:
  1. the `scale` to `0.1`
  1. the `translation.x` to `-600.0 + 150.0 * i as f32`, where `i` is the index number of the road line.
1. Add a constant near the top of your `main.rs` file called `ROAD_SPEED` and set it to `400.0`. This represents the speed our car is moving horizontally (even though it's "the road" that we're going to move)
  * `const ROAD_SPEED: f32 = 400.0;`

```rust
// In main()

// Road lines
for i in 0..10 {
    let roadline = game.add_actor(format!("roadline{}", i), ActorPreset::RacingBarrierWhite);
    roadline.scale = 0.1;
    roadline.translation.x = -600.0 + 150.0 * i as f32;
}
```


Now we need to make them move (yes, the lines will move, not the car).  Back at the bottom of the `game_logic()` function:
1. Loop through all the `actors` HashMap values with `actors.values_mut()` using a mutable reference, and:
  1. If the `actor.label` starts with `"player1"`, then:
    * Subtract `ROAD_SPEED * game_state.delta_seconds` from `translation.x`
  1. If the actor's `translation.x` is less than `-675.0` (meaning it has gone off the left side of the sreen) then add `1500.0` to it  (moving it off the right side of the screen), so it cn rush across the screen again.


```rust
// In game_logic()

// Move road objects
for actor in game_state.actors.values_mut() {
    if actor.label.starts_with("roadline") {
        actor.translation.x -= ROAD_SPEED * game_state.delta_seconds;
        if actor.translation.x < -675.0 {
            actor.translation.x += 1500.0;
        }
    }
}
```

Now it's
1. 

# Challenges

* Add a second player!
  * Variant A: The two players can overlap, harmlessly
  * Variant B: The two players are separated into their own lanes, and cannot cross to the same lane
  * Variant C: The two players crash if they touch each other. Requires implementing the "cars can move forward and backward a little" challenge for any chance at rewarding gameplay.
* Powerups!  All powerups wear off after a short time.
  * Powerup A: Boost car maneuverability
  * Powerup B: Armor - car can withstand more obstacle hits
  * Powerup C: Phase shift - car can move through obstacles
  * Powerup D: Explosion - all visible obstacles are cleared
* Hazards!  All hazards wear off after a short time.
  * Hazard A: Oil Slick - car is unable to control movement
  * Hazard B: Anti-Powerup - hitting the anti-powerup causes a new type of obstacle to appear
  * Hazard C: Afterburners - road speed increases

