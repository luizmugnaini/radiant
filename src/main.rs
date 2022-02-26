// Crate modules
mod camera;
mod color;
mod misc;
mod ray;
mod surf;
mod surf_list;
mod vec3;

// Imports
use camera::Camera;
use color::Color;
use misc::INFTY;
use ray::Ray;
use surf::{HitRecord, Sphere};
use surf_list::SurfList;
use vec3::Vec3;

fn ray_color(ray: Ray<f32>, world: &SurfList, depth: i32) -> Color {
    if depth <= 0 {
        // Exceded maximum number of bounces, considers that the location is
        // near a shadow, so it returns a black pixel
        Color::new(0.0, 0.0, 0.0)
    } else {
        let mut rec = HitRecord::new();
        if world.hit(&ray, 0.001, INFTY, &mut rec) {
            // The ray will be reflected at a random direction. This is done in
            // order to simulate a rough surface and therefore create texture.

            // The `target` is a random point within the unit sphere from the
            // direction of the incoming ray
            let target = rec.point + rec.normal + Vec3::random_unit_vector();

            // The object absorbes 50% of the light and we look for the
            // following reflection
            ray_color(
                Ray::new(rec.point, target - rec.point),
                world,
                depth - 1,
            ) * 0.5
        } else {
            let unit_dir = ray.dir.unit();
            let t = 0.5 * (unit_dir.y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t)
                + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    // World where the objects exist
    let mut world = SurfList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

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
