use crate::{Buffer, Coordinate, RGBA};
use std::fmt::Write;

pub fn ppm(buffer: &Buffer, dir: &str) {
    let mut value = format!("P3\n{} {}\n255\n", buffer.width, buffer.height);
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let RGBA {r, g, b, ..} = buffer.get_pixel(Coordinate{x: x as i32, y: y as i32});
            write!(value, "{} {} {} ", r, g, b).unwrap()
        }
        if y < buffer.height-1 {
            writeln!(value).unwrap()
        }
    };

    std::fs::write(dir, value).unwrap()
}
