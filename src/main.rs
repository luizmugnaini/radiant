use clap::{Arg, Command as ClapCommand};
use radiant::{
    camera::{self, Camera},
    misc::{self, LogLevel},
    rendering, scene,
    vec3::Vec3,
};
use std::{fs, path::PathBuf, process::Command};

fn main() {
    let matches = ClapCommand::new("radiant")
        .arg(
            Arg::new("outdir")
                .short('o')
                .default_value("./output/")
                .help("Output directory path."),
        )
        .arg(
            Arg::new("filename")
                .short('n')
                .default_value("out")
                .help("Output file name, without extensions."),
        )
        .arg(
            Arg::new("scene")
                .short('s')
                .default_value("basic")
                .help("Use a default scene. Possible values are `basic` and `complex`."),
        )
        .get_matches();

    let mut path = PathBuf::new();
    path.push(matches.get_one::<String>("outdir").unwrap());
    if !path.is_dir() {
        if let Err(e) = fs::create_dir(path.as_path()) {
            misc::log(
                LogLevel::Fatal,
                &format!("Unable to create output directory due to error {}", e),
            );
            std::process::exit(-1);
        }
    }
    path.push(matches.get_one::<String>("filename").unwrap());
    path.set_extension("png");
    misc::log(
        LogLevel::Debug,
        &format!("Output path: {}", path.as_path().display()),
    );

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

    rendering::render(
        path.as_path(),
        camera,
        scene::SceneType::from(&matches.get_one::<String>("scene").unwrap()),
    );

    if let Err(e) = Command::new("xdg-open").arg(path).spawn() {
        misc::log(
            LogLevel::Error,
            &format!("xdg-open failed to execute, {}", e),
        );
    }
}
