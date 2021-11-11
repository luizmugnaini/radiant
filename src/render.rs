use crate::color::Color;

/// Write ppm image to stdout
pub fn write_ppm(img_width: &i32, img_height: &i32) -> () {
    println!("P3\n{} {}\n255\n", img_width, img_height);

    for j in (0..*img_height).rev() {
        eprintln!("{} lines to go", j);
        for i in 0..*img_width {
            let color = Color::to_rgb(
                i as f32 / (img_width - 1) as f32,
                j as f32 / (img_height - 1) as f32,
                0.25,
            );
            color.write_color();
        }
    }
    eprintln!("Done!");
}
