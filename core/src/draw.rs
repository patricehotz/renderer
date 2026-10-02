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

pub fn line(buffer: &mut FrameBuffer, mut a: Coordinate, mut b: Coordinate, color: RGBA)  {
    let line_visible = clip(&mut a,  &mut b, buffer.width -1, buffer.height -1);

    if line_visible != true {return;} 

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
        buffer.set_pixels_unchecked(p, color)
    }

    buffer.set_pixels_unchecked(b, color)
}

const TOP:   u8 = 0b0001;
const LEFT:  u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const RIGHT: u8 = 0b1000;

pub fn clip(a: &mut Coordinate, b: &mut Coordinate,  max_x: usize, max_y: usize) -> bool {
    let bounds_a = get_bounds(*a, max_x, max_y);
    let bounds_b = get_bounds(*b, max_x, max_y);
    let vec = Vec2::calc_vector(*a, *b);
    let mut t_enter = 0.0;
    let mut t_leave = 1.0;


    if (bounds_a | bounds_b) <= 0  { return true; }
    if bounds_a & bounds_b > 0 { return false; }

    if bounds_a & TOP > 0 {
        t_enter = calc_t_min(a.y, b.y);
    }
    if bounds_a & LEFT > 0 {
        t_enter = t_enter.max(calc_t_min(a.x, b.x))
    }
    if bounds_a & BOTTOM > 0 {
        t_enter = t_enter.max(calc_t_max(a.y, b.y, max_y))
    }
    if bounds_a & RIGHT > 0 {
        t_enter = t_enter.max(calc_t_max(a.x, b.x, max_x))
    }
    if bounds_b & TOP > 0 {
        t_leave = calc_t_min(a.y, b.y);
    }
    if bounds_b & LEFT > 0 {
        t_leave = t_leave.min(calc_t_min(a.x, b.x))
    }
    if bounds_b & BOTTOM > 0 {
        t_leave = t_leave.min(calc_t_max(a.y, b.y, max_y))
    }
    if bounds_b & RIGHT > 0 {
        t_leave = t_leave.min(calc_t_max(a.x, b.x, max_x))
    }

    if t_enter > t_leave { return false; }

    *b = *a + t_leave * vec;
    *a = *a + t_enter * vec;
    true

}

fn get_bounds(p: Coordinate, max_x: usize, max_y: usize) -> u8 {
    let mut result: u8 = 0b0000;
    if p.y < 0 { result |= TOP }
    if p.x < 0 { result |= LEFT }
    if p.y > max_y as i32 {  result |= BOTTOM }
    if p.x > max_x as i32 {  result |= RIGHT }
    result
}

fn calc_t_min(a: i32, b: i32) -> f32 {
    - (a as f32) / (b - a) as f32
}

fn calc_t_max(a: i32, b: i32, max: usize) -> f32 {
    (max as f32 - (a as f32)) / (b - a) as f32
}