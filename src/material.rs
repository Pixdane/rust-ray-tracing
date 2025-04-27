use rand_distr::Distribution;

use crate::{
    color::Color,
    distribution::UniformUnitVec3D,
    hittable::HitRecord,
    ray::Ray,
    vector::{R3, Vector},
};

pub struct Scattering {
    ray: Ray,
    attenuation: Color,
}

impl Scattering {
    pub fn new(ray: Ray, attenuation: Color) -> Self {
        Scattering { ray, attenuation }
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn attenuation(&self) -> Color {
        self.attenuation
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scattering>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<Scattering> {
        let mut out_direction: Vector =
            UniformUnitVec3D::random_unit_vector() + hit_record.normal();
        if out_direction.near_zero() {
            out_direction = hit_record.normal();
        }
        let reflection: Ray = Ray::new(hit_record.point(), out_direction);

        Some(Scattering::new(reflection, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scattering> {
        let out_direction: Vector = ray_in.direction().reflect(&hit_record.normal()).normalize()
            + UniformUnitVec3D::random_unit_vector() * self.fuzz;
        let reflection: Ray = Ray::new(hit_record.point(), out_direction);
        (out_direction.dot(&hit_record.normal()) > 0.)
            .then_some(Scattering::new(reflection, self.albedo))
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scattering> {
        fn reflectance(cosine_theta: f64, ri: f64) -> f64 {
            let r0: f64 = ((1. - ri) / (1. + ri)).powi(2);
            r0 + (1. - r0) * (1. - cosine_theta).powi(5)
        }

        let ri: f64 = if hit_record.front_face() {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_in: Vector = ray_in.direction().normalize();

        let cosine_theta: f64 = (-unit_in.dot(&hit_record.normal())).min(1.);

        let sine_theta: f64 = (1. - cosine_theta.powi(2)).sqrt();

        let uniform = rand_distr::Uniform::new(0., 1.).unwrap();

        // Schlick's approximation
        let out_direction: Vector = if ri * sine_theta > 1.
            || reflectance(cosine_theta, ri) > uniform.sample(&mut rand::rng())
        {
            unit_in.reflect(&hit_record.normal())
        } else {
            unit_in.refract(&hit_record.normal(), ri)
        };

        let refraction: Ray = Ray::new(hit_record.point(), out_direction);
        let attenuation: Color = Color::new(1., 1., 1.);

        Some(Scattering::new(refraction, attenuation))
    }
}
