# Sprite Collisions

Rusty Engine has a basic system for detecting collisions between sprites. When two sprites with collision enabled begin or end overlapping, a [`CollisionEvent`](https://docs.rs/rusty_engine/latest/rusty_engine/physics/struct.CollisionEvent.html) will be produced. By default, collisions are disabled on sprites, so you need to set the sprite's `collision` field to `true` if you want it to emit `CollisionEvent`s.

### Processing collision events

Your game logic should process collision events each frame. Collision events which you don't handle are discarded at the end of each frame. Collision events are accessed through the `Engine.collision_events` vector.

Each `CollisionEvent` consists of a `CollisionState` (an enum with `Begin` and `End` variants) and a `CollisionPair`, which is a tuple of the labels of the two sprites involved in the collision. It is up to you to figure out what to do with the information that a collision occurred.


```rust,ignored
for event in engine.collision_events.drain(..) {
    match event.state {
        CollisionState::Begin => {
            println!("{} and {} collided!", event.pair.0, event.pair.1);
        }
        CollisionState::End => {
            println!("{} and {} are no longer colliding.", event.pair.0, event.pair.1);
        }
    }
}
```

### Colliders

Colliders are convex polygons that are used to detect if a collision has occurred between two sprites. Colliders will be rendered as polygons with white lines on the screen if `Engine.show_colliders` is set to `true`.

Colliders are stored in the same directory as the images they are for, and the collider files use the same filename as the image file the sprite uses, but with a `.collider` extension. If a valid collider file exists, it will be loaded automatically. 

### Creating colliders

All of the sprite presets in the game already have colliders, so you only have to set the `collision` field to `true` for sprite presets and you're ready to go.

If you create a new sprite using your own image, and you want it to produce `CollisionEvent`s, then you need to create a collider for that sprite.

Creating colliders from scratch is quite tedius, so there is an "example" program called `collider` that you can use to create a collider! To install and use `collider`, do the following:

```text
# Install the collider example (you only need to do this once)
$ cargo install rusty_engine --example collider

# Inside your project, run the collider example and pass it the path to your image file.
$ collider assets/db.png
```

Then follow the example's console instructions to create (or re-create) a collider and write it to a file.

<img width="1392" alt="Screen Shot 2021-12-26 at 10 45 40 PM" src="https://user-images.githubusercontent.com/5838512/147438683-c8af2db7-66dd-463c-a269-d03f37869496.png">

Once you have created the collider, [add the sprite to your game](55-sprite-creation.md) and set the `collision` field to `true`!
