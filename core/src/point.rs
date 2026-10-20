use crate::coordinate::Coordinate;
use crate::rgb::RGB;

pub struct Point {
    pub coordinate: Coordinate,
    pub color: RGB,
}

impl Point {
    pub fn new(coordinate: Coordinate, color: RGB) -> Self {
        Point {coordinate, color}
    }
}
