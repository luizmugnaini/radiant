mod v3;

mod ppm {
    use std::fs;

    pub fn write(img_width: &i32, img_height: &i32, img_name: &str) -> () {
        let mut content: String = format!("P3\n{} {}\n255\n", img_width, img_height);

        for j in (0..*img_height).rev() {
            eprintln!("{} lines to go", j);
            for i in 0..*img_width {
                let r: f32 = i as f32 / (img_width - 1) as f32;
                let g: f32 = j as f32 / (img_height - 1) as f32;
                let b: f32 = 0.25;

                let imr = (255.999 * r) as i32;
                let img = (255.999 * g) as i32;
                let imb = (255.999 * b) as i32;
                content.push_str(&format!("{} {} {}\n", imr, img, imb));
            }
        }

        let path: String = format!("img/{}.ppm", img_name);
        fs::write(path, content).expect("Could not write image");
        eprintln!("Done!");
    }
}

use v3::V3;

fn test_v3() {
    let v: V3<i32> = V3 { x: 2, y: 3, z: 6 };
    let c: i32 = 5;
    println!("v = {:?}", v);

    let y = v * c;
    println!("v * c = y = {:?}", y);

    let t = V3 { x: 1, y: 1, z: 1 };
    let mut u = y + t;
    println!("u = y + t = {:?}", u);

    println!("indices u = ({}, {}, {})", u[0], u[1], u[2]);

    println!("checking index mut");
    u[0] = 300;
    println!(
        "assign 300 to first yield: u = ({}, {}, {})",
        u[0], u[1], u[2]
    );

    println!("division by {}", 5);
    let ud5 = u / 5;
    let mut c = V3 { x: 5, y: 10, z: 15 };
    c /= 5;
    println!(
        "div: u = ({}, {}, {}); div assign: u = ({}, {}, {})",
        ud5[0], ud5[1], ud5[2], c[0], c[1], c[2]
    );
}

fn test_ppm() {
    let width = 300;
    let height = 200;
    ppm::write(&width, &height, &"img0");
}

fn main() {
    test_ppm();
    test_v3();
}
