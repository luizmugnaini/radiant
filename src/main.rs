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
use material::{Lambertian, Material, Metal};
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
            Color::new(1.0, 1.0, 1.0) * (1.0 - t)
                + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    // Various kinds of materials that compose the scene
    let material_ground =
        Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center =
        Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right =
        Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2)));

    // World where the objects exist
    let mut world = SurfList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let camera = Camera::new();

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
                let u = (i as f32 + misc::rand())
                    / (camera::IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + misc::rand())
                    / (camera::IMAGE_HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, camera::MAX_DEPTH);
            }
            pixel_color.write_color()
        }
    }
    eprintln!("Done!");
}
