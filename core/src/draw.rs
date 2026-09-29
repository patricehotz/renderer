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
    let mut vec = Vec2::calc_vector(a, b);
    let steep = vec.x.abs() < vec.y.abs();

    if steep {
        std::mem::swap(&mut vec.x, &mut vec.y);
    }

    if vec.x <= 0 {
        vec = vec * -1;
        std::mem::swap(&mut a, &mut b)
    }

    let y_direction = if vec.y < 0 {-1} else {1};

    //(vec.y / vec.x) * 2 * vec.x = 2 * vec.y
    let step_y  = vec.y.abs() * 2;
    let mut current_step_y = 0;

    for _ in 0..vec.x {
        if current_step_y >= vec.x{
            if steep {a.x += y_direction} else {a.y += y_direction}
            current_step_y -= 2 * vec.x
        }
        current_step_y += step_y;
        buffer.set_pixels(a, color);

        if steep {a.y += 1} else {a.x += 1}
    }

    buffer.set_pixels(b, color)
}