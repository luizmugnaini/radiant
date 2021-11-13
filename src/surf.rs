use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    point: Vec3<f32>,
    normal: Vec3<f32>,
    front_face: bool,
    parameter: f32,
}

trait Surface {
    fn hit(&self, ray: Ray<f32>, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

impl Surface for Sphere {
    fn hit(&self, ray: Ray<f32>, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
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
                    rec.normal = (rec.point - self.center) / self.radius;
                    true
                }
                None => false,
            }
        }
    }
}
