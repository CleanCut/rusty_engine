use std::collections::HashSet;

use bevy::prelude::*;

use crate::{actor::Actor, prelude::GameState};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision_detection.system());
    }
}

fn collision_detection(
    mut existing_collisions: Local<HashSet<CollisionPair>>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Actor, &Transform)>,
) {
    let mut current_collisions = HashSet::<CollisionPair>::new();
    'outer: for (actor1, transform1) in query.iter().filter(|(a, _)| a.collision) {
        for (actor2, transform2) in query.iter().filter(|(a, _)| a.collision) {
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
    // if !current_collisions.is_empty() {
    println!("{:?}", current_collisions);
    // }
}

#[derive(Debug, Default, Eq, Hash)]
struct CollisionPair(String, String);

impl PartialEq for CollisionPair {
    fn eq(&self, other: &Self) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Collider {
    NoCollider,
    Circle(f32),
    Rect {
        tl: Vec2,
        tr: Vec2,
        br: Vec2,
        bl: Vec2,
    },
}

impl Default for Collider {
    fn default() -> Self {
        Collider::NoCollider
    }
}

impl Collider {
    pub fn rect(topleft: Vec2, bottomright: Vec2) -> Self {
        Self::Rect {
            tl: topleft,
            tr: Vec2::new(bottomright.x, topleft.y),
            br: bottomright,
            bl: Vec2::new(topleft.x, bottomright.y),
        }
    }
    pub fn circle(radius: f32) -> Self {
        Self::Circle(radius)
    }
    pub fn is_rect(&self) -> bool {
        if let Self::Rect { .. } = self {
            true
        } else {
            false
        }
    }
    fn rotated(&self, rotation: f32) -> Vec<Vec2> {
        let mut rotated_points = Vec::new();
        if let &Self::Rect { tl, tr, br, bl } = self {
            let sin = rotation.sin();
            let cos = rotation.cos();
            for point in [tl, tr, br, bl] {
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
            .map(|&v| v + actor.translation)
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
        match (actor1.collider, actor2.collider) {
            (Circle(radius_a), Circle(radius_b)) => {
                return actor1.translation.distance(actor2.translation) < radius_a + radius_b;
            }
            (Circle(radius), rect) if rect.is_rect() => {
                let points = rect.relative_to(actor2);
                for coord in points {
                    if actor1.translation.distance(coord) < radius {
                        return true;
                    }
                }
                false
            }
            (rect, Circle(radius)) if rect.is_rect() => {
                let points = rect.relative_to(actor1);
                for coord in points {
                    if actor2.translation.distance(coord) < radius {
                        return true;
                    }
                }
                false
            }
            (rect1, rect2) if rect1.is_rect() && rect2.is_rect() => {
                let poly1 = rect1.relative_to(actor1);
                let poly2 = rect2.relative_to(actor2);
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
                true
            }
            _ => false,
        }
    }
}
