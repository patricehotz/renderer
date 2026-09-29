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
    clip(&mut a, &mut b, buffer.width, buffer.height);

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

pub fn clip(a: &mut Coordinate, b: &mut Coordinate,  width: usize, height: usize) {
    let v = Vec2::calc_vector(*a, *b);
    if a.x < 0 {
        a.x = calc_clamp_negative(a.x, b.x, v)
    }
    if b.x < 0 {
        b.x = calc_clamp_negative(b.x, a.x, v)
    }
    if a.y < 0 {
        a.y = calc_clamp_negative(a.y, b.y, v)
    }
    if b.y < 0 {
        b.y = calc_clamp_negative(b.y, a.y, v)
    }
    if a.x > width as i32 -1 {
        a.x = calc_clamp_positive(a.x, b.x, v, width as i32-1)
    }
    if b.x > width as i32 -1 {
        b.x = calc_clamp_positive(b.x, a.x, v, width as i32-1)
    }
    if a.y > width as i32 -1 {
        a.y = calc_clamp_positive(a.y, b.y, v, height as i32-1)
    }
    if b.y > width as i32 -1 {
        b.y = calc_clamp_positive(b.y, a.y, v, height as i32-1)
   }
}
           
fn calc_clamp_negative(a: i32, b: i32, v: Vec2) -> i32 {
    let t = 0 as f32 - (a as f32) / (b - a) as f32;
    (a as f32 * t) as i32
}

fn calc_clamp_positive(a: i32, b: i32, v: Vec2, max: i32) -> i32 {
    let t = max as f32 - (a as f32) / (b - a) as f32;
    (a as f32 * t) as i32
}