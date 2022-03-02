use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3<f64>,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Vec3<f64>, dir: Vec3<f64>) -> Ray {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Vec3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f64> {
        self.dir
    }

    pub fn point_at(&self, p: f64) -> Vec3<f64> {
        self.origin + self.dir * p
    }
}
