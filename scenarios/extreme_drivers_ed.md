# Extreme Driver's Education

Can you survive your driving exam?

Navigate a driving course full of obstacles. Carefully avoid the obstacles while driving your car around to collect all of the white circles.  Only a master driver will pass this test.

- [Reference Code](https://github.com/CleanCut/rusty_engine/blob/main/examples/scenarios/extreme_drivers_ed.rs)

## Common Setup

1. Follow the instructions in the [Common Setup](https://github.com/CleanCut/rusty_engine/tree/main/scenarios#common-setup-do-this-first) section of the scenarios readme to set up the skeleton of the project.

## Level Setup

It can be _really_ tedious to set up dozens of obstacles via code and guessing coordinates. Instead, use `rusty_engine`'s `level_creator` example to place several dozen obstacles and emit the level code for you to copy-and-paste into your own project.

The sprite preset `SpritePreset::RollingHoleStart` are the goals for collecting (you _want_ to run into them). All other sprites will be obstacles.

```
# You only need to install the level creator once
cargo install rusty_engine --example level_creator

# Then use it to create a level
level_creator
```

## Engine Initialization

- 

## Game Setup

In your `// game setup goes here` section of `main`...

1.

## Game Logic

In your [`game_logic(...)` function](https://cleancut.github.io/rusty_engine/25-game-logic-function.html)...

1. 
