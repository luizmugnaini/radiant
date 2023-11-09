use crate::{
    camera::{self, Camera},
    color::Color,
    misc::{self, LogLevel},
    ray::Ray,
    scene::{self, SceneType},
    surf::HitRecord,
    surf_list::SurfList,
};
use indicatif::{HumanDuration, ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fs::File, io::Write, path::PathBuf, process::Command, time::Instant};

type Pixel = (u8, u8, u8);
type Image = [Pixel; camera::IMAGE_WIDTH * camera::IMAGE_HEIGHT];

fn write_to_ppm_format(mut file: File, lines: Image, progress_style: ProgressStyle) {
    // ppm file header
    if let Err(write_err) = write!(
        file,
        "P3\n{} {}\n255\n",
        camera::IMAGE_WIDTH,
        camera::IMAGE_HEIGHT
    ) {
        misc::log(
            LogLevel::Fatal,
            &format!("Unable to write ppm header due to {}", write_err),
        );
        std::process::exit(-1);
    };

    // Writing progress bar
    let write_progress = ProgressBar::new(lines.len() as u64);
    write_progress.set_style(progress_style);

    let string_pixels: Vec<String> = lines
        .iter()
        .rev()
        .map(|rgb| format!("{} {} {}", rgb.0, rgb.1, rgb.2))
        .collect();

    match file.write_all(string_pixels.join("\n").as_bytes()) {
        Ok(()) => misc::log(LogLevel::Info, "Successfully written to file!"),
        Err(write_err) => {
            misc::log(
                LogLevel::Fatal,
                &format!("Unable to write to file due to {}", write_err),
            );
            std::process::exit(-1);
        }
    }
}

fn write_image(filepath: PathBuf, lines: Image, progress_style: ProgressStyle) {
    misc::log(
        LogLevel::Info,
        &format!("Writing image to {}", filepath.as_path().display()),
    );
    // Write file
    match File::create(&filepath) {
        Ok(file) => write_to_ppm_format(file, lines, progress_style),
        Err(create_err) => {
            misc::log(
                LogLevel::Fatal,
                &format!("Unable to create output file due to {}", create_err),
            );
            std::process::exit(-1);
        }
    }

    if let Err(e) = Command::new("xdg-open").arg(filepath).spawn() {
        misc::log(
            LogLevel::Error,
            &format!("xdg-open failed to execute, {}", e),
        );
    }
}

fn ray_color(ray: Ray, world: &SurfList, depth: i32) -> Color {
    if depth <= 0 {
        // Exceeded maximum number of bounces, considers that the location is
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

pub fn render_line(camera: &Camera, world: &SurfList, line_number: usize, line: &mut [Pixel]) {
    for (index, pixel) in line.iter_mut().rev().enumerate() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        // Antialiasing process for each pixel
        for _ in 0..camera::SAMPLES_PER_PIXEL {
            let u = (index as f32 + misc::rand()) / (camera::IMAGE_WIDTH - 1) as f32;
            let v = (line_number as f32 + misc::rand()) / (camera::IMAGE_HEIGHT - 1) as f32;
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(r, world, camera::MAX_DEPTH);
        }
        let rgb = pixel_color.rgb();
        (pixel.0, pixel.1, pixel.2) = (rgb.0, rgb.1, rgb.2);
    }
}

pub fn render(output_path: PathBuf, camera: Camera, scene_type: &SceneType) {
    let started = Instant::now();

    let world = scene::make_scene(scene_type);

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
    let mut pixels: [Pixel; camera::IMAGE_HEIGHT * camera::IMAGE_WIDTH] =
        [(0, 0, 0); camera::IMAGE_HEIGHT * camera::IMAGE_WIDTH];

    // Split the whole image into lines (with the width size) and enumerate them.
    let lines: Vec<(usize, &mut [Pixel])> = pixels
        .chunks_mut(camera::IMAGE_WIDTH as usize)
        .enumerate()
        .collect();

    misc::log(LogLevel::Info, "Starting parallel image renderization...");
    lines
        .into_par_iter()
        .progress_with(progress_lines.clone())
        .for_each(|(i, l)| render_line(&camera, &world, i, l));
    progress_lines.finish_with_message("Finished renderization!");

    // Write image to file
    write_image(output_path, pixels, progress_style);

    misc::log(
        LogLevel::Info,
        &format!("Done in {}!", HumanDuration(started.elapsed())),
    );
}
