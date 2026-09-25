use crate::{Buffer, Coordinate, RGBA, Vec2};

pub fn gradiant(buffer: &mut Buffer, from_p: Coordinate, from_c: RGBA, to_p: Coordinate, to_c: RGBA) {
    let d = Vec2::calc_vector(from_p, to_p);

    for y in 0..buffer.height {
        for x in 0..buffer.width {
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
