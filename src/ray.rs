use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3<f32>,
    dir: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Ray {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Vec3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f32> {
        self.dir
    }

    pub fn point_at(&self, p: f32) -> Vec3<f32> {
        self.origin + self.dir * p
    }
}
