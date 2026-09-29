use crate::{Coordinate, FrameBuffer, RGBA, Vec2 };

pub fn gradiant(buffer: &mut FrameBuffer, from_p: Coordinate, from_c: RGBA, to_p: Coordinate, to_c: RGBA) {
    let d = Vec2::calc_vector(from_p, to_p);

    for y in 0..buffer.height as i32 {
        for x in 0..buffer.width as i32 {
            let pos = Coordinate{x,y};
            let v = Vec2::calc_vector(from_p, pos);
            let t: f32 = (d*v) as f32 / (d*d) as f32;
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

pub fn line(buffer: &mut FrameBuffer, mut a: Coordinate, mut b: Coordinate, color: RGBA) {
    if a.x > b.x {
        std::mem::swap(&mut a, &mut b);
    }

    let vec = Vec2::calc_vector(a, b);
    let steps =  vec.x.abs().max(vec.y.abs());
    let step_x: f32 = vec.x as f32 / steps as  f32;
    let step_y: f32 = vec.y as f32 / steps as f32;

    for t in 0..steps {
        let v = Vec2 {x:( t as f32 * step_x).round() as i32, y: ( t as f32 * step_y).round() as i32};
        let p = a + v ;
        buffer.set_pixels(p, color)
    }

    buffer.set_pixels(b, color)
}