use crate::camera;
use rand::{rngs::ThreadRng, Rng};
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn rand(rng: &mut ThreadRng) -> Self {
        Self {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }

    pub fn rand_on(rng: &mut ThreadRng, min: f32, max: f32) -> Self {
        Self {
            r: rng.gen_range(min..max),
            g: rng.gen_range(min..max),
            b: rng.gen_range(min..max),
        }
    }

    pub fn rgb(self) -> (u8, u8, u8) {
        // Gamma correction for gamma = 2.0
        let scale = 1.0 / camera::SAMPLES_PER_PIXEL as f32;
        let r = (f32::sqrt(self.r * scale)).clamp(0.0, 0.999);
        let g = (f32::sqrt(self.g * scale)).clamp(0.0, 0.999);
        let b = (f32::sqrt(self.b * scale)).clamp(0.0, 0.999);

        // Translated colors to the interval [0, 255]
        ((256.0 * r) as u8, (256.0 * g) as u8, (256.0 * b) as u8)
    }
}

/// Add vectors
impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

/// Assign add with +=
impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

/// Multiply by scalar
/// Notice that `v` * `c`d you're right. It is (the best one, imo is allowed
/// but `c` * `v` is not
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, c: f32) -> Self {
        Self {
            r: c * self.r,
            g: c * self.g,
            b: c * self.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

/// Multiply assign with *=
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, c: f32) {
        *self = Self {
            r: c * self.r,
            g: c * self.g,
            b: c * self.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, d: f32) -> Self {
        Self {
            r: self.r / d,
            g: self.g / d,
            b: self.b / d,
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, d: f32) {
        *self = Self {
            r: self.r / d,
            g: self.g / d,
            b: self.b / d,
        }
    }
}

pub enum RGB {
    Red,
    Green,
    Blue,
}

impl Index<RGB> for Color {
    type Output = f32;

    fn index(&self, index: RGB) -> &f32 {
        match index {
            RGB::Red => &self.r,
            RGB::Green => &self.g,
            RGB::Blue => &self.b,
        }
    }
}

impl IndexMut<RGB> for Color {
    fn index_mut(&mut self, index: RGB) -> &mut f32 {
        match index {
            RGB::Red => &mut self.r,
            RGB::Green => &mut self.g,
            RGB::Blue => &mut self.b,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
