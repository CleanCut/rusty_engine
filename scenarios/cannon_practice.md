# Cannon Practice

Cannons are the perfect toy for playing with ballistic trajectories!

In this scenario you will create a cannon that sits on the bottom left side of the screen. You will be able to aim the cannon and choose how much gunpowder to use. The cannonball will follow a ballistic trajectory across the screen.  Avoid various obstacles and hit your targets to score points!

## Common Setup

1. Follow the instructions in the [Common Setup](https://github.com/CleanCut/rusty_engine/tree/main/scenarios#common-setup-do-this-first) section of the scenarios readme to set up the skeleton of the project.

## Game State & Constants

1. Define a game state struct with fields for:
   - The firing magnitude of the cannon (an `f32`)
   - The rotation of the cannon (an `f32`)
   - The current velocity of the cannon ball (a `Vec2`)
1. Define a constant for acceleration due to gravity. The unit will be pixels per second per second.
1. Decide on a sprite to use as a cannon ball
1. Decide on a sprite to use as the cannon
1. Decide on a sprite to use for the goal that you are trying to hit
1. Decide on a sprite (or sprites) to use for obstacles that you should avoid hitting

## Game Initialization

In your `// setup goes here` section of `main()`...

1. Create the initial game state struct with good starting values
1. Create and place the cannon, obstacles, and goal sprites. You may use the `level_creator` example to do this, if you wish.
   - Place the cannon on the lower left side of the screen
   - Use a field from the game state to set the rotation of the cannon
   - Place a single obstacle in lower middle of the screen (so you have to fire over it)
   - Place the goal on the lower right side of the screen
1. Create the text for displaying the firing magnitude of the cannon, place it in the top left corner of the screen.
1. If you want music, start it now.

## Gameplay Logic

In your [`game_logic(...)` function](https://cleancut.github.io/rusty_engine/25-game-logic-function.html)...

1. Decide which keyboard/mouse input will control the rotation of the cannon, and implement rotating the cannon.
   - Constrain the min/max angle of rotation to angles in the first quadrant (from straight up to straight right) with [the `.clamp` method](https://doc.rust-lang.org/std/primitive.f32.html#method.clamp) and the [`UP` and `RIGHT` constants](https://docs.rs/rusty_engine/latest/rusty_engine/#constants).
1. Decide which keyboard/mouse input will fire the cannon. Implement it so that pressing (whatever you chose) creates a cannon ball sprite, but only if one does not exist. Place it at the same coordinates as the cannon, but at a layer lower than the cannon so the cannon obscures it until it is out from underneath it.
   - Play a sound when the cannon is fired.
1. Set the initial velocity `Vec2` for the cannon ball. This is fairly straightforward math:
```rust
let initial_cannonball_velocity = Vec2::new(
   game_state.firing_magnitude * cannon.rotation.cos(),
   game_state.firing_magnitude * cannon.rotation.sin(),
);
```

1. Move the cannon ball sprite by the amount stored in the game state's velocity field multiplied by `engine.delta_f32` each frame. At this point, you should be able to run the game, rotate the cannon, fire the cannon, and see the cannon move across the screen in a straight line.
1. Implement the gravity logic. Each frame, subtract (_gravity constant_ * `engine.delta_f32`) from the "Y" value of the cannon ball's velocity.
1. Decide which keyboard/mouse input will change the firing magnitude of the cannon, and implement it.
   - Constrain the firing magnitude between `0.0` and some semi-reasonable value.
   - Every time the firing magnitude changes, change the value of the `Text` that is displaying it in the top left corner of the screen. Don't change the value of the `Text` if the firing magnitude didn't change.
1. Detect collisions between the cannon ball and obstacles. The cannon ball should be destroyed if it hits an obstacle.
   1. For collisions to be detected between two sprites, the `.collision` field of _both_ sprites must be set to true. Set this field on the cannon ball, the obstacles, and the goal.
   1. Play a sound when the cannon ball hits an obstacle
1. Detect collisions between the cannon ball and the goal. The game is won if the cannon ball hits the goal.
   1. Play a sound when the cannon ball hits the goal.


## Challenge

- Introduce constant wind that varies between shots
- Make the obstactle move, rotate, or scale dynamically to make it so you have to time your shot correctly as well
- Replace the `Text` displaying the magnitude of your starting velocity with a visual slider (literally slide a barrier from the edge of the screen to some pre-defined point)
- Make destructible obstactles that reduce the cannon ball's velocity by half in the X direction
- Allow the cannon to move a small distance in the +/- X direction
- Add scorekeeping and alter the layout of the obstacles each time the cannon hits the goal
