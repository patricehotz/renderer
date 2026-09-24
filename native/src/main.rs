use renderer_core::{Buffer, Coordinate};
use renderer_core::draw::gradiant;

fn main() {
    let buffer = Buffer::new(420, 420);
    gradiant(&buffer, Coordinate {x: 20, y: 20}, Coordinate {x: 400, y:400})
}
