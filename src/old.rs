use crate::{
    color::Color,
    ray::Ray,
    vector::{Point, R3, Vector},
};

pub fn test1_par() {
    use itertools::Itertools;
    use log::info;
    use rayon::prelude::*;
    use std::time::{Duration, Instant};

    env_logger::init();

    fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> f64 {
        let oc: Vector = center - ray.origin();
        let a: f64 = Vector::dot(&ray.direction(), &ray.direction());
        let h: f64 = ray.direction().dot(&oc);
        let c: f64 = Vector::dot(&oc, &oc) - radius.powi(2);
        let disciminant: f64 = h.powi(2) - a * c;

        if disciminant < 0.0 {
            -1.0
        } else {
            (h - disciminant.sqrt()) / a
        }
    }

    fn ray_color(ray: Ray) -> Color {
        let center: Point = Point::new(0.0, 0.0, -1.0);
        let t = hit_sphere(center, 0.5, &ray);
        if t > 0.0 {
            let normal: Vector = (ray.at(t) - center).normalize();
            return Color(0.5 * (normal + Vector::new(1.0, 1.0, 1.0)));
        }

        let unit_direction: Vector = ray.direction().normalize();
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);

        Color((1.0 - a) * *Color::new(1.0, 1.0, 1.0) + a * *Color::new(0.5, 0.7, 1.0))
    }

    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: i32 = 400;
    let image_height: i32 = {
        let image_height: i32 = (f64::from(image_width) / aspect_ratio).trunc() as i32;
        if image_height < 1 { 1 } else { image_height }
    };

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (f64::from(image_width) / f64::from(image_height));

    let camera_center: Point = Point::new(0.0, 0.0, 0.0);
    let focal_length: f64 = 1.0;

    let viewport_u: Vector = Vector::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vector = Vector::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vector = viewport_u / f64::from(image_width);
    let pixel_delta_v: Vector = viewport_v / f64::from(image_height);

    let viewport_upper_left: Point =
        camera_center + Vector::new(0.0, 0.0, -focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_pos: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let start = Instant::now();

    let mut buf: Vec<Color> = Vec::with_capacity((image_width * image_height) as usize);

    (0..image_height)
        .cartesian_product(0..image_width)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(j, i)| {
            let pixel_center: Point =
                pixel_00_pos + f64::from(i) * pixel_delta_u + f64::from(j) * pixel_delta_v;
            let ray_direction: Vector = pixel_center - camera_center;

            let ray: Ray = Ray::new(camera_center, ray_direction);
            ray_color(ray)
        })
        .collect_into_vec(&mut buf);

    buf.iter().for_each(|x| println!("{}", x));

    let duration: Duration = start.elapsed();
    info!("Done. Time: {:?}.", duration);
}

pub fn test_picture() {
    use log::{info, logger};

    env_logger::init();

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        info!("Scanlines remaining: {}", { image_height - j });
        logger().flush();

        for i in 0..image_width {
            let r: f64 = f64::from(i) / f64::from(image_width - 1);
            let g: f64 = f64::from(j) / f64::from(image_height - 1);
            let b: f64 = 0.0;

            let color = Color::new(r, g, b);

            println!("{}", color);
        }
    }

    info!("Done.");
}
