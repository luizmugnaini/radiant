mod color;
mod ray;
mod surf;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: &Vec3<f32>, radius: f32, ray: &Ray<f32>) -> f32 {
    let oc: Vec3<f32> = ray.origin - *center;
    let a = ray.dir.dot(&ray.dir);
    let half_b = oc.dot(&ray.dir);
    let discriminant = {
        let c = oc.dot(&oc) - radius * radius;
        half_b * half_b - a * c
    };

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray<f32>) -> Color {
    let t: f32 = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let normal: Vec3<f32> = (ray.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
    } else {
        let unit_dir = ray.direction().unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin: Vec3<f32> = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3<f32> = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3<f32> = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3<f32> =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("{} lines to go", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            ray_color(r).write_color()
        }
    }
    eprintln!("Done!");
}
