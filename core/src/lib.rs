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

pub struct BBox {
    min: Coordinate,
    max: Coordinate,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, pixels: vec![RGBA{r: 0, g: 0, b: 0, a: 0}; width * height ]}
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

impl BBox {
    pub fn new (a: Coordinate, b: Coordinate, c: Coordinate) -> Self {
        Self {min: Coordinate { x: a.x.min(b.x.min(c.x)), y: a.y.min(b.y.min(c.y))}, max: Coordinate {x: a.x.max(b.x.max(c.x)), y: a.y.max(b.y.max(c.y))}}
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
