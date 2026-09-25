use renderer_core::{Buffer, Coordinate, RGBA, export};
use renderer_core::draw::gradiant;

fn main() {
    let mut buffer = Buffer::new(420, 420);
    gradiant(&mut buffer, Coordinate {x: 20, y: 20}, RGBA {r: 255, g: 0, b:0, a: 100}, Coordinate {x: 400, y:400},  RGBA {r: 255, g: 0, b: 255, a: 100});
    export::ppm(&buffer, "/Users/photz/repos/renderer/gradient.ppm");
}
