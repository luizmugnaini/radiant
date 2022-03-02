use crate::{misc, ray::Ray, vec3::Vec3};

// Image constants
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGHT: i32 = (400.0 * 9.0 / 16.0) as i32;
pub const SAMPLES_PER_PIXEL: i32 = 45; // 100
pub const MAX_DEPTH: i32 = 40; // 50

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    origin: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        // Viewport
        let theta = misc::deg_to_rad(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        // Positioning
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3::new(0.0, 0.0, focal_length);

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        )
    }
}
