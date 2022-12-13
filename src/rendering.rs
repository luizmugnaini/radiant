use crate::{
    camera::{self, Camera},
    color::Color,
    material::Material,
    misc,
    ray::Ray,
    surf::{HitRecord, Sphere},
    surf_list::SurfList,
    vec3::Vec3,
};
use indicatif::{HumanDuration, ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fs::File, io::Write, process::Command, time::Instant};

type Pixel = (u8, u8, u8);
type Line = [Pixel];

fn write_image(filepath: &str, lines: &Line, progress_style: ProgressStyle) {
    eprintln!("=> Writing image to file: {filepath} ...");
    let file = File::create(filepath);

    // Write file
    match file {
        Ok(mut f) => {
            let write_error =
                |we: std::io::Error| panic!("Unable to write to {}: {}", filepath, we);

            // ppm file header
            if let Err(we) = write!(
                f,
                "P3\n{} {}\n255\n",
                camera::IMAGE_WIDTH,
                camera::IMAGE_HEIGHT
            ) {
                write_error(we);
            };

            // Writing progress bar
            let write_progress = ProgressBar::new(lines.len() as u64);
            write_progress.set_style(progress_style);

            let string_pixels: Vec<String> = lines
                .iter()
                .map(|rgb| format!("{} {} {}", rgb.0, rgb.1, rgb.2))
                .collect();

            match f.write_all(string_pixels.join("\n").as_bytes()) {
                Ok(()) => eprintln!("=> Successfully written!"),
                Err(we) => panic!("Unable to write to {}: {}", filepath, we),
            }
        }
        Err(create_err) => {
            panic!("Unable to create file {}: {}", filepath, create_err);
        }
    }

    // Open file for visualization
    if let Err(e) = Command::new("xdg-open").arg(filepath).spawn() {
        eprintln!("=> Error: xdg-open failed to execute, {}", e);
    }
}

fn ray_color(ray: Ray, world: &SurfList, depth: i32) -> Color {
    if depth <= 0 {
        // Exceded maximum number of bounces, considers that the location is
        // near a shadow, so it returns a black pixel
        Color::new(0.0, 0.0, 0.0)
    } else {
        let mut rec = HitRecord::new();
        if world.hit(&ray, 0.001, misc::INFTY, &mut rec) {
            match rec.material().scatter(ray, rec) {
                Some((scattered, attenuation)) => {
                    attenuation * ray_color(scattered, world, depth - 1)
                }
                None => Color::new(0.0, 0.0, 0.0),
            }
        } else {
            let unit_dir = ray.direction().unit();
            let t = 0.5 * (unit_dir.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) as f32 + Color::new(0.5, 0.7, 1.0) * t as f32
        }
    }
}

#[allow(dead_code)]
fn random_scene() -> SurfList {
    eprintln!("=> Creating random scene...");
    let mut world = SurfList::new();

    // Ground
    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
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

pub fn easy_scene() -> SurfList {
    eprintln!("=> Creating easy scene...");
    let material_ground = Material::lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center = Material::lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left = Material::dielectric(1.5);
    let material_right = Material::metal(Color::new(0.8, 0.6, 0.2), 0.0);

    let mut world = SurfList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    ));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    world
}

pub fn render_line(camera: &Camera, world: &SurfList, line_number: usize, line: &mut Line) {
    for (index, pixel) in line.iter_mut().rev().enumerate() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        // Antialiasing process for each pixel
        for _ in 0..camera::SAMPLES_PER_PIXEL {
            let u = (index as f64 + misc::rand()) / (camera::IMAGE_WIDTH - 1) as f64;
            let v = (line_number as f64 + misc::rand()) / (camera::IMAGE_HEIGHT - 1) as f64;
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(r, world, camera::MAX_DEPTH);
        }
        let rgb = pixel_color.rgb();
        (pixel.0, pixel.1, pixel.2) = (rgb.0, rgb.1, rgb.2);
    }
}

pub fn render(filepath: &str, camera: Camera, scene: &str) {
    let started = Instant::now();

    // World where the objects exist
    let world = match scene {
        "random" => random_scene(),
        "easy" => easy_scene(),
        s => panic!("{} not avaliable. Scenes: \"random\" and \"easy\"", s),
    };

    // Progress bar for the line rendering
    let progress_style = ProgressStyle::default_bar()
        .template(concat!(
            "{spinner:.green} [{pos:>3}/{len:3}] ",
            "{bar:40.magenta/blue} [time: {elapsed_precise}]\n{msg}"
        ))
        .progress_chars("=>-")
        .tick_chars("|/-|/-\\");
    let progress_lines = ProgressBar::new(camera::IMAGE_HEIGHT as u64);
    progress_lines.set_style(progress_style.clone());

    // Array containing all pixels from the image
    let mut pixels: Vec<Pixel> =
        vec![(0, 0, 0); (camera::IMAGE_HEIGHT * camera::IMAGE_WIDTH) as usize];
    let lines: Vec<(usize, &mut Line)> = pixels
        .chunks_mut(camera::IMAGE_WIDTH as usize)
        .enumerate()
        .collect();

    // Parallel renderization of the image lines
    eprintln!("=> Image renderization...");
    lines
        .into_par_iter()
        .progress_with(progress_lines.clone())
        .for_each(|(i, l)| render_line(&camera, &world, i, l));
    progress_lines.finish_with_message("=> Finished renderization!");

    // Write image to file
    write_image(filepath, &pixels, progress_style);

    eprintln!("Done in {}!", HumanDuration(started.elapsed()));
}
