use crate::{Coordinate, FrameBuffer, RGBA, Vec2 };

pub fn gradiant(buffer: &mut FrameBuffer, from_p: Coordinate, from_c: RGBA, to_p: Coordinate, to_c: RGBA) {
    let d = Vec2::calc_vector(from_p, to_p);

    for y in 0..buffer.height as i32 {
        for x in 0..buffer.width as i32 {
            let pos = Coordinate{x,y};
            let v = Vec2::calc_vector(from_p, pos);
            let t = (d*v)/(d*d);
            let res = RGBA {
                r: ((1.0-t) * from_c.r as f32 + t * to_c.r as f32).round() as u8,
                g: ((1.0-t) * from_c.g as f32 + t * to_c.g as f32).round() as u8,
                b: ((1.0-t) * from_c.b as f32 + t * to_c.b as f32).round() as u8,
                a: 100
            };

            buffer.set_pixels(Coordinate {x, y}, res)
        }
    };
}

pub fn line(buffer: &mut FrameBuffer ,a: Coordinate, b: Coordinate, color: RGBA) {
    let vec = Vec2::calc_vector(a, b);
    let length = vec.calc_length() as u32;

    for t in 0..length {
        let t: f32 = t as f32 / length as f32;
        let p = a + vec * t;
        buffer.set_pixels(p, color)
    }
}