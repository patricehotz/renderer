use crate::bbox::BBox;
use crate::coordinate::Coordinate;
use crate::framebuffer::FrameBuffer;
use crate::point::Point;
use crate::rgba::RGBA;
use crate::vec2::Vec2;

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle {a, b, c}
    }
}

pub fn triangle(buffer: &mut FrameBuffer, Triangle {a, b, c}: Triangle) {
    let b_box = BBox::new(a.coordinate,b.coordinate,c.coordinate);
    let ab = Vec2::calc_vector(a.coordinate, b.coordinate);
    let ac = Vec2::calc_vector(a.coordinate, c.coordinate);
    let total_area = ab.signed_c_product(ac);
    if total_area > 1 {return;}

    for y in b_box.min.y..b_box.max.y {
        for x in b_box.min.x..b_box.max.x {
            let p = Coordinate::new(x, y, 1);
            let pa = Vec2::calc_vector(p, a.coordinate);
            let pb = Vec2::calc_vector(p, b.coordinate);
            let pc = Vec2::calc_vector(p, c.coordinate);

            let alpha = pb.signed_c_product(pc) as f32 / total_area as f32;
            let beta = pc.signed_c_product(pa) as f32 / total_area as f32;
            let gamma = pa.signed_c_product(pb) as f32 / total_area as f32;

            if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {continue}

            let color  = RGBA {
                r: (alpha * a.color.r as f32 + beta * b.color.r as f32 + gamma * c.color.r as f32).round() as u8,
                g: (alpha * a.color.g as f32 + beta * b.color.g as f32 + gamma * c.color.g as f32).round() as u8,
                b: (alpha * a.color.b as f32 + beta * b.color.b as f32 + gamma * c.color.b as f32).round() as u8,
                a: 100
            };

            buffer.set_pixels(p, color)

        }
    }
}
