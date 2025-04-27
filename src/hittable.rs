use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{Point, Vector},
};
use std::sync::Arc;

pub struct HitRecord {
    point: Point,
    normal: Vector,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point,
        normal: Vector,
        material: Arc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn normal(&self) -> Vector {
        self.normal
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<HitRecord>;
}

pub struct HittableList(Vec<Arc<dyn Hittable>>);

impl HittableList {
    pub fn new(vec: Vec<Arc<dyn Hittable>>) -> Self {
        HittableList(vec)
    }
}

impl std::ops::Deref for HittableList {
    type Target = Vec<Arc<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<HitRecord> {
        let mut closest = time.max();
        let mut result = None;

        for obj in self.iter() {
            if let Some(rec) = obj.hit(ray, Interval::new(time.min(), closest)) {
                closest = rec.t();
                result = Some(rec);
            }
        }
        result
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<HitRecord> {
        let oc: Vector = self.center - ray.origin();
        let a: f64 = Vector::dot(&ray.direction(), &ray.direction());
        let h: f64 = ray.direction().dot(&oc);
        let c: f64 = Vector::dot(&oc, &oc) - self.radius.powi(2);
        let disciminant: f64 = h.powi(2) - a * c;

        if disciminant < 0. {
            return None;
        }

        let mut t: f64 = (h - disciminant.sqrt()) / a;
        if !time.surrounds(t) {
            t = (h + disciminant.sqrt()) / a;
            if !time.surrounds(t) {
                return None;
            }
        }

        let point: Point = ray.at(t);
        let outward_normal: Vector = (point - self.center) / self.radius;
        let front_face: bool = ray.direction().dot(&outward_normal) < 0.;

        let normal: Vector = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord::new(
            point,
            normal,
            self.material.clone(),
            t,
            front_face,
        ))
    }
}
