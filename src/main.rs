// Crate modules
mod camera;
mod color;
mod material;
mod misc;
mod ray;
mod surf;
mod surf_list;
mod vec3;

// Imports
use camera::Camera;
use color::Color;
use material::Material;
use misc::INFTY;
use ray::Ray;
use surf::{HitRecord, Sphere};
use surf_list::SurfList;
use vec3::Vec3;

fn ray_color(ray: Ray, world: &SurfList, depth: i32) -> Color {
    if depth <= 0 {
        // Exceded maximum number of bounces, considers that the location is
        // near a shadow, so it returns a black pixel
        Color::new(0.0, 0.0, 0.0)
    } else {
        let mut rec = HitRecord::new();
        if world.hit(&ray, 0.001, INFTY, &mut rec) {
            match rec.material().scatter(ray, rec) {
                Some((scattered, attenuation)) => {
                    attenuation * ray_color(scattered, world, depth - 1)
                }
                None => Color::new(0.0, 0.0, 0.0),
            }
        } else {
            let unit_dir = ray.direction().unit();
            let t = 0.5 * (unit_dir.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) as f32
                + Color::new(0.5, 0.7, 1.0) * t as f32
        }
    }
}

fn main() {
    // Various kinds of materials that compose the scene
    let material_left = Material::lambertian(Color::new(0.0, 0.0, 1.0));
    let material_right = Material::lambertian(Color::new(1.0, 0.0, 0.0));

    // World where the objects exist
    let r = f64::cos(misc::PI / 4.0);
    let mut world = SurfList::new();
    world.add(Sphere::new(Vec3::new(-r, 0.0, -1.0), r, material_left));
    world.add(Sphere::new(Vec3::new(r, 0.0, -1.0), r, material_right));

    let camera = Camera::new(90.0, camera::ASPECT_RATIO);

    // Render to ppm format
    println!(
        "P3\n{} {}\n255\n",
        camera::IMAGE_WIDTH,
        camera::IMAGE_HEIGHT
    );

    for j in (0..camera::IMAGE_HEIGHT).rev() {
        eprintln!("{} lines to go", j);
        for i in 0..camera::IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            // Antialiasing process for each pixel
            for _ in 0..camera::SAMPLES_PER_PIXEL {
                let u = (i as f64 + misc::rand())
                    / (camera::IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + misc::rand())
                    / (camera::IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, camera::MAX_DEPTH);
            }
            pixel_color.write_color()
        }
    }
    eprintln!("Done!");
}
