use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug)]
pub struct V3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Add vectors
impl<T: Copy + Add<T, Output = T>> Add for V3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Assign add with +=
impl<T: Copy + Add<T, Output = T> + AddAssign> AddAssign for V3<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Multiply by scalar
/// Notice that `v` * `c` is allowed but `c` * `v` is not
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

/// Multiply assign with *=
impl<T: Copy + Mul<T, Output = T>> MulAssign<T> for V3<T> {
    fn mul_assign(&mut self, c: T) {
        *self = Self {
            x: c * self.x,
            y: c * self.y,
            z: c * self.z,
        }
    }
}

impl<T: Copy + Div<T, Output = T>> Div<T> for V3<T> {
    type Output = V3<T>;

    fn div(self, d: T) -> Self {
        Self {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl<T: Copy + Div<T, Output = T>> DivAssign<T> for V3<T> {
    fn div_assign(&mut self, d: T) {
        *self = Self {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl<T> Index<u8> for V3<T> {
    type Output = T;

    fn index(&self, index: u8) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for V3"),
        }
    }
}

impl<T> IndexMut<u8> for V3<T> {
    fn index_mut(&mut self, index: u8) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for V3"),
        }
    }
}
