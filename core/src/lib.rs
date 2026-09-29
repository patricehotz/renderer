use std::ops::Mul;
use std::ops::Add;
pub mod export;
pub mod draw;

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct FrameBuffer {
    width: usize,
    height: usize ,
    pixels: Vec<RGBA>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, pixels: vec![RGBA{r: 255, g: 255, b: 255, a: 100}; width * height ]}
    }

    pub fn get_pixel(&self, Coordinate {x, y}: Coordinate) -> RGBA {
        self.pixels[self.get_index(x, y).unwrap()]
    }

    pub fn set_pixels(&mut self, Coordinate {x, y}: Coordinate, value: RGBA) {
        if let Some(i) = self.get_index(x, y) {
            self.pixels[i] = value;
        };
    }

    pub fn set_pixels_unchecked(&mut self, Coordinate {x, y}: Coordinate, value: RGBA) {
            self.pixels[self.width * y as usize + x as usize] = value;
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < self.width as i32 && x >= 0 && y < self.height as i32 && y >= 0 {
            return Some(self.width * y as usize + x as usize)
        }
        None
    }
}

impl Vec2 {
    pub fn calc_vector(a: Coordinate, b: Coordinate) -> Vec2 {
        Vec2 { x: b.x  - a.x, y:  b.y - a.y }
    }

    pub fn calc_length(&self) -> i32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt().round() as i32
    }
}

impl Coordinate {
    pub fn clamp(a: &mut Coordinate, b: &mut Coordinate,  width: usize, height: usize) {
        let v = Vec2::calc_vector(*a, *b);
        if a.x < 0 {
            a.x = Coordinate::calc_clamp_negative(a.x, b.x, v)
        }
        if b.x < 0 {
            b.x = Coordinate::calc_clamp_negative(b.x, a.x, v)
        }
        if a.y < 0 {
            a.y = Coordinate::calc_clamp_negative(a.y, b.y, v)
        }
        if b.y < 0 {
            b.y = Coordinate::calc_clamp_negative(b.y, a.y, v)
        }
        if a.x > width as i32 -1 {
            a.x = Coordinate::calc_clamp_positive(a.x, b.x, v, width as i32-1)
        }
        if b.x > width as i32 -1 {
            b.x = Coordinate::calc_clamp_positive(b.x, a.x, v, width as i32-1)
        }
        if a.y > width as i32 -1 {
            a.y = Coordinate::calc_clamp_positive(a.y, b.y, v, width as i32-1)
        }
        if b.y > width as i32 -1 {
            b.y = Coordinate::calc_clamp_positive(b.y, a.y, v, width as i32-1)
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
}

impl Mul<Vec2> for Vec2 {
    type Output = i32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2{x: self.x * rhs, y: self.y * rhs}
    }
}


impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2{x: self * rhs.x, y: self * rhs.y}
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2{x: (self * rhs.x as f32).round() as i32, y: (self * rhs.y as f32).round() as i32}

    }
}

impl Add<Vec2> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Vec2) -> Self::Output {
        Coordinate {x: self.x + rhs.x as i32, y: self.y + rhs.y}
    }
}
