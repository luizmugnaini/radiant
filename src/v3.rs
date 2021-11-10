use std::ops::{Add, Index, Mul};

#[derive(Debug)]
pub struct V3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Add vectors
impl<T: Add<T, Output = T>> Add for V3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Multiply by scalar
impl<T: Copy + Mul<T, Output = T>> Mul<T> for V3<T> {
    type Output = Self;

    fn mul(self, c: T) -> Self {
        Self {
            x: c * self.x,
            y: c * self.y,
            z: c * self.z,
        }
    }
}

/// Vector indexing
/// TODO: this doesn't reproduce a good behaviour, its best to have an option none
impl<T> Index<u8> for V3<T> {
    type Output = T;

    fn index(&self, index: u8) -> &T {
        if index % 3 == 0 {
            return &self.x;
        } else if index % 3 == 1 {
            return &self.y;
        }
        &self.z
    }
}
