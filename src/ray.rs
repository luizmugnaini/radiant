use crate::vec3::Vec3;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub dir: Vec3<T>,
}

impl<T> Ray<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    pub fn new(origin: Vec3<T>, dir: Vec3<T>) -> Ray<T> {
        Self { origin, dir }
    }

    pub fn origin(self) -> Vec3<T> {
        self.origin
    }

    pub fn direction(self) -> Vec3<T> {
        self.dir
    }

    pub fn point_at(&self, p: T) -> Vec3<T> {
        self.origin + self.dir * p
    }
}
