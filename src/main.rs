use ray_tracer::{
    INFINITY,
    camera::Camera,
    color::Color,
    hittable::{HittableList, Sphere},
    interval::Interval,
    material::{Dielectric, Lambertian, Metal},
    vector::Point,
};
use std::sync::Arc;

fn main() {
    let image_width: i32 = 400;
    let aspect_ratio: f64 = 16. / 9.;
    let viewport_height: f64 = 2.;
    let camera_center: Point = Point::origin();
    let focal_length: f64 = 1.;
    let samples_per_pixel: i32 = 500;
    let max_depth: i32 = 50;

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        viewport_height,
        camera_center,
        focal_length,
        samples_per_pixel,
        max_depth,
    );

    let mut world = HittableList::new(Vec::new());

    let material_center: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_ground: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_left: Arc<Dielectric> = Arc::new(Dielectric::new(1.50));
    let material_bubble: Arc<Dielectric> = Arc::new(Dielectric::new(1. / 1.50));
    let material_right: Arc<Metal> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    world.push(Arc::new(Sphere::new(
        Point::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    )));

    world.push(Arc::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    )));

    world.push(Arc::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));

    world.push(Arc::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.4,
        material_bubble.clone(),
    )));

    world.push(Arc::new(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    )));

    camera.render(
        world,
        Camera::test_colorizer,
        Interval::new(0.001, INFINITY),
    );
}
