use crate::coordinate::Coordinate;

pub struct BBox {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl BBox {
    pub fn new (a: Coordinate, b: Coordinate, c: Coordinate) -> Self {
        Self {min: Coordinate::new( a.x.min(b.x.min(c.x)), a.y.min(b.y.min(c.y)), 1.0), max: Coordinate::new (a.x.max(b.x.max(c.x)), a.y.max(b.y.max(c.y)), 1.0)}
    }
}


