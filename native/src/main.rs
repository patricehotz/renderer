use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use renderer_core::coordinate::Coordinate;
use renderer_core::export::ppm;
use renderer_core::framebuffer::FrameBuffer;
use renderer_core::point::Point;
use renderer_core::rgb::{BLACK, RGB, WHITE};
use renderer_core::triangle::{triangle, Triangle};

const WIDTH:usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let mut depth_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let mut points = Vec::new();
    let centre_x = WIDTH as f32 * 0.5;
    let centre_y = HEIGHT as f32 * 0.5;

    // let file = File::open("/Users/photz/repos/renderer/obj/african_head/african_head.obj").unwrap();
    let file = File::open("/Users/photz/repos/renderer/obj/diablo3_pose/diablo3_pose.obj").unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines.map_while(Result::ok) {
        if line.starts_with("v") {
            let res = line.split_whitespace().collect::<Vec<&str>>();
            let x = (res[1].parse::<f32>().unwrap() * centre_x).round() as i32 + centre_x as i32 ;
            let y = (res[2].parse::<f32>().unwrap() * centre_y).round() as i32 * -1 + centre_y as i32;
            let z = res[3].parse::<f32>().unwrap();
            points.push(Coordinate::new(x, y, z))
        }
        else if line.starts_with("f") {
            let res = line.split_whitespace().collect::<Vec<&str>>();
            let a = res[1].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            let b = res[2].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            let c = res[3].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            triangle(
                &mut frame_buffer,
                Triangle::new(
                   Point::new(
                       points[a-1],
                       RGB::new(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>())
                   ),
                   Point::new(
                       points[b-1],
                       RGB::new (rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>())
                   ),
                   Point::new(
                       points[c-1],
                       RGB::new (rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>())
                   )
               )
            );
            triangle(
                &mut depth_buffer,
                Triangle::new(
                    Point::new(
                        points[a-1],
                        RGB::new_grayscale((127.5 * points[a-1].z + 175.5).round() as u8)
                    ),
                    Point::new(
                        points[b-1],
                        RGB::new_grayscale((127.5 * points[b-1].z + 175.5).round() as u8)
                    ),
                    Point::new(
                        points[c-1],
                        RGB::new_grayscale((127.5 * points[c-1].z + 175.5).round() as u8)
                    )
                )
            )
        }
    }

    //export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
    ppm(&frame_buffer, "/Users/photz/repos/renderer/gradient.ppm");
    ppm(&depth_buffer, "/Users/photz/repos/renderer/depth.ppm");
}