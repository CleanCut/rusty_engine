# Labrinth

Guide the marble from the beginning to the end of the labrinth...but don't fall in any holes!

NOTE: This scenario is not fully supported by the capabilities of the engine. You will need to supplement the engine with your own physics logic and/or make changes to the engine itself to accomplish this complete scenario. This is included here because we _might_ add enough features to support this scenario in the future. You're certainly welcome to help!

This game consists of a [Labrinth](https://en.wikipedia.org/wiki/Labyrinth) or maze with a beginning and an end.  The marble starts at the beginning of the labrynth (naturally) and must proceed to the end. Sounds easy...until you realize that there are holes all along the maze, and you don't have perfect control of the marble! If you fall in one of the holes, start over from the beginning.

## Common Setup

1. Follow the instructions in the [Common Setup](https://github.com/CleanCut/rusty_engine/tree/main/scenarios#common-setup-do-this-first) section of the scenarios readme to set up the skeleton of the project.

## Game State

1. Define a game state struct with fields for:
   - Current tilt of the labrinth (a `Vec2`)
   - Current velocity of the marble (a `Vec2`)
   - Lives left (a `u8`)
1. Define constants for:
   - Marble movement speed (an `f32`)
   - Maximum tilt magnitude (an `f32`)
   - Maximum marble speed (an `f32`)
1. Choose a sprite to represent the player's marble
1. Choose a sprite to represent the starting area or spot
1. Choose a sprite to represent holes in the labrinth
1. Choose a sprite to represent the ending area or spot
1. Choose a sprite to represent walls of the labrinth

## Game Initialization

In your `// setup goes here` section of `main()`...

1. Use the `level_creator` example to create a labrinth (maze) with the sprite you selected for walls.
   - Place "holes" as obstacles to avoid.
   - Place one "starting area" sprite, where the marble will start on top of
   - Place one "ending area" sprite, which will signal winning the game when touched
   - Save out the game, copy and paste the sprite positioning code into your `main.rs`
1. Create the player's marble sprite and place it at the same coordinates as the "starting area" sprite, but at a high layer so it will be on top of any sprites it overlaps.
1. If you would like music, start playing it now.

## Gameplay Logic

In your [`game_logic(...)` function](https://cleancut.github.io/rusty_engine/25-game-logic-function.html)...

1. We will move the marble by virtually tilting the whole labrinth (even though it won't look like we're tilting it). The relative movement of the mouse will do the tilting.  The more tilted the labrinth is, the faster the marble will accelerate in that direction.
   - Collect [mouse movement events](https://cleancut.github.io/rusty_engine/120-mouse-events.html#mouse-motion-events) (not location events!) and add them to the current tilt of the labrinth
   - Clamp the maximum length of the tilt `Vec2` to the maximum tilt magnitude constant with [the `.clamp_length_max` method](https://docs.rs/glam/latest/glam/f32/struct.Vec2.html#method.clamp_length_max).
1. Accelerate the marble.
   - Each frame, multiply the tilt `Vec2` in the game state by the movement speed constant AND `engine.delta_f32`, and then add that resulting `Vec2` to the marble velocity in the game state. Clamp the maximum length of the marble velocity using the maximum marble velocity constant with [the `.clamp_length_max` method](https://docs.rs/glam/latest/glam/f32/struct.Vec2.html#method.clamp_length_max).
   - Each frame, increment the marble sprite's translation by the velocity in the game state.
   - At this point, you should be able to get the marble to move around the screen (though it ignores all the other sprites). Play with all the constant values until you get something that feels reasonable. For the game to be playable, you'll need a decently large max tilt magnitude paired with a relatively small movement speed and small max movement speed to give you enough control over the marble.  But you don't want _too_ much control...it should feel like rolling a marble on a flat surface by tilting it.

## The sort of unfinished part

This section isn't well-supported by the underlying engine. 😬 Sorry.

1. Make it so that you can't go through your barriers.
   - Missing engine feature: Collision contact normals.
1. Make it so that if the center of the marble overlaps a hole, you lose.
   - Missing engine feature: Testing if an arbitrary `Vec2` is within a sprite's collider.

## The rest

1. When you touch the goal, you win!


## Challenges

- When you fall down a hole, reset the game nicely and keep playing, keeping track of number of tries.
