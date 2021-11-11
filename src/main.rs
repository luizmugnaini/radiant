mod color;
mod render;
mod v3;

use color::{Color, RGB};
use v3::Vec3;

fn test_v3() {
    let v: Vec3<i32> = Vec3 { x: 2, y: 3, z: 6 };
    let c: i32 = 5;
    println!("v = {:?}", v);

    let y = v * c;
    println!("v * c = y = {:?}", y);

    let t = Vec3 { x: 1, y: 1, z: 1 };
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
    let mut c = Vec3 { x: 5, y: 10, z: 15 };
    c /= 5;
    println!(
        "div: u = ({}, {}, {}); div assign: c = ({}, {}, {})",
        ud5[0], ud5[1], ud5[2], c[0], c[1], c[2]
    );
    println!("dot product u * c = {:?}", ud5.dot(&c));
    println!("len c = {}", c.len());
}

fn test_color() {
    let white = Color::new(255, 255, 255);
    println!(
        "white = ({}, {}, {})",
        white[RGB::Red],
        white[RGB::Green],
        white[RGB::Blue]
    );
}

fn test_ppm() {
    let width = 300;
    let height = 200;
    render::write_ppm(&width, &height);
}

fn main() {
    test_ppm();
    //test_v3();
    //test_color();
}
