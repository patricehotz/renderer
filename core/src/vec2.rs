use std::ops::Mul;
use crate::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn calc_vector(a: Coordinate, b: Coordinate) -> Vec2 {
        Vec2 { x: b.x  - a.x, y:  b.y - a.y }
    }

    pub fn calc_length(&self) -> i32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt().round() as i32
    }

    pub fn c_product(&self, b: Vec2) -> u32 {
        (self.x * b.y - self.y * b.x).abs() as u32
    }

    pub fn signed_c_product(&self, b: Vec2) -> i32 {
        self.x * b.y - self.y * b.x
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

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2{x: (rhs * self.x as f32).round() as i32, y: (rhs * self.y as f32).round() as i32}
    }
}
