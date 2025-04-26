use ray_tracer::{
    INFINITY,
    drawer::{RayTracer, Scene},
    hittable::{HittableList, Sphere},
    vector::Point,
};
use std::sync::Arc;

fn main() {
    let scene = Scene::new(400, 16.0 / 9.0, 2.0, Point::origin(), 1.0);
    let mut hittable_list = HittableList::new(Vec::new());

    hittable_list.push(Arc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    hittable_list.push(Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let ray_tracer = RayTracer::new(scene, hittable_list);
    ray_tracer.draw(RayTracer::test_colorizer, 0.0, INFINITY);
}
