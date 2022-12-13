use crate::misc;
use std::{
    iter::{IntoIterator, Iterator},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    /// New vector
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Self { x, y, z }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    /// Vector dot product
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    /// Vector cross product
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Vector length
    pub fn len_squared(&self) -> T {
        self.dot(self)
    }
}

impl Vec3<f64> {
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Unitary vector
    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        self.x < f64::EPSILON && self.y < f64::EPSILON && self.z < f64::EPSILON
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - (*normal * self.dot(normal) * 2.0)
    }

    pub fn refract(&self, normal: &Self, index_refrac_ratio: f64) -> Self {
        let cos_theta = f64::min(-self.dot(normal), 1.0);
        let refrac_perpendicular = (*self + *normal * cos_theta) * index_refrac_ratio;
        let refrac_parallel =
            -*normal * f64::sqrt(f64::abs(1.0 - refrac_perpendicular.len_squared()));
        refrac_perpendicular + refrac_parallel
    }

    pub fn random() -> Self {
        Self::new(misc::rand(), misc::rand(), misc::rand())
    }

    pub fn random_on(min: f64, max: f64) -> Self {
        Self::new(
            misc::rand_on(min, max),
            misc::rand_on(min, max),
            misc::rand_on(min, max),
        )
    }

    // Hacky incorrect method for diffusion
    pub fn random_unit_sphere() -> Self {
        loop {
            let rand = Self::random_on(-1.0, 1.0);
            if rand.len_squared() > 1.0 {
                continue;
            } else {
                return rand;
            }
        }
    }

    // Lambertian diffusion method
    pub fn random_unit_vector() -> Self {
        Self::random_unit_sphere().unit_vector()
    }

    // Another approach for diffusion
    pub fn random_in_hemisphere(normal: Self) -> Self {
        let v = Self::random_unit_sphere();
        // If `v` is in the same side as the `normal`, return it
        if v.dot(&normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let point = Vec3::new(misc::rand_on(-1.0, 1.0), misc::rand_on(-1.0, 1.0), 0.0);
            if point.len_squared() >= 1.0 {
                continue;
            } else {
                return point;
            }
        }
    }
}

/// Iterator structure for `Vec3`
pub struct Vec3IntoIter<T> {
    vec3: Vec3<T>,
    index: usize,
}

impl<T: Copy> Iterator for Vec3IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let next = match self.index {
            0 => Some(self.vec3.x),
            1 => Some(self.vec3.y),
            2 => Some(self.vec3.z),
            _ => None,
        };
        self.index += 1;
        next
    }
}

impl<T: Copy> IntoIterator for Vec3<T> {
    type Item = T;
    type IntoIter = Vec3IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vec3IntoIter {
            vec3: self,
            index: 0,
        }
    }
}

/// Add vectors
impl<T> Add for Vec3<T>
where
    T: Copy + Add<T, Output = T>,
{
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
impl<T> AddAssign for Vec3<T>
where
    T: Copy + Add<T, Output = T> + AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Vector negation
impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Vec3<T> {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Multiply vector by scalar
/// Notice that `v` * `c` is allowed but `c` * `v` is not
impl<T> Mul<T> for Vec3<T>
where
    T: Copy + Mul<T, Output = T>,
{
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
impl<T> MulAssign<T> for Vec3<T>
where
    T: Copy + Mul<T, Output = T>,
{
    fn mul_assign(&mut self, c: T) {
        *self = Self {
            x: c * self.x,
            y: c * self.y,
            z: c * self.z,
        }
    }
}

/// Divide vector by scalar
impl<T> Div<T> for Vec3<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, d: T) -> Self {
        Self {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl<T> DivAssign<T> for Vec3<T>
where
    T: Copy + Div<T, Output = T>,
{
    fn div_assign(&mut self, d: T) {
        *self = Self {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl<T> Index<u8> for Vec3<T> {
    type Output = T;

    fn index(&self, index: u8) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl<T> IndexMut<u8> for Vec3<T> {
    fn index_mut(&mut self, index: u8) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}
