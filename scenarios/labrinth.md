# Labrinth

Guide the marble from the beginning to the end of the labrinth...but don't fall in any holes!

This game consists of a [Labrinth](https://en.wikipedia.org/wiki/Labyrinth) or maze with a beginning and an end.  The marble starts at the beginning of the labrynth (naturally) and must proceed to the end. Sounds easy...until you realize that there are holes all along the maze, and you don't have perfect control of the marble! If you fall in one of the holes, start over from the beginning.

## Common Setup

1. Follow the instructions in the [Common Setup](https://github.com/CleanCut/rusty_engine/tree/main/scenarios#common-setup-do-this-first) section of the scenarios readme to set up the skeleton of the project.

## Game State

1. Define a game state struct with fields for:
   - Current velocity of the marble (a `Vec2`)
   - Lives left (a u8)
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

In your `game_logic(...)` function...

1. We will move the marble by virtually tilting the whole labrinth. The relative movement of the mouse will do the tilting.  The more tilted the labrinth is, the faster the marble will accelerate in that direction.
