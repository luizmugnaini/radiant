use clap::{Arg, Command};
use radiant::{
    camera::{self, Camera},
    misc::{self, LogLevel},
    rendering, scene,
    vec3::Vec3,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let matches = Command::new("radiant")
        .arg(
            Arg::new("outdir")
                .short('o')
                .default_value("./output")
                .help("Output directory path."),
        )
        .arg(
            Arg::new("filename")
                .short('n')
                .default_value("out")
                .help("Output file name, without extensions."),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .value_parser(["ppm", "png"])
                .default_value("ppm")
                .help("Output file format."),
        )
        .arg(
            Arg::new("scene")
                .short('s')
                .value_parser(clap::value_parser!(scene::SceneType))
                .default_value("basic")
                .help("Use a default scene."),
        )
        .get_matches();

    let path = Path::new(matches.get_one::<String>("outdir").unwrap());
    if !path.is_dir() {
        if let Err(e) = fs::create_dir(path) {
            misc::log(
                LogLevel::Fatal,
                &format!("Unable to create output directory due to error {}", e),
            );
            std::process::exit(-1);
        }
    }
    let mut path: PathBuf = path.to_path_buf();
    path.set_file_name(matches.get_one::<String>("filename").unwrap());
    path.set_extension(matches.get_one::<String>("format").unwrap());

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

    rendering::render(path, camera, matches.get_one("scene").unwrap());
}
