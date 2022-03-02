use rand::Rng;

pub const INFTY: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

// Degrees to radians
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

// Random f64 in the open interval `[0.0, 1.0)`
pub fn rand() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

// Random f64 in the open interval `[min, max)`
pub fn rand_on(min: f64, max: f64) -> f64 {
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
