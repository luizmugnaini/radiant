use crate::color::Color;
use crate::ray::Ray;
use crate::surf::HitRecord;
use crate::vec3::Vec3;

pub trait Scatterable {
    // How the ray interacts with the material
    fn scatter(&self, ray: Ray, hit_record: HitRecord)
        -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn default() -> Self {
        Self::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 0.0)))
    }

    pub fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(m) => m.scatter(ray, rec),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let mut direction = rec.normal() + Vec3::random_unit_vector();
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
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&rec.normal());
        let scattered = Ray::new(rec.point(), reflected);

        if scattered.direction().dot(&rec.normal()) > 0.0 {
            return Some((scattered, self.albedo));
        }
        None
    }
}
