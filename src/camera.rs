use crate::{ray::Ray, vec3::Vec3};

// Image constants
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGHT: i32 = (400.0 * 9.0 / 16.0) as i32;
pub const SAMPLES_PER_PIXEL: i32 = 100;

// Camera constants
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTHT: f32 = 2.0 * 16.0 / 9.0;
const FOCAL_LENGTH: f32 = 1.0;

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    origin: Vec3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
    lower_left_corner: Vec3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTHT, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            aspect_ratio: ASPECT_RATIO,
            viewport_height: VIEWPORT_HEIGHT,
            viewport_width: VIEWPORT_WIDTHT,
            focal_length: FOCAL_LENGTH,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray<f32> {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        )
    }
}
