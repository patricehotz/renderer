use renderer_core::{FrameBuffer, Coordinate, RGBA, export};
use renderer_core::draw::{gradiant, line};

fn main() {
    let mut buffer = FrameBuffer::new(420, 420);
    gradiant(&mut buffer, Coordinate {x: 20, y: 20}, RGBA {r: 255, g: 0, b:0, a: 100}, Coordinate {x: 400, y:400},  RGBA {r: 0, g: 0, b: 255, a: 100});
    line(&mut buffer, Coordinate {x: 20, y: 20}, Coordinate {x: 300, y: 120}, RGBA {r: 0, g: 0, b: 0, a: 100});
    //export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
    export::ppm(&buffer, "/Users/photz/repos/renderer/gradient.ppm");
}

fn optimisations_test() {
    let mut buffer = FrameBuffer::new(400, 400);
    let mut x: i32 = 1;
    const A: i32 = 69;
    const C: i32 = 33;

    for _i in 0..(10 as u32).pow(4){
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let ax = x % 401;
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let ay = x % 401;
        let a = Coordinate {x: ax, y: ay};

        x = x.wrapping_mul(A).wrapping_add(C) ;
        let bx = x % 401;
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let by = x % 401;
        let b = Coordinate {x: bx, y: by};

        x = x.wrapping_mul(A).wrapping_add(C) ;
        let r = x as u8;
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let g = x as u8;
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let _b = x as u8;
        let color = RGBA {r, g, b: _b, a: 100};

        line(&mut buffer, a, b, color);
    }
    export::ppm(&buffer, "/Users/photz/repos/renderer/gradient.ppm");
}
