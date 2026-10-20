use std::fmt::Write;
use crate::coordinate::Coordinate;
use crate::framebuffer::FrameBuffer;
use crate::rgb::RGB;

pub fn ppm(buffer: &FrameBuffer, dir: &str) {
    let mut value = format!("P3\n{} {}\n255\n", buffer.width, buffer.height);
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let RGB {r, g, b, ..} = buffer.get_pixel(Coordinate::new(x as i32, y as i32, 1.0));
            write!(value, "{} {} {} ", r, g, b).unwrap()
        }
        if y < buffer.height-1 {
            writeln!(value).unwrap()
        }
    };

    std::fs::write(dir, value).unwrap()
}
