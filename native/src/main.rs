use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use renderer_core::{FrameBuffer, Coordinate, RGBA, export};
use renderer_core::draw::{gradiant, line, triangle, Point, Triangle};

const WIDTH:usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut buffer = FrameBuffer::new(WIDTH, HEIGHT);
    // let mut points = Vec::new();
    // let centre_x = WIDTH as f32 * 0.5;
    // let centre_y = HEIGHT as f32 * 0.5;
    //
    // let file = File::open("/Users/photz/repos/renderer/obj/african_head/african_head.obj").unwrap();
    // let lines = io::BufReader::new(file).lines();
    // for line in lines.map_while(Result::ok) {
    //     if line.starts_with("v") {
    //         let res = line.split_whitespace().collect::<Vec<&str>>();
    //         let x = (res[1].parse::<f32>().unwrap() * centre_x).round() as i32 + centre_x as i32 ;
    //         let y = (res[2].parse::<f32>().unwrap() * centre_y).round() as i32 * -1 + centre_y as i32;
    //         points.push(Coordinate {x, y})
    //     }
    //     else if line.starts_with("f") {
    //         let res = line.split_whitespace().collect::<Vec<&str>>();
    //         let a = res[1].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    //         let b = res[2].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    //         let c = res[3].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    //         triangle(&mut buffer, points[a-1], points[b-1], points[c-1], RGBA {r: rand::random::<u8>(), g: rand::random::<u8>(), b: rand::random::<u8>(), a: 0})
    //     }
    // }

    let rgb_triangle = Triangle {
        a: Point {
            coordinate: Coordinate {
                x: 250,
                y: 10,
            },
            color: RGBA {
                r: 255,
                g: 0,
                b: 0,
                a: 100
            }
        },
        b: Point {
            coordinate: Coordinate {
                x: 10,
                y: 490,
            },
            color: RGBA {
                r: 0,
                g: 255,
                b: 0,
                a: 100
            }
        },
        c: Point {
            coordinate: Coordinate {
                x: 490,
                y: 490,
            },
            color: RGBA {
                r: 0,
                g: 0,
                b: 255,
                a: 100
            }
        },
    };

    triangle(&mut buffer, rgb_triangle);
        //export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
    export::ppm(&buffer, "/Users/photz/repos/renderer/gradient.ppm");
}