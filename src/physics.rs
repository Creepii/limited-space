use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub enum CollisionBox {
    Circle {
        radius: f32,
    },
    AABB {
        width_radius: f32,
        height_radius: f32,
    },
}
impl CollisionBox {
    pub fn to_collider(&self, x: f32, y: f32) -> Collider {
        match self {
            CollisionBox::Circle { radius } => Collider::Circle(Vec2::new(x, y), *radius),
            CollisionBox::AABB {
                width_radius,
                height_radius,
            } => Collider::AABB(Vec2::new(x, y), Vec2::new(*width_radius, *height_radius)),
        }
    }
}

pub enum Collider {
    Circle(Vec2, f32),
    AABB(Vec2, Vec2),
}

impl Collider {
    pub fn does_collide(&self, other: &Collider) -> bool {
        self.collide(other).is_some()
    }

    pub fn collide(&self, other: &Collider) -> Option<Vec2> {
        match (self, other) {
            (Collider::Circle(_, _), Collider::Circle(_, _)) => self.collide_circle_circle(other),
            (Collider::AABB(_, _), Collider::AABB(_, _)) => self.collide_aabb_aabb(other),
            (_, Collider::Circle(_, _)) => todo!(), //self.collide_circle_aabb(other),
            (_, _) => self.collide_circle_aabb(other),
        }
    }

    fn collide_circle_circle(&self, other: &Collider) -> Option<Vec2> {
        match (self, other) {
            (
                Collider::Circle(self_center, self_radius),
                Collider::Circle(other_center, other_radius),
            ) => {
                let distance = self_center.distance(*other_center);
                let collide_distance = self_radius.abs() + other_radius.abs();
                if distance <= collide_distance {
                    Some(
                        (*other_center - *self_center)
                            * ((self_radius.abs() + other_radius.abs() - distance) / 2.0),
                    )
                } else {
                    None
                }
            }
            _ => panic!("Passed invalid colliders to circle-circle-collision"),
        }
    }

    fn collide_aabb_aabb(&self, other: &Collider) -> Option<Vec2> {
        match (self, other) {
            (Collider::AABB(self_center, self_size), Collider::AABB(other_center, other_size)) => {
                let [f_min_x, f_max_x, f_min_y, f_max_y] = aabb_bounds(self_center, self_size);
                let [s_min_x, s_max_x, s_min_y, s_max_y] = aabb_bounds(other_center, other_size);
                if f_min_x <= s_max_x
                    && s_min_x <= f_max_x
                    && f_min_y <= s_max_y
                    && s_min_y <= f_max_y
                {
                    todo!()
                } else {
                    None
                }
            }
            _ => panic!("Passed invalid colliders to aabb-aabb-collision"),
        }
    }

    fn collide_circle_aabb(&self, other: &Collider) -> Option<Vec2> {
        match (self, other) {
            (
                Collider::Circle(self_center, self_radius),
                Collider::AABB(other_center, other_size),
            ) => {
                let [aabb_min_x, aabb_max_x, aabb_min_y, aabb_max_y] =
                    aabb_bounds(other_center, other_size);
                let circle_in_aabb = aabb_min_x <= self_center.x
                    && self_center.x <= aabb_max_x
                    && aabb_min_y <= self_center.y
                    && self_center.y <= aabb_max_y;
                if circle_in_aabb {
                    let penetration_x = Vec2::new(other_center.x - self_center.x, 0.0);
                    let penetration_x = penetration_x.normalize()
                        * (other_size.x / 2.0 - penetration_x.length() + self_radius);
                    let penetration_y = Vec2::new(0.0, other_center.y - self_center.y);
                    let penetration_y = penetration_y.normalize()
                        * (other_size.y / 2.0 - penetration_y.length() + self_radius);
                    return Some(
                        if penetration_x.length_squared() < penetration_y.length_squared() {
                            penetration_x
                        } else {
                            penetration_y
                        },
                    );
                }
                aabb_line_segments(other_center, other_size)
                    .iter()
                    .map(|line_segment| self.intersect_circle_line_segment(line_segment))
                    .filter(|intersection| intersection.is_some())
                    .map(|intersection| intersection.unwrap())
                    .max_by(|first, second| first.length().partial_cmp(&second.length()).unwrap())
            }
            _ => panic!("Passed invalid colliders to circle-aabb-collision"),
        }
    }

    fn intersect_circle_line_segment(&self, line_segment: &(Vec2, Vec2)) -> Option<Vec2> {
        match self {
            Collider::Circle(center, radius) => {
                if line_segment.0.x == line_segment.1.x {
                    // vertical
                    let nearest_point = if between(center.y, line_segment.0.y, line_segment.1.y) {
                        Vec2::new(line_segment.0.x, center.y)
                    } else if center.distance(line_segment.0) < center.distance(line_segment.1) {
                        line_segment.0
                    } else {
                        line_segment.1
                    };
                    if center.distance(nearest_point) <= *radius {
                        let penetration = nearest_point - *center;
                        Some(penetration.normalize() * (radius - penetration.length()))
                    } else {
                        None
                    }
                } else if line_segment.0.y == line_segment.1.y {
                    // horizontal is just vertical with x and y flipped
                    Collider::intersect_circle_line_segment(
                        &Collider::Circle(Vec2::new(center.y, center.x), *radius),
                        &(
                            Vec2::new(line_segment.0.y, line_segment.0.x),
                            Vec2::new(line_segment.1.y, line_segment.1.x),
                        ),
                    )
                    .map(|point| Vec2::new(point.y, point.x))
                } else {
                    panic!("Only axis-aligned line segment intersections are currently supported")
                }
            }
            _ => panic!("Passed invalid collider to circle-line segment-intersection"),
        }
    }
}

fn aabb_bounds(center: &Vec2, size: &Vec2) -> [f32; 4] {
    let min_x = center.x - size.x.abs() / 2.0;
    let min_y = center.y - size.y.abs() / 2.0;
    let max_x = center.x + size.x.abs() / 2.0;
    let max_y = center.y + size.y.abs() / 2.0;
    [min_x, max_x, min_y, max_y]
}

fn aabb_line_segments(center: &Vec2, size: &Vec2) -> [(Vec2, Vec2); 4] {
    let [min_x, max_x, min_y, max_y] = aabb_bounds(center, size);
    [
        (Vec2::new(min_x, min_y), Vec2::new(min_x, max_y)),
        (Vec2::new(min_x, min_y), Vec2::new(max_x, min_y)),
        (Vec2::new(max_x, max_y), Vec2::new(min_x, max_y)),
        (Vec2::new(max_x, max_y), Vec2::new(max_x, min_y)),
    ]
}

fn between(to_check: f32, bound0: f32, bound1: f32) -> bool {
    bound0.min(bound1) <= to_check && to_check <= bound0.max(bound1)
}
