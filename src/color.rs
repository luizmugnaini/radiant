use std::fmt::{self, Display, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
/// Notice that `v` * `c` is allowed but `c` * `v` is not
impl Mul<u8> for Color {
    type Output = Self;

    fn mul(self, c: u8) -> Self {
        Self {
            r: c * self.r,
            g: c * self.g,
            b: c * self.b,
        }
    }
}

/// Multiply assign with *=
impl MulAssign<u8> for Color {
    fn mul_assign(&mut self, c: u8) {
        *self = Self {
            r: c * self.r,
            g: c * self.g,
            b: c * self.b,
        }
    }
}

impl Div<u8> for Color {
    type Output = Color;

    fn div(self, d: u8) -> Self {
        Self {
            r: self.r / d,
            g: self.g / d,
            b: self.b / d,
        }
    }
}

impl DivAssign<u8> for Color {
    fn div_assign(&mut self, d: u8) {
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
    type Output = u8;

    fn index(&self, index: RGB) -> &u8 {
        match index {
            RGB::Red => &self.r,
            RGB::Green => &self.g,
            RGB::Blue => &self.b,
        }
    }
}

impl IndexMut<RGB> for Color {
    fn index_mut(&mut self, index: RGB) -> &mut u8 {
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

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn write_color(&self) {
        println!("{} {} {}\n", self.r, self.g, self.b)
    }

    pub fn to_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: (255.999 * r) as u8,
            g: (255.999 * g) as u8,
            b: (255.999 * b) as u8,
        }
    }
}
