use bevy::prelude::Vec2;

pub enum Collider {
    Circle(Vec2, f32),
    AABB(Vec2, Vec2),
}

impl Collider {
    pub fn collide(first: &Collider, second: &Collider) -> bool {
        match (first, second) {
            (first @ Collider::Circle(_, _), second @ Collider::Circle(_, _)) => {
                Collider::collide_circle_circle(first, second)
            }
            (first @ Collider::AABB(_, _), second @ Collider::AABB(_, _)) => {
                Collider::collide_aabb_aabb(first, second)
            }
            (aabb, circle @ Collider::Circle(_, _)) => Collider::collide_circle_aabb(circle, aabb),
            (circle, aabb) => Collider::collide_circle_aabb(circle, aabb),
            _ => todo!(),
        }
    }

    fn collide_circle_circle(first: &Collider, second: &Collider) -> bool {
        match (first, second) {
            (Collider::Circle(f_center, f_radius), Collider::Circle(s_center, s_radius)) => {
                let distance = f_center.distance(*s_center);
                let collide_distance = f_radius.abs() + s_radius.abs();
                distance <= collide_distance
            }
            _ => panic!("Passed invalid colliders to circle-circle-collision"),
        }
    }

    fn collide_aabb_aabb(first: &Collider, second: &Collider) -> bool {
        match (first, second) {
            (Collider::AABB(f_center, f_size), Collider::AABB(s_center, s_size)) => {
                let [f_min_x, f_max_x, f_min_y, f_max_y] = aabb_bounds(f_center, f_size);
                let [s_min_x, s_max_x, s_min_y, s_max_y] = aabb_bounds(s_center, s_size);
                f_min_x <= s_max_x && s_min_x <= f_max_x && f_min_y <= s_max_y && s_min_y <= f_max_y
            }
            _ => panic!("Passed invalid colliders to aabb-aabb-collision"),
        }
    }

    fn collide_circle_aabb(circle: &Collider, aabb: &Collider) -> bool {
        match (circle, aabb) {
            (Collider::Circle(circle_center, _), Collider::AABB(aabb_center, aabb_size)) => {
                let [aabb_min_x, aabb_max_x, aabb_min_y, aabb_max_y] =
                    aabb_bounds(aabb_center, aabb_size);
                let circle_in_aabb = aabb_min_x <= circle_center.x
                    && circle_center.x <= aabb_max_x
                    && aabb_min_y <= circle_center.y
                    && circle_center.y <= aabb_max_y;
                circle_in_aabb
                    || aabb_line_segments(aabb_center, aabb_size)
                        .iter()
                        .any(|line_segment| {
                            Collider::intersect_circle_line_segment(circle, line_segment)
                        })
            }
            _ => panic!("Passed invalid colliders to circle-aabb-collision"),
        }
    }

    fn intersect_circle_line_segment(circle: &Collider, line_segment: &(Vec2, Vec2)) -> bool {
        match circle {
            Collider::Circle(center, radius) => {
                if line_segment.0.x == line_segment.1.x {
                    // vertical
                    if between(center.y, line_segment.0.y, line_segment.1.y) {
                        (center.x - line_segment.0.x).abs() <= *radius
                    } else {
                        center
                            .distance(line_segment.0)
                            .min(center.distance(line_segment.1))
                            <= *radius
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
