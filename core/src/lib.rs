pub mod export;
pub mod draw;

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

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
        let i: usize = self.get_index(x, y);
        self.pixels[i] = value;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}
