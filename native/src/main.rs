use renderer_core::{FrameBuffer, Coordinate, RGBA, export};
use renderer_core::draw::{gradiant, line, triangle};

fn main() {
    let mut buffer = FrameBuffer::new(120, 120);
    triangle(&mut buffer, Coordinate {x: 12, y: 12}, Coordinate {x: 80, y: 30},Coordinate {x: 100, y: 80}, RGBA {r: 255, g: 0, b:0, a: 255});
    triangle(&mut buffer, Coordinate {x: 40, y: 30}, Coordinate {x: 40, y: 110},Coordinate {x: 58, y: 105}, RGBA {r: 0, g: 0, b:255, a: 255});
    triangle(&mut buffer, Coordinate {x: 88, y: 60}, Coordinate {x: 64, y: 119},Coordinate {x: 110, y: 100}, RGBA {r: 0, g: 255, b:0, a: 255});
    //export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
    export::ppm(&buffer, "/Users/photz/repos/renderer/gradient.ppm");
}

fn optimisations_test() {
    let width: i32 = 3840;
    let height: i32 = 1600;
    let mut buffer = FrameBuffer::new(width as usize, height as usize);
    let mut x: i32 = 1;
    const A: i32 = 69;
    const C: i32 = 33;

    for _i in 0..(10 as u32).pow(7){
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let ax = x.abs() % width  ;
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let ay = x.abs() % height ;
        let a = Coordinate {x: ax, y: ay};

        x = x.wrapping_mul(A).wrapping_add(C) ;
        let bx = x.abs() % width;
        x = x.wrapping_mul(A).wrapping_add(C) ;
        let by = x.abs() % height;
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
    //export::ppm(&buffer, "/Users/photz/repos/renderer/gradient.ppm");
    export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
}
