use std::ops::Mul;
pub mod export;
pub mod draw;

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Buffer {
    width: usize,
    height: usize ,
    pixels: Vec<RGBA>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, pixels: vec![RGBA{r: 255, g: 255, b: 255, a: 100}; width * height ]}
    }

    pub fn get_pixel(&self, Coordinate {x, y}: Coordinate) -> RGBA {
        self.pixels[self.get_index(x, y)]
    }

    pub fn set_pixels(&mut self, Coordinate {x, y}: Coordinate, value: RGBA) {
        if x > self.width || y > self.height { return; }
        let i: usize = self.get_index(x, y);
        self.pixels[i] = value;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

impl Vec2 {
        pub fn calc_vector(a: Coordinate, b: Coordinate) -> Vec2 {
        Vec2 { x: b.x as f32 - a.x as f32, y:  b.y as f32 - a.y as f32}
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}