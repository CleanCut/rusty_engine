//! Rusty Engine's custom collision detection implementation.

use crate::sprite::Sprite;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    f32::consts::{PI, TAU},
    hash::Hash,
};

pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, collision_detection);
    }
}

// TODO: Replace the handmade 2D overlap detection with real rapier2d physics
// can now be multiline.

/// This is the struct that is generated when a collision occurs. Collisions only occur between two
/// [Sprite]s which:
/// - have colliders (you can use the `collider` example to create your own colliders)
/// - have their `collision` flags set to `true`.
#[derive(Clone, Debug, PartialEq, Eq, Event)]
pub struct CollisionEvent {
    pub state: CollisionState,
    pub pair: CollisionPair,
}

/// Indicates whether a [`CollisionEvent`] is at the beginning or ending of a collision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionState {
    Begin,
    End,
}

impl CollisionState {
    /// Returns true if the value is [`CollisionState::Begin`]
    pub fn is_begin(&self) -> bool {
        match self {
            CollisionState::Begin => true,
            CollisionState::End => false,
        }
    }
    /// Returns true if the value is [`CollisionState::End`]
    pub fn is_end(&self) -> bool {
        match self {
            CollisionState::Begin => false,
            CollisionState::End => true,
        }
    }
}

/// Contains the labels of the two sprites involved in the collision. As the labels are unordered,
/// several convenience methods are provided for searching the values.
#[derive(Debug, Default, Eq, Clone)]
pub struct CollisionPair(pub String, pub String);

impl CollisionPair {
    /// Whether either of the labels contains the text.
    pub fn either_contains<T: Into<String>>(&self, text: T) -> bool {
        let text = text.into();
        self.0.contains(&text) || self.1.contains(&text)
    }
    /// Whether either of the labels equals to the text.
    pub fn either_equals_to<T: Into<String>>(&self, text: T) -> bool {
        let text = text.into();
        (self.0 == text) || (self.1 == text)
    }
    /// Whether either of the labels starts with the text.
    pub fn either_starts_with<T: Into<String>>(&self, text: T) -> bool {
        let text = text.into();
        self.0.starts_with(&text) || self.1.starts_with(&text)
    }
    /// Whether exactly one of the labels starts with the text.
    pub fn one_starts_with<T: Into<String>>(&self, text: T) -> bool {
        let text = text.into();
        let a_matches = self.0.starts_with(&text);
        let b_matches = self.1.starts_with(&text);
        (a_matches && !b_matches) || (!a_matches && b_matches)
    }

    pub fn array(&self) -> [&str; 2] {
        [self.0.as_str(), self.1.as_str()]
    }
    pub fn array_mut(&mut self) -> [&mut String; 2] {
        [&mut self.0, &mut self.1]
    }
}

impl IntoIterator for CollisionPair {
    type Item = String;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;
    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1].into_iter()
    }
}

impl PartialEq for CollisionPair {
    fn eq(&self, other: &Self) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

impl Hash for CollisionPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Make sure we return the same hash no matter which position the same two strings might be
        // in (so we match our PartialEq implementation)
        if self.0 < self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

/// system - detect collisions and generate the collision events
fn collision_detection(
    mut existing_collisions: Local<HashSet<CollisionPair>>,
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<&Sprite>,
) {
    let mut current_collisions = HashSet::<CollisionPair>::new();
    'outer: for sprite1 in query.iter().filter(|a| a.collision) {
        for sprite2 in query.iter().filter(|a| a.collision) {
            if sprite1.label == sprite2.label {
                // We only need to compare one half of the matrix triangle
                continue 'outer;
            }
            if Collider::colliding(sprite1, sprite2) {
                current_collisions
                    .insert(CollisionPair(sprite1.label.clone(), sprite2.label.clone()));
            }
        }
    }

    let beginning_collisions: Vec<_> = current_collisions
        .difference(&existing_collisions)
        .cloned()
        .collect();

    collision_events.send_batch(beginning_collisions.iter().map(|p| CollisionEvent {
        state: CollisionState::Begin,
        pair: p.clone(),
    }));

    for beginning_collision in beginning_collisions {
        existing_collisions.insert(beginning_collision);
    }

    let ending_collisions: Vec<_> = existing_collisions
        .difference(&current_collisions)
        .cloned()
        .collect();

    collision_events.send_batch(ending_collisions.iter().map(|p| CollisionEvent {
        state: CollisionState::End,
        pair: p.clone(),
    }));

    for ending_collision in ending_collisions {
        let _ = existing_collisions.remove(&ending_collision);
    }
}

/// Represents the collider (or lack thereof) of a sprite. Two sprites need to have colliders AND
/// have their `Sprite.collision` fields set to `true` to generate collision events. See the
/// `collider` example to create your own colliders
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum Collider {
    #[default]
    NoCollider,
    Poly(Vec<Vec2>),
}

impl Collider {
    /// Generate a rectangular collider based on top-left and bottom-right points
    pub fn rect<T: Into<Vec2>>(topleft: T, bottomright: T) -> Self {
        let topleft = topleft.into();
        let bottomright = bottomright.into();
        Self::Poly(vec![
            topleft,
            Vec2::new(bottomright.x, topleft.y),
            bottomright,
            Vec2::new(topleft.x, bottomright.y),
        ])
    }
    /// Convert a slice of Vec2's into a polygon collider. This is helpful if you want to hard-code
    /// colliders in your code as arrays or vectors of Vec2.
    pub fn poly<T: Into<Vec2> + Copy>(points: &[T]) -> Self {
        Self::Poly(points.iter().map(|&x| x.into()).collect())
    }
    /// Generate a polygon circle approximation with the specified radius and amount of vertices
    pub fn circle_custom(radius: f32, vertices: usize) -> Self {
        let mut points = vec![];
        for x in 0..vertices {
            let inner = std::f64::consts::TAU / vertices as f64 * x as f64;
            let mut inner_x = inner.cos() as f32 * radius;
            let mut inner_y = inner.sin() as f32 * radius;
            // Clamp near-zero values to zero when producing RON files: (-0.0000000000000044087286)
            if (inner_x > -0.000001) && (inner_x < 0.000001) {
                inner_x = 0.0;
            }
            if (inner_y > -0.000001) && (inner_y < 0.000001) {
                inner_y = 0.0;
            }
            points.push(Vec2::new(inner_x, inner_y));
        }
        Self::Poly(points)
    }
    /// Generate a 16-vertex polygon circle approximation. 16 was chosen as the default as it works
    /// quite well with the circular sprites in the asset pack.
    pub fn circle(radius: f32) -> Self {
        Self::circle_custom(radius, 16)
    }
    /// Whether or not the collider is a `Collider::Poly`.
    pub fn is_poly(&self) -> bool {
        matches!(self, Self::Poly(_))
    }
    /// Whether the points in the collider represent a convex polygon (not concave or complex).
    /// This is important, because Rusty Engine's collision detection doesn't work correctly unless
    /// colliders are convex polygons.
    ///
    /// This implementation is based on Rory Daulton's answer on https://stackoverflow.com/questions/471962/how-do-i-efficiently-determine-if-a-polygon-is-convex-non-convex-or-complex?answertab=votes#tab-top
    pub fn is_convex(&self) -> bool {
        if let Collider::Poly(points) = self {
            let length = points.len();
            if length < 3 {
                return false; // empty sets, points and lines are not convex polygons
            }
            // the source algorithm deals with individual x's and y's and the combined points in
            // disjoint ways, so we need to follow the pattern unless we want to modify the
            // algorithm itself.
            let mut old_x = points[length - 2].x;
            let mut old_y = points[length - 2].y;
            let mut new_x = points[length - 1].x;
            let mut new_y = points[length - 1].y;
            let mut new_direction = (new_y - old_y).atan2(new_x - old_x);
            let mut angle_sum = 0.0;
            let mut old_direction;
            let mut orientation = 0.0;
            for (idx, newpoint) in points.iter().enumerate() {
                // The fact that new_x and new_y are re-used at the top of the loop with the
                // expectation that they have the last loop's values is why we can't use the
                // newpoint loop variable directly. Messy. :-/
                old_x = new_x;
                old_y = new_y;
                old_direction = new_direction;
                new_x = newpoint.x;
                new_y = newpoint.y;
                new_direction = (new_y - old_y).atan2(new_x - old_x);
                if (old_x == new_x) && (old_y == new_y) {
                    return false; // repeated consecutive points
                }
                // Calculate & check the normalized deriction-change angle
                let mut angle = new_direction - old_direction;
                if angle <= -PI {
                    angle += TAU; // make it in half-open interval (-Pi, Pi]
                } else if angle > PI {
                    angle -= TAU;
                }
                if idx == 0 {
                    // if first time through loop, initialize orientation
                    if angle == 0.0 {
                        return false; // the source algorithm doesn't explain this one
                    }
                    if angle > 0.0 {
                        orientation = 1.0;
                    } else {
                        orientation = -1.0;
                    }
                } else if orientation * angle <= 0.0 {
                    // not both positive or both negative
                    return false;
                }

                // Accumulate the direction-change angle
                angle_sum += angle;
            }
            // Check that the total number of full turns is plus-or-minus 1
            let full_turns = (angle_sum / TAU).abs();
            return (full_turns > 0.9999) && (full_turns < 1.0001);
        }
        false
    }
    /// Return the points rotated by a number of radians
    fn rotated(&self, rotation: f32) -> Vec<Vec2> {
        let mut rotated_points = Vec::new();
        if let Self::Poly(points) = self {
            let sin = rotation.sin();
            let cos = rotation.cos();
            for point in points.iter() {
                rotated_points.push(Vec2::new(
                    point.x * cos - point.y * sin,
                    point.x * sin + point.y * cos,
                ));
            }
        }
        rotated_points
    }
    #[doc(hidden)]
    /// Used internally to scale colliders to match a sprite's current translation, rotation, and scale
    pub fn relative_to(&self, sprite: &Sprite) -> Vec<Vec2> {
        self.rotated(sprite.rotation)
            .iter()
            .map(|&v| v * sprite.scale + sprite.translation) // scale & translation
            .collect()
    }
    /// Returns a `Vec<Vec2>` containing the points of the collider, or an empty `Vec` if there is
    /// no collider.
    pub fn points(&self) -> Vec<Vec2> {
        if let Self::Poly(points) = self {
            points.clone()
        } else {
            Vec::with_capacity(0)
        }
    }
    /// Whether or not two sprites are currently colliding. This method ignores the `collision`
    /// field of the sprites.
    pub fn colliding(sprite1: &Sprite, sprite2: &Sprite) -> bool {
        use Collider::*;
        if let NoCollider = sprite1.collider {
            return false;
        }
        if let NoCollider = sprite2.collider {
            return false;
        }
        if sprite1.collider.is_poly() && sprite2.collider.is_poly() {
            let poly1 = sprite1.collider.relative_to(sprite1);
            let poly2 = sprite2.collider.relative_to(sprite2);
            // Polygon intersection algorithm adapted from
            // https://stackoverflow.com/questions/10962379/how-to-check-intersection-between-2-rotated-rectangles
            for poly in [poly1.clone(), poly2.clone()] {
                for (idx, &p1) in poly.iter().enumerate() {
                    let p2 = poly[(idx + 1) % poly.len()];
                    let normal = Vec2::new(p2.y - p1.y, p1.x - p2.x);

                    let mut min_a = None;
                    let mut max_a = None;
                    for &p in poly1.iter() {
                        let projected = normal.x * p.x + normal.y * p.y;
                        if min_a.is_none() || projected < min_a.unwrap() {
                            min_a = Some(projected);
                        }
                        if max_a.is_none() || projected > max_a.unwrap() {
                            max_a = Some(projected);
                        }
                    }

                    let mut min_b = None;
                    let mut max_b = None;
                    for &p in poly2.iter() {
                        let projected = normal.x * p.x + normal.y * p.y;
                        if min_b.is_none() || projected < min_b.unwrap() {
                            min_b = Some(projected);
                        }
                        if max_b.is_none() || projected > max_b.unwrap() {
                            max_b = Some(projected);
                        }
                    }

                    if max_a < min_b || max_b < min_a {
                        return false;
                    }
                }
            }
            return true;
        }
        false
    }
}
