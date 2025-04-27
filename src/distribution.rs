use rand::Rng;
use rand_distr::{Distribution, Uniform};

use crate::vector::Vector;

pub struct UniformOffset2D {
    min: f64,
    max: f64,
}

impl UniformOffset2D {
    pub fn new(min: f64, max: f64) -> Self {
        UniformOffset2D { min, max }
    }
}

impl Distribution<(f64, f64)> for UniformOffset2D {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64, f64) {
        let uniform = Uniform::new(self.min, self.max).unwrap();
        let rand_x = uniform.sample(rng);
        let rand_y = uniform.sample(rng);

        (rand_x, rand_y)
    }
}

pub struct UniformVec3D {
    min: f64,
    max: f64,
}

impl UniformVec3D {
    pub fn new(min: f64, max: f64) -> Self {
        UniformVec3D { min, max }
    }
}

impl Distribution<Vector> for UniformVec3D {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector {
        let uniform = Uniform::new(self.min, self.max).unwrap();
        let rand_x = uniform.sample(rng);
        let rand_y = uniform.sample(rng);
        let rand_z = uniform.sample(rng);

        Vector::new(rand_x, rand_y, rand_z)
    }
}

pub struct UniformUnitVec3D;

impl UniformUnitVec3D {
    pub fn random_unit_vector() -> Vector {
        let uniform = UniformUnitVec3D;
        uniform.sample(&mut rand::rng())
    }

    pub fn random_on_hemisphere(normal: &Vector) -> Vector {
        let uniform = UniformUnitVec3D;
        let unit_vec: Vector = uniform.sample(&mut rand::rng());
        if normal.dot(&unit_vec) > 0. {
            unit_vec
        } else {
            -unit_vec
        }
    }
}

impl Distribution<Vector> for UniformUnitVec3D {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector {
        let uniform = UniformVec3D::new(-1., 1.);
        loop {
            let vec: Vector = uniform.sample(rng);
            let norm_squared = vec.norm_squared();
            if 1e-160 < norm_squared && norm_squared <= 1. {
                return vec.normalize();
            }
        }
    }
}
