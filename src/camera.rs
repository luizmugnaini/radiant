use crate::{misc, ray::Ray, vec3::Vec3};

// Image constants
pub const ASPECT_RATIO: f64 = 16.0 / 9.0; // 16.0 / 9.0
pub const IMAGE_WIDTH: i32 = 400; // 400
pub const IMAGE_HEIGHT: i32 = (400.0 * 9.0 / 16.0) as i32; // img_width / asp_ratio
pub const SAMPLES_PER_PIXEL: i32 = 100; // 100
pub const MAX_DEPTH: i32 = 50; // 50

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    origin: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    ortho_basis: (Vec3<f64>, Vec3<f64>, Vec3<f64>),
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3<f64>, // Point the camera looks from
        lookat: Vec3<f64>,   // Point the camera looks
        vup: Vec3<f64>,      // Vector up
        vfov: f64,           // Vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        // Viewport
        let theta = misc::deg_to_rad(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // Distance between the projection point and the image plane
        let focal_length = 1.0;

        // Orthonormal basis
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Camera positioning
        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            ortho_basis: (u, v, w),
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.ortho_basis.1 * rd.x() + self.ortho_basis.2 * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin
                - offset,
        )
    }
}
