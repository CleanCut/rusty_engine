use crate::sprite::Sprite;
use bevy::prelude::*;
use std::{collections::HashSet, hash::Hash};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CollisionEvent>()
            .add_system(collision_detection.system());
    }
}

#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub state: CollisionState,
    pub pair: CollisionPair,
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionState {
    Begin,
    End,
}

impl CollisionState {
    pub fn is_begin(&self) -> bool {
        match self {
            CollisionState::Begin => true,
            CollisionState::End => false,
        }
    }
    pub fn is_end(&self) -> bool {
        match self {
            CollisionState::Begin => false,
            CollisionState::End => true,
        }
    }
}

#[derive(Debug, Default, Eq, Clone)]
pub struct CollisionPair(pub String, pub String);

impl CollisionPair {
    pub fn either_contains<T: Into<String>>(&self, label: T) -> bool {
        let label = label.into();
        (self.0 == label) || (self.1 == label)
    }
    pub fn either_starts_with<T: Into<String>>(&self, label: T) -> bool {
        let label = label.into();
        self.0.starts_with(&label) || self.1.starts_with(&label)
    }
    pub fn one_starts_with<T: Into<String>>(&self, label: T) -> bool {
        let label = label.into();
        let a_matches = self.0.starts_with(&label);
        let b_matches = self.1.starts_with(&label);
        (a_matches && !b_matches) || (!a_matches && b_matches)
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

#[derive(Clone, Debug)]
pub enum Collider {
    NoCollider,
    Poly(Vec<Vec2>),
}

impl Default for Collider {
    fn default() -> Self {
        Collider::NoCollider
    }
}

impl Collider {
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
    pub fn poly<T: Into<Vec2> + Copy>(points: &[T]) -> Self {
        Self::Poly(points.iter().map(|&x| x.into()).collect())
    }
    pub fn circle_custom(radius: f32, vertices: usize) -> Self {
        let mut points = vec![];
        for x in 0..=vertices {
            let inner = 2.0 * std::f64::consts::PI / vertices as f64 * x as f64;
            points.push(Vec2::new(
                inner.cos() as f32 * radius,
                inner.sin() as f32 * radius,
            ));
        }
        Self::Poly(points)
    }
    pub fn circle(radius: f32) -> Self {
        Self::circle_custom(radius, 16)
    }
    pub fn is_poly(&self) -> bool {
        matches!(self, Self::Poly(_))
    }
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
    fn relative_to(&self, sprite: &Sprite) -> Vec<Vec2> {
        self.rotated(sprite.rotation)
            .iter()
            .map(|&v| v * sprite.scale + sprite.translation) // scale & translation
            .collect()
    }
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
            for poly in vec![poly1.clone(), poly2.clone()] {
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
