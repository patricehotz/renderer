use std::arch::aarch64::uint8x8_t;

#[derive(Clone, Copy)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Buffer {
    width: usize,
    height: usize ,
    pixels: Vec<RGBA>,
}



impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, pixels: Vec::new()}
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> RGBA {
        self.pixels[self.get_index(x, y)]
    }

    pub fn set_pixels(&mut self, x: usize, y: usize, value: RGBA) {
        let i: usize = self.get_index(x, y);
        self.pixels[i] = value;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }


}

