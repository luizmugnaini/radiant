use crate::{color::Color, misc, ray::Ray, surf::HitRecord, vec3::Vec3};
use rand;

pub trait Scatterable {
    // How the ray interacts with the material
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn default() -> Self {
        Self::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 0.0)))
    }

    pub fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian(l) => l.scatter(ray, rec),
            Self::Metal(m) => m.scatter(ray, rec),
            Self::Dielectric(d) => d.scatter(ray, rec),
        }
    }

    pub fn lambertian(albedo: Color) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    pub fn metal(albedo: Color, fuzz: f32) -> Self {
        Self::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(index_refraction: f32) -> Self {
        Self::Dielectric(Dielectric::new(index_refraction))
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let mut direction = rec.normal() + Vec3::random_unit_vector(&mut rand::thread_rng());
        // Degenerate scatter direction
        if direction.near_zero() {
            direction = rec.normal();
        }
        let scattered = Ray::new(rec.point(), direction);
        Some((scattered, self.albedo))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    fn new(albedo: Color, fuzz_: f32) -> Self {
        let fuzz = if fuzz_ < 1.0 { fuzz_ } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&rec.normal());
        let scattered = Ray::new(
            rec.point(),
            reflected + Vec3::random_unit_sphere(&mut rand::thread_rng()) * self.fuzz,
        );

        if scattered.direction().dot(&rec.normal()) > 0.0 {
            return Some((scattered, self.albedo));
        }
        None
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    index_refraction: f32,
}

impl Dielectric {
    fn new(index_refraction: f32) -> Self {
        Self { index_refraction }
    }

    fn reflectance(cos: f32, refraction_ratio: f32) -> f32 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face() {
            1.0 / self.index_refraction
        } else {
            self.index_refraction
        };

        // Decide whether the incoming `ray` reflects or refracts
        let unit_direction = ray.direction().unit_vector();
        let cos_theta = f32::min(-unit_direction.dot(&rec.normal()), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let direction = if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > misc::rand()
        {
            // Reflection occurs
            unit_direction.reflect(&rec.normal())
        } else {
            // Refraction occurs
            unit_direction.refract(&rec.normal(), refraction_ratio)
        };

        Some((Ray::new(rec.point(), direction), attenuation))
    }
}
