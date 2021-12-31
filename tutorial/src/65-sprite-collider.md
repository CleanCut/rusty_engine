# Sprite Collider

Rusty Engine has an basic collision system. You may define one convex polygon shape to be a collider for a sprite. When two sprites with colliders whose `collision` fields are both set to `true` begin or end overlapping, a [`CollisionEvent`](https://docs.rs/rusty_engine/latest/rusty_engine/physics/struct.CollisionEvent.html) will be produced.  If either of the sprites lacks a collider, or if either of the sprites has their `collision` field set to `false`, then no collision event is produced.

Colliders will be rendered as lines on the screen if `EngineState.debug_sprite_colliders` is set to `true`.

Colliders are stored in files with the same name and path as the image file they are for, but with the `.collider` extension. If a valid collider file exists when a sprite is created, it will be loaded automatically. However, the `collision` field for a sprite always defaults to `false`, so you must opt in to the collision system by setting `collision` to `true` on your sprites.

### Processing collision events

Your game logic should process collision events each frame. Collision events which you don't handle are discarded at the end of each frame. Collision events are accessed through the `EngineState.collision_events` vector.

Each `CollisionEvent` consists of a `CollisionState` (an enum of either `Begin` or `End`) and a `CollisionPair`, which is a tuple of the labels of the two sprites involved in the collision. It is up to you to figure out what to do with the information that a collision occurred.


```rust,ignored
for event in engine_state.collision_events.drain(..) {
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

### Creating colliders

All of the sprite presets in the game already have colliders, so you don't need to worry about creating any of them.

Creating colliders for custom sprites from scratch can be quite difficult, so there is an "example" program called `collider_creator` that you can run to create the collider by clicking around a sprite!  Clone the [`rusty_engine`](https://github.com/CleanCut/rusty_engine/) repository, place your image file in the `assets/sprite` directory (let's call it `db.png`), and then run:

```text
$ cargo run --release --example collider_creator -- db.png
```

Then follow the directions to create (or re-create) a collider and write it to a file.

<img width="1392" alt="Screen Shot 2021-12-26 at 10 45 40 PM" src="https://user-images.githubusercontent.com/5838512/147438683-c8af2db7-66dd-463c-a269-d03f37869496.png">

Once you have a good collider created, copy (or move) both your image and `.collider` file to your own project, under the `assets/sprite` directory.
