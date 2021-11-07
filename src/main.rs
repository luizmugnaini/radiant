use std::fs;

fn main() {
    let width = 300;
    let height = 200;

    let mut content: String = format!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let r: f32 = i as f32 / (width - 1) as f32;
            let g: f32 = j as f32 / (height - 1) as f32;
            let b: f32 = 0.25;

            let imr = (255.999 * r) as i32;
            let img = (255.999 * g) as i32;
            let imb = (255.999 * b) as i32;
            content.push_str(&format!("{} {} {}\n", imr, img, imb));
        }
    }
    fs::write("img/img0.ppm", content);
}
