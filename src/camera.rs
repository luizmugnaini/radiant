use crate::{misc, ray::Ray, vec3::Vec3};

/// Aspect ratio of the displayed image .
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

/// Width of the displayed image.
pub const IMAGE_WIDTH: usize = 400;

/// Height of the displayed image. It is given by the image width divided by the
/// aspect ratio.
pub const IMAGE_HEIGHT: usize = 225; // (400.0 * 9.0 / 16.0) as i32;

/// Number of samples used per pixel in the process of aliasing.
pub const SAMPLES_PER_PIXEL: i32 = 20;

/// Maximum number of bounces of a ray.
pub const MAX_DEPTH: i32 = 8;

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    origin: Vec3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
    lower_left_corner: Vec3<f32>,
    ortho_basis: (Vec3<f32>, Vec3<f32>, Vec3<f32>),
    lens_radius: f32,
}

impl Camera {
    /// Create a new instance of a `Camera`.
    ///
    /// # Arguments
    ///
    /// * `lookfrom` - The point the camera looks from.
    /// * `lookat` - The point the camera looks at.
    /// * `vup` - Vector up.
    /// * `vfov` - Vertical field-of-view in degrees.
    /// * `aspect_ratio` - Aspect ratio of the camera.
    /// * `aperture` - Aperture of the camera.
    /// * `focus_dist` - Distance of focus.
    pub fn new(
        lookfrom: Vec3<f32>,
        lookat: Vec3<f32>,
        vup: Vec3<f32>,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        // Viewport
        let theta = misc::deg_to_rad(vfov);
        let h = f32::tan(theta / 2.0);
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
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

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

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk(&mut rand::thread_rng()) * self.lens_radius;
        let offset = self.ortho_basis.1 * rd.x() + self.ortho_basis.2 * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
