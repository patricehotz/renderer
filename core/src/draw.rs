use crate::{Buffer, Coordinate, RGBA, Vec2};
use std::fmt::Write;

pub fn gradiant(buffer: &mut Buffer, from_p: Coordinate, from_c: RGBA, to_p: Coordinate, to_c: RGBA) {
    let _v_d = get_vector(from_p, to_p);

    for y in 0..buffer.height {
        for x in 0..buffer.width-1 {
            let p = Coordinate{x,y};
            let _v_v = get_vector(from_p, p);
            let t = (_v_d.x * _v_v.x  + _v_d.y * _v_v.y)/(_v_d.x.powi(2) + _v_d.y.powi(2) );
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

fn get_vector(a: Coordinate, b: Coordinate) -> Vec2 {
    Vec2 { x: b.x as f32 - a.x as f32, y:  b.y as f32 - a.y as f32}
}