use crate::actor::Actor;
use bevy::prelude::*;
use std::collections::HashSet;

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

#[derive(Debug, Clone)]
pub enum CollisionState {
    Begin,
    End,
}

#[derive(Debug, Default, Eq, Hash, Clone)]
pub struct CollisionPair(String, String);

impl PartialEq for CollisionPair {
    fn eq(&self, other: &Self) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

fn collision_detection(
    mut existing_collisions: Local<HashSet<CollisionPair>>,
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<&Actor>,
) {
    let mut current_collisions = HashSet::<CollisionPair>::new();
    'outer: for actor1 in query.iter().filter(|a| a.collision) {
        for actor2 in query.iter().filter(|a| a.collision) {
            if actor1.label == actor2.label {
                // We only need to compare one half of the matrix triangle
                continue 'outer;
            }
            if Collider::colliding(&actor1, &actor2) {
                current_collisions
                    .insert(CollisionPair(actor1.label.clone(), actor2.label.clone()));
            }
        }
    }

    let beginning_collisions: Vec<_> = current_collisions
        .difference(&existing_collisions)
        .map(|x| x.clone())
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
        .map(|x| x.clone())
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
        Self::Poly(points.into_iter().map(|&x| x.into()).collect())
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
        match self {
            Self::Poly(_) => true,
            _ => false,
        }
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
    fn relative_to(&self, actor: &Actor) -> Vec<Vec2> {
        self.rotated(actor.rotation)
            .iter()
            .map(|&v| v * actor.scale + actor.translation) // scale & translation
            .collect()
    }
    pub fn colliding(actor1: &Actor, actor2: &Actor) -> bool {
        use Collider::*;
        if let NoCollider = actor1.collider {
            return false;
        }
        if let NoCollider = actor2.collider {
            return false;
        }
        if actor1.collider.is_poly() && actor2.collider.is_poly() {
            let poly1 = actor1.collider.relative_to(actor1);
            let poly2 = actor2.collider.relative_to(actor2);
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
