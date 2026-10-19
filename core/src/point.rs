use crate::coordinate::Coordinate;
use crate::rgba::RGBA;

pub struct Point {
    pub coordinate: Coordinate,
    pub color: RGBA,
}

impl Point {
    pub fn new(coordinate: Coordinate, color: RGBA) -> Self {
        Point {coordinate, color}
    }
}
