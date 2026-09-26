use renderer_core::{FrameBuffer, Coordinate, RGBA, export};
use renderer_core::draw::{gradiant, line};

fn main() {
    let mut buffer = FrameBuffer::new(420, 420);
    gradiant(&mut buffer, Coordinate {x: 20, y: 20}, RGBA {r: 255, g: 0, b:0, a: 100}, Coordinate {x: 400, y:400},  RGBA {r: 0, g: 0, b: 255, a: 100});
    line(&mut buffer, Coordinate {x: 20, y: 20}, Coordinate {x: 300, y: 120}, RGBA {r: 0, g: 0, b: 0, a: 100});
    export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
}
