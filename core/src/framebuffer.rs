use std::fmt::Debug;
use crate::coordinate::Coordinate;
use crate::rgb::RGB;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<RGB>,
    pub depth: Vec<f32>
}


impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, pixels: vec![RGB {r: 0, g: 0, b: 0}; width * height ], depth: vec![-1.0; width * height]}
    }

    pub fn get_pixel(&self, Coordinate {x, y, ..}: Coordinate) -> RGB {
        self.pixels[self.get_index(x, y).unwrap()]
    }

    pub fn get_depth(&self, Coordinate {x, y, ..}: Coordinate) -> f32 {
            match self.get_index(x, y) {
                None => -1.0,
                Some(i) => self.depth[i]
            }
    }

    pub fn set_pixels(&mut self, Coordinate {x, y, ..}: Coordinate, value: RGB) {
        if let Some(i) = self.get_index(x, y) {
            self.pixels[i] = value;
        };
    }

    pub fn set_depth(&mut self, Coordinate {x, y, ..}: Coordinate, value: f32) {
        if let Some(i) = self.get_index(x, y) {
            self.depth[i] = value;
        };
    }

    pub fn set_pixels_unchecked(&mut self, Coordinate {x, y, ..}: Coordinate, value: RGB) {
        self.pixels[self.width * y as usize + x as usize] = value;
    }

    pub fn set_depth_unchecked(&mut self, Coordinate {x, y, ..}: Coordinate, value: f32) {
        self.depth[self.width * y as usize + x as usize] = value;
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < self.width as i32 && x >= 0 && y < self.height as i32 && y >= 0 {
            return Some(self.width * y as usize + x as usize)
        }
        None
    }
}
