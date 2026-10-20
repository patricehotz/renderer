use std::ops::Add;
use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: f32
}

impl Coordinate {
    pub fn new (x: i32, y: i32, z: f32) -> Self {
        Coordinate { x, y, z }
    }
}

impl Add<Vec2> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Vec2) -> Self::Output {
        Coordinate::new(self.x + rhs.x as i32, self.y + rhs.y, 1.0)
    }
}
