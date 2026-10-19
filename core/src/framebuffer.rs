use crate::coordinate::Coordinate;
use crate::rgba::RGBA;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<RGBA>,
}


impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, pixels: vec![RGBA{r: 0, g: 0, b: 0, a: 0}; width * height ]}
    }

    pub fn get_pixel(&self, Coordinate {x, y, ..}: Coordinate) -> RGBA {
        self.pixels[self.get_index(x, y).unwrap()]
    }

    pub fn set_pixels(&mut self, Coordinate {x, y, ..}: Coordinate, value: RGBA) {
        if let Some(i) = self.get_index(x, y) {
            self.pixels[i] = value;
        };
    }

    pub fn set_pixels_unchecked(&mut self, Coordinate {x, y, ..}: Coordinate, value: RGBA) {
        self.pixels[self.width * y as usize + x as usize] = value;
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < self.width as i32 && x >= 0 && y < self.height as i32 && y >= 0 {
            return Some(self.width * y as usize + x as usize)
        }
        None
    }
}
