use crate::{Buffer, Coordinate, RGBA};
use std::fmt::Write;

pub fn ppm(buffer: &Buffer, dir: String) {
    let mut value = format!("p3\n{} {}\n255\n", buffer.width, buffer.height);
    for y in 0..buffer.height {
        for x in 0..buffer.width-1 {
            let RGBA {r, g, b, ..} = buffer.get_pixel(Coordinate{x, y});
            write!(value, "{} {} {}", r, g, b).unwrap()
        }
        if (y < buffer.height-1) {
            writeln!(value).unwrap()
        }
    };
}
