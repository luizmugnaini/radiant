use crate::ray::Ray;
use crate::vec3::Vec3;

// Note: this HitRecord takes the approach of calculating whether the ray hits
// from the front or back of the surface on the coloring.
//
// The normal should point always oposite to the incoming ray
//
// If the dot product between the incoming ray and the outward normal is
// negative, then we have that the normal should be pointing outward, otherwise
// inward (-outward normal)
pub struct HitRecord {
    pub point: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub parameter: f32,
    front_face: bool,
}

impl HitRecord {
    // Returns default HitRecord to the user
    pub fn new() -> Self {
        Self {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            parameter: 0.0,
            front_face: true,
        }
    }

    fn set_face_normal(&mut self, ray: &Ray<f32>, outward_normal: Vec3<f32>) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Surface {
    fn hit(
        &self,
        ray: &Ray<f32>,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Surface for Sphere {
    fn hit(
        &self,
        ray: &Ray<f32>,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        let oc: Vec3<f32> = ray.origin - self.center;
        let a = ray.dir.dot(&ray.dir);
        let half_b = oc.dot(&ray.dir);
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
                    let outward_normal: Vec3<f32> =
                        (rec.point - self.center) / self.radius;
                    rec.set_face_normal(&ray, outward_normal);
                    true
                }
                None => false,
            }
        }
    }
}
