use rand::Rng;
use std::f32::consts;

pub const INFTY: f32 = f32::MAX;
pub const PI: f32 = consts::PI;

// Degrees to radians
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

/// Hash function for fast generation of pseudo-random numbers.
fn pcg_hash(input: u32) -> u32 {
    let state = input * 747796405 + 2891336453;
    let word = ((state >> ((state >> 28) + 4)) ^ state) * 277803737;
    return (word >> 22) ^ word;
}

pub fn rand_float(mut seed: u32) -> f32 {
    seed = pcg_hash(seed);
    return seed as f32 / u32::MAX as f32;
}

// Random f32 in the open interval `[0.0, 1.0)`
pub fn rand() -> f32 {
    rand::thread_rng().gen_range(0.0..1.0)
}

// Random f32 in the open interval `[min, max)`
pub fn rand_on(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

// Clamps a number into the interval `min..max`
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x < max {
        x
    } else {
        max
    }
}
