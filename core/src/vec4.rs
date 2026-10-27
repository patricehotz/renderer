use std::ops::{Add, Div, Index, Mul};
use crate::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy)]
pub struct Matrix4(pub [Vec4; 4]);

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 {x, y, z, w}
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vec4;
    fn index(&self, i: usize) -> &Vec4 { &self.0[i] }
}

impl Mul<Vec4> for Vec4 {
    type Output = f32;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Mul<i32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec4 {x: self.x * rhs as f32, y: self.y * rhs as f32, z: self.z * rhs as f32, w: rhs as f32 * self.w}
    }
}


impl Mul<Vec4> for i32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4 {x: self as f32 * rhs.x, y: self as f32 * rhs.y, z: self as f32 * rhs.z, w: self as f32 * rhs.w}
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4 {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec4 {x: rhs * self.x, y: rhs * self.y, z: rhs * self.z, w: rhs * self.w}
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        Vec4 {x: self / rhs.x, y: self / rhs.y, z: self / rhs.z, w: self / rhs.w}
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        Vec4 {x: self.x / rhs, y: self.y / rhs , z: self.z / rhs, w: self.w / rhs}
    }
}


impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Mul<Vec4> for Matrix4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self[0] * rhs.x + self[1] * rhs.y + self[2] * rhs.z + self[3] * rhs.w
    }
}

impl Mul<Matrix4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        self.x * rhs[0] + self.y * rhs[1] + self.z * rhs[2] + self.w * rhs[3]
    }
}
