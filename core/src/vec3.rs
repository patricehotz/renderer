use std::ops::{Div, Mul};
use crate::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {x, y, z}
    }

    pub fn calc_vector(a: Coordinate, b: Coordinate) -> Vec3 {
        Vec3 { x: (b.x  - a.x) as f32, y:  (b.y - a.y) as f32, z: (b.z - a.z) as f32 }
    }

    // pub fn calc_length(&self) -> i32 {
    //     ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt().round() as i32
    // }
    //
    // pub fn c_product(&self, b: Vec2) -> u32 {
    //     (self.x * b.y - self.y * b.x).abs() as u32
    // }
    //
    // pub fn signed_c_product(&self, b: Vec2) -> i32 {
    //     self.x * b.y - self.y * b.x
    // }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3{x: self.x * rhs as f32, y: self.y * rhs as f32, z: self.z * rhs as f32}
    }
}


impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{x: self as f32 * rhs.x, y: self as f32 * rhs.y, z: self as f32 * rhs.z}
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3{x: rhs * self.x, y: rhs * self.y, z: rhs * self.z}
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3{x: self / rhs.x, y: self / rhs.y, z: self / rhs.z }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3{x: self.x / rhs, y: self.y / rhs , z: self.z / rhs }
    }
}