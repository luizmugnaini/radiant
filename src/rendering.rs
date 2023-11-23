use crate::{
    camera::{self, Camera},
    color::Color,
    misc::{self, LogLevel},
    ray::Ray,
    scene::{self, SceneType},
    surf::HitRecord,
    surf_list::SurfList,
};
use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::path::Path;

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

pub fn render(output_path: &Path, camera: Camera, scene_type: SceneType) {
    let world = scene::make_scene(scene_type);

    let progress_style = ProgressStyle::default_bar()
        .template(concat!(
            "{spinner:.green} [{pos:>3}/{len:3}] ",
            "{bar:40.magenta/blue} [time: {elapsed_precise}]\n{msg}"
        ))
        .progress_chars("=>-")
        .tick_chars("|/-|/-\\");
    let progress_lines = ProgressBar::new(camera::IMAGE_HEIGHT as u64);
    progress_lines.set_style(progress_style);

    let mut pixel_buffer = [Rgb::from([0u8, 0u8, 0u8]); camera::IMAGE_HEIGHT * camera::IMAGE_WIDTH];
    pixel_buffer
        .par_chunks_mut(camera::IMAGE_WIDTH)
        .enumerate()
        .for_each(|(pixel_y, row)| {
            for (pixel_x, px) in row.iter_mut().enumerate() {
                let mut px_col = Color::default();
                for _ in 0..camera::SAMPLES_PER_PIXEL {
                    let u = (pixel_x as f32 + misc::rand()) / (camera::IMAGE_WIDTH - 1) as f32;
                    let v = (pixel_y as f32 + misc::rand()) / (camera::IMAGE_HEIGHT - 1) as f32;
                    let r = camera.get_ray(u, v);
                    px_col += ray_color(r, &world, camera::MAX_DEPTH);
                }
                *px = px_col.rgb();
            }
            progress_lines.inc(1);
        });

    let img = ImageBuffer::from_fn(
        camera::IMAGE_WIDTH as u32,
        camera::IMAGE_HEIGHT as u32,
        |x, y| {
            pixel_buffer[x as usize + (camera::IMAGE_HEIGHT - 1 - y as usize) * camera::IMAGE_WIDTH]
        },
    );
    let _ = img.save(output_path);
}
