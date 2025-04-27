extern crate nalgebra as na;

pub type Point = na::Point3<f64>;
pub type Vector = na::Vector3<f64>;

pub trait R3 {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn near_zero(&self) -> bool;
    fn reflect(&self, normal: &Vector) -> Vector;
    fn refract(&self, normal: &Vector, refractive_index: f64) -> Vector;
}

impl R3 for Vector {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> f64 {
        self[2]
    }

    fn near_zero(&self) -> bool {
        self.abs().iter().all(|x| *x < 1e-8)
    }

    fn reflect(&self, normal: &Vector) -> Vector {
        self - 2. * (self.dot(normal)) * normal
    }

    fn refract(&self, normal: &Vector, refractive_index: f64) -> Vector {
        let cosine_theta: f64 = (-self.dot(normal)).min(1.);
        let out_prep: Vector = refractive_index * (self + cosine_theta * normal);
        let out_parallel: Vector = -((1. - out_prep.norm_squared()).abs().sqrt()) * normal;

        out_prep + out_parallel
    }
}
