# Car Shoot

Cars are floating past. Shoot them down!

You are at a carnival booth. Cars float across the back of the booth. The player uses their marble gun to shoot down as many of the cars as possible before the time runs out.

This scenario can be extended to 2 players.

- [Reference Code](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/car_shoot.rs)

https://user-images.githubusercontent.com/5838512/147995928-4705d9fc-c3fa-41b7-901f-d120307e455f.mp4

## Common Setup

1. Follow the instructions in the [Common Setup](https://github.com/CleanCut/rusty_engine/tree/main/scenarios#common-setup-do-this-first) section of the scenarios readme to set up the skeleton of the project.

## Engine Initialization

1. Define a `GameState` struct with the following fields:
    - `marble_labels` - a vector of strings. These will be labels for our marble sprites.
    - `cars_left` - an integer tracking how many cars are left to spawn
    - `spawn_timer` - a timer indicating when it's time to spawn another car
1. Create an instance of your `GameState` struct to `game.run()` with the following values:
    - `marble_labels` should be this vector of strings: `vec!["marble1".into(), "marble2".into(), "marble3".into()]` - we'll pop these off to use as labels for marble sprites, and then push them back on the vector when the marble sprites get destroyed. That way, we can only have three marbles in-flight at any time.
    - `cars_left` to a reasonable number such as `25`. This will be the number of cars which drive by.
    - `spawn_timer` should be a non-repeating [`Timer`](https://cleancut.github.io/rusty_engine/250-timer.html) set to `0.0` seconds so that it goes off immediately upon startup.
1. Pass your game state variable to [`game.run()`](https://cleancut.github.io/rusty_engine/450-game.html#running-the-game)

## Game Setup

In your `// game setup goes here` section of `main`...

1. (Optional) Set the [window title](https://cleancut.github.io/rusty_engine/450-game.html#window-settings) to be `Car Shoot`
1. (Optional) [Play some music.](https://cleancut.github.io/rusty_engine/205-music.html#play) We recommend the music preset `MusicPreset::Classy8Bit` at a volume of `0.1`.
1. [Create a player sprite.](https://cleancut.github.io/rusty_engine/55-sprite-creation.html) The player will be represented by a rectangle that represents the barrel of the marble gun. We'll use `SpritePreset::RacingBarrierRed`
1. We'll pretend the player is standing off the bottom of the screen, and only the barrel of their gun is visible. Let's place the sprite accordingly--set the sprite's:
    - `rotation` to `UP` so it is pointing towards the top of the screen
    - `scale` to `0.5` so it's about the right size
    - `translation.y` to `-325.0` so it goes off the bottom of the screen a little
    - `layer` to `10.0` so it will be on top of the marble which will be at a lower layer
1. [Create a `Text`](https://cleancut.github.io/rusty_engine/155-text-creation.html) with the label `"cars left"` that displays how many cars are left to spawn, with:
    - the `value` of `format!("Cars left: {}", game_state.cars_left);`
    - the `translation` of `Vec2::new(540.0, -320.0);` to put it in the bottom right corner of the screen.

## Game Logic

In your [`game_logic(...)` function](https://cleancut.github.io/rusty_engine/25-game-logic-function.html)...

1. Have the "gun barrel" follow the mouse on the X axis by set the `translation.x` of the player sprite to the `x` value of the mouse location.
    - Get a [mutable reference to the player sprite](https://cleancut.github.io/rusty_engine/60-sprite-placement.html#adjusting-an-existing-sprite)
    - Get the mouse location via the [mouse state's `location` method](https://cleancut.github.io/rusty_engine/115-mouse-state.html#location)
    - Make a variable `player_x` and set it to the player's current `translation.x` so we can use it later on: `let player_x = player.translation.x;` <img width="1392" alt="screenshot1" src="https://user-images.githubusercontent.com/5838512/147995315-e6b4108a-6344-4b38-9e11-057fb8d66017.png">
1. If the [left mouse button was just pressed](https://cleancut.github.io/rusty_engine/115-mouse-state.html#mouse-buttons), then:
    1. [If there is](https://doc.rust-lang.org/book/ch06-03-if-let.html) a label string [left in the `game_state.marble_labels` vector](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop), then:
        - Using the label value, create a new marble sprite using `SpritePreset::RollingBallBlue`
        - Make sure the label has been removed from the `game-state.marble_labels` vector. This way, we can only have as many marbles on thes screen as there are labels to remove from the vector. (We'll add the label back to the vector when we've finished with the marble).
        - Set the marble sprite's:
            - `translation.x` to the player's x location that we put in our `player_x` variable.
            - `translation.y` to `-275.0`, which will put the marble under the end of the gun.
            - `layer` to `5.0`, which will put it underneath the gun sprite
            - `collision` to `true` so that we can detect collisions between the marble and the cars.
        - [Play a sound effect](https://cleancut.github.io/rusty_engine/210-sfx.html#play) to indicate the firing of the marble. We suggest the sound effect preset `SfxPreset::Impact2` at a volume of `0.4`.
1. Move the marbles upwards (in the positive Y direction)
    1. Define a `MARBLE_SPEED` [constant](https://doc.rust-lang.org/std/keyword.const.html) (probably out in the module level) for how fast your marble should move and set it to the `f32` value of `600.0`.
    1. Loop through all the marble sprites (the sprites whose labels [start with](https://doc.rust-lang.org/std/string/struct.String.html#method.starts_with) `"marble"`), for each of them:
        - increment the marble sprite's `translation.y` by `MARBLE_SPEED * engine.delta_f32`
1. Clean up sprites that have moved off the top or the right side of the screen.
    1. We can't modify a hash map of sprites while we're looping through its values, so let's create an empty vector of strings and fill it with labels of sprites that we want to delete. Once we're done examining the hash map, we can loop through the vector of labels and remove those hash map entries.
    1. Create a new vector `labels_to_delete`
    1. For every sprite [value in the hash map](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.values):
        - check to see if either the `translation.y > 400.0` or the `translation.x > 750.0`. If either of those conditions are true, push a clone of the label onto the `labels_to_delete` vector.
    1. For every label in `labels_to_delete`:
        - [Remove the sprite entry.](https://cleancut.github.io/rusty_engine/60-sprite-placement.html#deleting-a-sprite) The hash map's `remove` method takes an immutable reference to the key type, so if you are looping through the label strings by value, you may need to add a `&` in front of your label variable: `engine.sprites.remove(&label)`
        - If the label starts with `marble`, then push the label onto the `game_state.marble_labels` vector. That way we'll be able to shoot another marble.
1. Spawn a car if the `game_state.spawn_timer` just finished! So [tick the spawn timer and check to see if it just finished](https://cleancut.github.io/rusty_engine/250-timer.html#counting-down--finishing) -- if it did, then:
    1. Set `game_state.spawn_timer` to a new `Timer` with a random value between `0.1` and `1.25`
        - Add the `rand` crate as a dependency in your `Cargo.toml`
        - Add `use rand::prelude::*;` to the top of your `main.rs` file
        - Use `thread_rng().gen_range(0.1..1.25)` to obtain a random `f32` value between `0.1` and `1.25`
        - [Create a non-repeating `Timer`](https://cleancut.github.io/rusty_engine/250-timer.html#creation) and assign it as the value to `game_state.spawn_timer`
    1. If there are any cars left (check the value of `game_state.cars_left`), then:
        1. Decrement `game_state.cars_left` by one
        1. [Retrieve a mutable reference to the](https://cleancut.github.io/rusty_engine/165-text-placement.html#adjusting-an-existing-text) `Text` we labeled `"cars left"`
            - Set the `value` to `format!("Cars left: {}", game_state.cars_left)`
        1. Create a label for the current car that starts with `car`: `format!("car{}", game_state.cars_left)` (remember, a label starting with `car` is what the movement code is looking for).
        1. Create a vector of `SpritePreset`s of cars to randomly select from: `let car_choices = vec![SpritePreset::RacingCarBlack, SpritePreset::RacingCarBlue, SpritePreset::RacingCarGreen, SpritePreset::RacingCarRed, SpritePreset::RacingCarYellow];`
        1. Make a random sprite preset choice: `car_choices.iter().choose(&mut thread_rng()).unwrap().clone()`
        1. Actually create the sprite with the label and sprite preset selected above. Set the sprite's:
            - `translation.x` to `-740.0`
            - `translation.y` to a random value from `-100.0` to `325.0` -- `thread_rng().gen_range(-100.0..325.0)`
            - `collision` to `true` so that the car will collide with marbles
1. Move cars right across the screen (in the positive X direction). The logic for this section is _very_ similar to the previous section that moved marbles.
    1. Define a `CAR_SPEED` constant and set it to `250.0`
    1. Loop through all the car sprites (the sprites whose labels start with `"car"`), for each of them:
        - increment the car sprite's `translation.x` by `CAR_SPEED * engine.delta_f32`
1. Now it's time to handle the collisions! For each [`CollisionEvent`](https://docs.rs/rusty_engine/latest/rusty_engine/physics/struct.CollisionEvent.html) in `engine.collision_events`:
    - We only care about the start of collisions, not the ending of them, so if `event.state.is_end()`, then `continue` the loop.
    - Similarly, if one of the event pair's labels _doesn't_ start with `"marble"`, then it's either two marbles or two cars colliding with each other, which we don't care about. So if `!event.pair.one_starts_with("marble")`, then `continue` the loop.
    - At this point we know that one of the pair is a marble and the other is a car, and they both need to be removed. So using the labels in the `event.pair` tuple, [delete both sprites](https://cleancut.github.io/rusty_engine/60-sprite-placement.html#deleting-a-sprite).
    - Now that a marble has been "destroyed", we are allowed to shoot it from the gun again, so grab whichever label of the `event.pair` tuple that starts with `"marble"` and [push](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) a clone of it back onto the `game_state.marble_labels` vector.
    - [Play a sound effect](https://cleancut.github.io/rusty_engine/210-sfx.html#play) for successfully hitting a car with a marble. Use `SfxPreset::Confirmation1` with a volume of `0.2`

        
You made it to the end of the main scenario! You should have a playable game prototype by this point.

<img width="1392" alt="screenshot2" src="https://user-images.githubusercontent.com/5838512/147995324-1ba02e86-86d1-4456-bb67-ba03ffeaff89.png">

# Challenges

* Keep track of points, display the points in a corner of the screen
* Make it so that after the game ends, you can press a key and start a new game
* Keep track of the high score across games and display it when the game ends
* Don't allow cars to spawn on top of other cars
* Powerups! Powerups float across like cars and activate when hit.
    * Spread-fire
    * Rapid-fire
    * Explosion - clear the screen
* Make the movement of the cars more interesting - have them drive in curvy motions
* Smart black cars - black cars sometimes slow down or speed up so a shot will miss them
* Armored cars - Green cars take two marbles to take down
* Add support for a second player, with separate scores for each player. You'll need to figure out some way for the second player to control their marble gun...
