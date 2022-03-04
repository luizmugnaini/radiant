use radiant::{
    camera::{self, Camera},
    rendering,
    vec3::Vec3,
};
use std::{env, fs};

fn check_output_dir(output: &str) -> std::io::Result<bool> {
    let mut path = env::current_dir()?;
    path.push(output);
    let meta = fs::metadata(path)?;
    Ok(meta.is_dir())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("File name path is needed!");
    }

    // File path operations
    let create_out = || {
        if let Err(e) = fs::create_dir("output") {
            panic!("Unable to create directory output: {}", e);
        }
    };
    match check_output_dir("output") {
        Ok(b) => {
            if !b {
                create_out();
            }
        }
        Err(_) => create_out(),
    }
    let mut filepath = String::from(args[1].as_str());
    filepath.push_str(".ppm");
    filepath.insert_str(0, "output/");

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

    rendering::render(&filepath, camera, &args[2]);
}
