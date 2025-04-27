use ray_tracer::{
    INFINITY,
    camera::Camera,
    hittable::{HittableList, Sphere},
    interval::Interval,
    vector::Point,
};
use std::sync::Arc;

fn main() {
    let camera = Camera::new(400, 16.0 / 9.0, 2.0, Point::origin(), 1.0);

    let mut world = HittableList::new(Vec::new());
    world.push(Arc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(&world, Camera::test_colorizer, Interval::new(0.0, INFINITY));
}
