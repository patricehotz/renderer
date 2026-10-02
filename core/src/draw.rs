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

const TOP:   u8 = 0b0001;
const LEFT:  u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const RIGHT: u8 = 0b1000;
const NEGATIVE: u8 = TOP | LEFT;

pub fn clip(a: &mut Coordinate, b: &mut Coordinate,  width: usize, height: usize) -> bool {
    let bounds_a = get_bounds(*a, width, height);
    let bounds_b = get_bounds(*b, width, height);
    let mut t_enter = 0.0;
    let mut t_leave = 0.0;


    if (bounds_a | bounds_b) <= 0  { true; }
    if bounds_a & bounds_b > 0 { false; }

    if bounds_a & TOP > 0 {
        t_enter = calc_t_enter(a.y, b.y);
    }
    if bounds_a & LEFT > 0 {
        t_enter = t_enter.max(calc_t_enter(a.x, b.x))
    }
    if bounds_a & BOTTOM > 0 {
        t_enter = t_enter.max(calc_t_leave(a.y, b.y, height-1))
    }
    if bounds_a & RIGHT > 0 {
        t_enter = t_enter.max(calc_t_leave(a.x, b.x, width-1))
    }
    if bounds_b & TOP > 0 {
        t_enter = calc_t_enter(b.y, a.y);
    }
    if bounds_b & LEFT > 0 {
        t_enter = t_enter.max(calc_t_enter(b.x, a.x))
    }
    if bounds_b & BOTTOM > 0 {
        t_leave = calc_t_leave(b.y, a.y, height-1)
    }
    if bounds_b & RIGHT > 0 {
        t_leave = t_leave.max(calc_t_leave(b.x, a.x, width-1))
    }

    if t_enter > t_leave

}

fn get_bounds(p: Coordinate, width: usize, height: usize) -> u8 {
    let mut result: u8 = 0b0000;
    if p.y < 0 { result |= TOP }
    if p.x < 0 { result |= LEFT }
    if p.y > height as i32 {  result |= BOTTOM }
    if p.x > width as i32 {  result |= RIGHT }
    result
}

fn calc_t_enter(a: i32, b: i32) -> f32 {
    - (a as f32) / (b - a) as f32
}

fn calc_t_leave(a: i32, b: i32, max: usize) -> f32 {
    max as f32 - (a as f32) / (b - a) as f32
}