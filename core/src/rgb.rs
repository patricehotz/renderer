use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const BLACK: RGB = RGB {r: 0, g: 0, b: 0};
pub const WHITE: RGB = RGB {r: 255, g: 255, b: 255};

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    pub fn new_grayscale(a: u8) -> Self {
        RGB::new(a, a, a)
    }
}


impl Add<RGB> for u8 {
    type Output = RGB;

    fn add(self, rhs: RGB) -> Self::Output {
        RGB::new(
            rhs.r + self,
            rhs.g + self,
            rhs.b + self,
        )
    }
}

impl Add<u8> for RGB {
    type Output = RGB;

    fn add(self, rhs: u8) -> Self::Output {
        RGB::new(
            rhs + self.r,
            rhs + self.g,
            rhs + self.b,
        )
    }
}

impl Mul<RGB> for f32 {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> Self::Output {
        RGB::new(
            (rhs.r as f32 * self).round() as u8,
            (rhs.g as f32 * self).round() as u8,
            (rhs.b as f32 * self).round() as u8,
        )
    }
}

impl Mul<f32> for RGB {
    type Output = RGB;

    fn mul(self, rhs: f32) -> Self::Output {
        RGB::new(
            (self.r as f32 * rhs).round() as u8,
            (self.g as f32 * rhs).round() as u8,
            (self.b as f32 * rhs).round() as u8,
        )
    }
}

impl Div<RGB> for f32 {
    type Output = RGB;

    fn div(self, rhs: RGB) -> Self::Output {
        RGB::new(
            (rhs.r as f32 / self).round() as u8,
            (rhs.g as f32 / self).round() as u8,
            (rhs.b as f32 / self).round() as u8,
        )
    }
}

impl Div<f32> for RGB {
    type Output = RGB;

    fn div(self, rhs: f32) -> Self::Output {
        RGB::new(
            (self.r as f32 / rhs).round() as u8,
            (self.g as f32 / rhs).round() as u8,
            (self.b as f32 / rhs).round() as u8,
        )
    }
}
