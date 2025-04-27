use ray_tracer::{
    INFINITY,
    camera::Camera,
    color::Color,
    hittable::{HittableList, Sphere},
    interval::Interval,
    material::{Dielectric, Lambertian, Metal},
    vector::{Point, Vector},
};
use std::sync::Arc;

fn main() {
    test_scene();
}

fn test_scene() {
    let image_width: i32 = 1200;
    let aspect_ratio: f64 = 16. / 9.;
    let vfov: f64 = 80.;
    let look_from: Point = Point::new(0.2, 2.6, 4.);
    let look_at: Point = Point::new(0.6, 1.3, -0.6);
    let vup: Vector = Vector::new(0., 1., 0.);
    let samples_per_pixel: i32 = 500;
    let max_depth: i32 = 50;
    let defocus_angle: f64 = 0.5;
    let focus_dist: f64 = (look_from - look_at).norm();

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_dist,
        samples_per_pixel,
        max_depth,
    );

    let mut world = HittableList::new(Vec::new());

    let material_ground: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.push(Arc::new(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        material_ground.clone(),
    )));

    let material_1: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));

    world.push(Arc::new(Sphere::new(
        Point::new(0.3, 1., 0.3),
        1.,
        material_1.clone(),
    )));

    let material_2: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));

    world.push(Arc::new(Sphere::new(
        Point::new(-1.4, 1.3, -1.),
        1.3,
        material_2.clone(),
    )));

    let material_3: Arc<Metal> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.push(Arc::new(Sphere::new(
        Point::new(2.4, 2., -1.2),
        2.,
        material_3.clone(),
    )));

    camera.render(
        world,
        Camera::test_colorizer,
        Interval::new(0.001, INFINITY),
    );
}
