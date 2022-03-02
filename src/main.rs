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

fn random_scene() -> SurfList {
    let mut world = SurfList::new();

    // Ground
    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.0, 0.0),
        1000.0,
        ground_material,
    ));

    // Random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = misc::rand();
            let center = Vec3::new(
                a as f64 + 0.9 * misc::rand(),
                0.2,
                b as f64 + 0.9 * misc::rand(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material = if choose_material < 0.8 {
                    // Lambertian
                    let albedo = Color::rand() * Color::rand();
                    Material::lambertian(albedo)
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::rand_on(0.5, 1.0);
                    let fuzz = misc::rand_on(0.0, 0.5);
                    Material::metal(albedo, fuzz)
                } else {
                    // Glass
                    Material::dielectric(1.5)
                };

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    // Standard spheres to all scenes
    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::dielectric(1.5),
    ));
    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::lambertian(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::metal(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

fn main() {
    // World where the objects exist
    let world = random_scene();

    // Create a camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        camera::ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
