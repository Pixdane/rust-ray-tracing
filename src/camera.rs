use rand::Rng;
use std::sync::Arc;

use crate::{
    color::{Color, Color3},
    distribution::UniformUnitVec3D,
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

    defocus_angle: f64,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,

    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        vfov: f64,
        look_from: Point,
        look_at: Point,
        vup: Vector,
        defocus_angle: f64,
        focus_dist: f64,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        let image_height: i32 = {
            let image_height: i32 = (f64::from(image_width) / aspect_ratio).trunc() as i32;
            if image_height < 1 { 1 } else { image_height }
        };

        let pixel_samples_scale: f64 = 1.0 / f64::from(samples_per_pixel);

        let camera_center: Point = look_from;

        let theta: f64 = vfov.to_radians();
        let h: f64 = (theta / 2.).tan();
        let viewport_height: f64 = 2. * h * focus_dist;
        let viewport_width: f64 =
            viewport_height * (f64::from(image_width) / f64::from(image_height));

        let w: Vector = (look_from - look_at).normalize();
        let u: Vector = vup.cross(&w);
        let v: Vector = w.cross(&u);

        let viewport_u: Vector = viewport_width * u;
        let viewport_v: Vector = viewport_height * -v;

        let pixel_delta_u: Vector = viewport_u / f64::from(image_width);
        let pixel_delta_v: Vector = viewport_v / f64::from(image_height);

        let viewport_upper_left: Point =
            camera_center - focus_dist * w - viewport_u / 2. - viewport_v / 2.;
        let pixel_00_pos: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius: f64 = (defocus_angle / 2.).to_radians().tan() * focus_dist;
        let defocus_disk_u: Vector = u * defocus_radius;
        let defocus_disk_v: Vector = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00_pos,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn get_ray(&self, i: i32, j: i32, offset: (f64, f64)) -> Ray {
        let pixel_center: Point = self.pixel_00_pos
            + (f64::from(i) + offset.0) * self.pixel_delta_u
            + (f64::from(j) + offset.1) * self.pixel_delta_v;

        let ray_origin: Point = if self.defocus_angle <= 0. {
            self.camera_center
        } else {
            let offset = UniformUnitVec3D::random_in_unit_disk();
            self.camera_center + offset.0 * self.defocus_disk_u + offset.1 * self.defocus_disk_v
        };

        let ray_direction: Vector = pixel_center - ray_origin;

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
                    * self.pixel_samples_scale
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
