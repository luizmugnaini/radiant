use crate::{
    color::Color,
    material::Material,
    misc::{self, LogLevel},
    surf::Sphere,
    surf_list::SurfList,
    vec3::Vec3,
};
use rand::Rng;

#[derive(Clone, clap::Parser, clap::ValueEnum)]
pub enum SceneType {
    Basic,
    Complex,
}

impl SceneType {
    pub fn from(scene_name: &str) -> Self {
        match scene_name {
            "basic" => Self::Basic,
            "complex" => Self::Complex,
            _ => {
                misc::log(
                    LogLevel::Fatal,
                    &format!(
                        "Scene name {} not available [options: basic, complex].",
                        scene_name
                    ),
                );
                std::process::exit(-1);
            }
        }
    }
}

fn complex_scene() -> SurfList {
    misc::log(LogLevel::Info, "Creating complex scene");
    let mut world = SurfList::new();

    // Ground
    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = rand::thread_rng();

    // Random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = misc::rand();
            let center = Vec3::new(
                a as f32 + 0.9 * misc::rand(),
                0.2,
                b as f32 + 0.9 * misc::rand(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material = if choose_material < 0.8 {
                    // Lambertian
                    let albedo = Color::rand(&mut rng) * Color::rand(&mut rng);
                    Material::lambertian(albedo)
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::rand_on(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
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

fn basic_scene() -> SurfList {
    misc::log(LogLevel::Info, "Creating basic scene");
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

pub fn make_scene(scene_type: SceneType) -> SurfList {
    match scene_type {
        SceneType::Basic => basic_scene(),
        SceneType::Complex => complex_scene(),
    }
}
