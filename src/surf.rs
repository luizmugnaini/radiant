pub use crate::{material::Material, ray::Ray, vec3::Vec3};

// Note: this HitRecord takes the approach of calculating whether the ray hits
// from the front or back of the surface on the coloring.
pub struct HitRecord {
    point: Vec3<f32>,
    normal: Vec3<f32>,
    material: Material,
    parameter: f32,
    front_face: bool,
}

impl HitRecord {
    // Returns default HitRecord to the user
    pub fn new() -> Self {
        Self {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Material::default(),
            parameter: 0.0,
            front_face: true,
        }
    }

    // The normal should point always oposite to the incoming ray
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3<f32>) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn point(&self) -> Vec3<f32> {
        self.point
    }

    pub fn normal(&self) -> Vec3<f32> {
        self.normal
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn parameter(&self) -> f32 {
        self.parameter
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vec3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3<f32> = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let half_b = oc.dot(&ray.direction());
        let discriminant = {
            let c = oc.dot(&oc) - self.radius * self.radius;
            half_b * half_b - a * c
        };

        if discriminant < 0.0 {
            false
        } else {
            let sqrtd = discriminant.sqrt();

            // nearest root within the parameters
            let root: Option<f32> = {
                let r1 = (-half_b - sqrtd) / a;
                if r1 < t_min || t_max < r1 {
                    // try the next possible root
                    let r2 = (-half_b + sqrtd) / a;
                    if r2 < t_min || t_max < r2 {
                        None
                    } else {
                        Some(r2)
                    }
                } else {
                    Some(r1)
                }
            };

            match root {
                Some(r) => {
                    rec.parameter = r;
                    rec.point = ray.point_at(r);
                    let outward_normal: Vec3<f32> = (rec.point - self.center) / self.radius;
                    rec.set_face_normal(ray, outward_normal);
                    rec.material = self.material;
                    true
                }
                None => false,
            }
        }
    }
}
