use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vector::{Point, R3, Vector},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    aspect_ratio: f64,

    viewport_height: f64,
    viewport_width: f64,

    camera_center: Point,
    focal_length: f64,

    viewport_u: Vector,
    viewport_v: Vector,

    pixel_delta_u: Vector,
    pixel_delta_v: Vector,

    viewport_upper_left: Point,
    pixel_00_pos: Point,
}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        viewport_height: f64,
        camera_center: Point,
        focal_length: f64,
    ) -> Self {
        let image_height: i32 = {
            let image_height: i32 = (f64::from(image_width) / aspect_ratio).trunc() as i32;
            if image_height < 1 { 1 } else { image_height }
        };

        let viewport_width: f64 =
            viewport_height * (f64::from(image_width) / f64::from(image_height));

        let viewport_u: Vector = Vector::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector = Vector::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u: Vector = viewport_u / f64::from(image_width);
        let pixel_delta_v: Vector = viewport_v / f64::from(image_height);

        let viewport_upper_left: Point = camera_center + Vector::new(0.0, 0.0, -focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel_00_pos: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            aspect_ratio,
            viewport_height,
            viewport_width,
            camera_center,
            focal_length,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel_00_pos,
        }
    }

    pub fn image_width(&self) -> i32 {
        self.image_width
    }

    pub fn image_height(&self) -> i32 {
        self.image_height
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    pub fn viewport_width(&self) -> f64 {
        self.viewport_width
    }

    pub fn camera_center(&self) -> Point {
        self.camera_center
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn viewport_u(&self) -> Vector {
        self.viewport_u
    }

    pub fn viewport_v(&self) -> Vector {
        self.viewport_v
    }

    pub fn pixel_delta_u(&self) -> Vector {
        self.pixel_delta_u
    }

    pub fn pixel_delta_v(&self) -> Vector {
        self.pixel_delta_v
    }

    pub fn viewport_upper_left(&self) -> Point {
        self.viewport_upper_left
    }

    pub fn pixel_00_pos(&self) -> Point {
        self.pixel_00_pos
    }

    pub fn render(
        &self,
        world: &dyn Hittable,
        colorizer: fn(&Ray, &dyn Hittable, Interval) -> Color,
        time: Interval,
    ) {
        use itertools::Itertools;
        use log::info;
        use rayon::prelude::*;
        use std::time::{Duration, Instant};

        env_logger::init();

        let image_width: i32 = self.image_width;
        let image_height: i32 = self.image_height;

        let camera_center: Point = self.camera_center;

        let pixel_00_pos: Point = self.pixel_00_pos;

        let pixel_delta_u: Vector = self.pixel_delta_u;
        let pixel_delta_v: Vector = self.pixel_delta_v;

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

                colorizer(&ray, world, time)
            })
            .collect_into_vec(&mut buf);

        buf.iter().for_each(|x| println!("{}", x));

        let duration: Duration = start.elapsed();
        info!("Done. Time: {:?}.", duration);
    }

    pub fn test_colorizer(ray: &Ray, hittable: &dyn Hittable, time: Interval) -> Color {
        if let Some(hit_record) = hittable.hit(ray, time) {
            Color(0.5 * (hit_record.normal() + Vector::from_element(1.0)))
        } else {
            let a: f64 = 0.5 * (ray.direction().normalize().y() + 1.0);
            Color((1.0 - a) * *Color::new(1.0, 1.0, 1.0) + a * *Color::new(0.5, 0.7, 1.0))
        }
    }
}
