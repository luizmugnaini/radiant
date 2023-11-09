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

pub enum LogLevel {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}

pub fn log(level: LogLevel, msg: &str) {
    match level {
        LogLevel::Fatal => eprintln!("\x1b[1;41m[FATAL]\x1b[0m: {}", msg),
        LogLevel::Error => eprintln!("\x1b[1;31m[ERROR]\x1b[0m: {}", msg),
        LogLevel::Warning => eprintln!("\x1b[1;33m[WARNING]\x1b[0m: {}", msg),
        LogLevel::Info => eprintln!("\x1b[1;32m[INFO]\x1b[0m: {}", msg),
        LogLevel::Debug => eprintln!("\x1b[1;34m[DEBUG]\x1b[0m: {}", msg),
    }
}
