use rand::Rng;
use std::sync::Arc;

use crate::{
    color::{Color, Color3},
    hittable::{Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vector::{Point, R3, Vector},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,

    camera_center: Point,

    pixel_delta_u: Vector,
    pixel_delta_v: Vector,

    pixel_00_pos: Point,

    samples_per_pixel: i32,
    max_depth: i32,
}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        viewport_height: f64,
        camera_center: Point,
        focal_length: f64,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        let image_height: i32 = {
            let image_height: i32 = (f64::from(image_width) / aspect_ratio).trunc() as i32;
            if image_height < 1 { 1 } else { image_height }
        };

        let viewport_width: f64 =
            viewport_height * (f64::from(image_width) / f64::from(image_height));

        let viewport_u: Vector = Vector::new(viewport_width, 0., 0.);
        let viewport_v: Vector = Vector::new(0., -viewport_height, 0.);

        let pixel_delta_u: Vector = viewport_u / f64::from(image_width);
        let pixel_delta_v: Vector = viewport_v / f64::from(image_height);

        let viewport_upper_left: Point =
            camera_center + Vector::new(0., 0., -focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel_00_pos: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00_pos,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn get_ray(&self, i: i32, j: i32, offset: (f64, f64)) -> Ray {
        let pixel_center: Point = self.pixel_00_pos
            + (f64::from(i) + offset.0) * self.pixel_delta_u
            + (f64::from(j) + offset.1) * self.pixel_delta_v;
        let ray_direction: Vector = pixel_center - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }

    pub fn render(
        &self,
        world: HittableList,
        colorizer: fn(&Ray, Arc<HittableList>, Interval, i32) -> Color,
        time: Interval,
    ) {
        use crate::distribution::UniformOffset2D;
        use indicatif::{ParallelProgressIterator, ProgressStyle};
        use itertools::Itertools;
        use log::info;
        use rayon::prelude::*;
        use std::time::{Duration, Instant};

        env_logger::init();

        let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})",
        ).unwrap().progress_chars("#>-");

        info!("Rendering started.");

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let start = Instant::now();

        let pixel_samples_scale: f64 = 1. / f64::from(self.samples_per_pixel);

        let pixels: Vec<(i32, i32)> = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect();

        let mut buf: Vec<Color> =
            Vec::with_capacity((self.image_width * self.image_height) as usize);

        let r_dist = UniformOffset2D::new(-0.5, 0.5);

        let world = Arc::new(world);

        pixels
            .into_par_iter()
            .progress_with_style(style)
            .map(|(j, i)| {
                (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map_init(rand::rng, |rng, _| rng.sample(&r_dist))
                    .map(|offset| {
                        let ray: Ray = self.get_ray(i, j, offset);
                        colorizer(&ray, world.clone(), time, self.max_depth)
                    })
                    .sum::<Color>()
                    * pixel_samples_scale
            })
            .collect_into_vec(&mut buf);

        buf.iter().for_each(|x| println!("{}", x.write()));

        let duration: Duration = start.elapsed();
        info!("Done. Time: {:?}.", duration);
    }

    pub fn test_colorizer(
        ray: &Ray,
        world: Arc<HittableList>,
        time: Interval,
        depth: i32,
    ) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        world
            .hit(ray, time)
            .map(|hit_record| {
                hit_record
                    .material()
                    .scatter(ray, &hit_record)
                    .map(|scattering| {
                        scattering.attenuation().component_mul(
                            &(Self::test_colorizer(
                                scattering.ray(),
                                world.clone(),
                                time,
                                depth - 1,
                            )),
                        )
                    })
                    .unwrap_or(Color::new(0., 0., 0.))
            })
            .unwrap_or(Self::sky_box(ray))
    }

    pub fn sky_box(ray: &Ray) -> Color {
        let a: f64 = 0.5 * (ray.direction().normalize().y() + 1.);
        (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
    }
}
