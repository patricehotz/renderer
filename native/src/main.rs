use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use renderer_core::coordinate::Coordinate;
use renderer_core::export::ppm;
use renderer_core::framebuffer::FrameBuffer;
use renderer_core::point::Point;
use renderer_core::rgba::RGBA;
use renderer_core::triangle::{triangle, Triangle};

const WIDTH:usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let mut depth_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let mut points = Vec::new();
    let centre_x = WIDTH as f32 * 0.5;
    let centre_y = HEIGHT as f32 * 0.5;

    let file = File::open("/Users/photz/repos/renderer/obj/african_head/african_head.obj").unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines.map_while(Result::ok) {
        if line.starts_with("v") {
            let res = line.split_whitespace().collect::<Vec<&str>>();
            let x = (res[1].parse::<f32>().unwrap() * centre_x).round() as i32 + centre_x as i32 ;
            let y = (res[2].parse::<f32>().unwrap() * centre_y).round() as i32 * -1 + centre_y as i32;
            points.push(Coordinate::new(x, y, 1))
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
                       RGBA::new(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 0)
                   ),
                   Point::new(
                       points[b-1],
                       RGBA::new (rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 0)
                   ),
                   Point::new(
                      points[c-1],
                       RGBA::new (rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 0)
                   )

               ))
        }
    }

    //export::ppm(&buffer, "/home/paho/git/renderer/gradient.ppm");
    ppm(&frame_buffer, "/Users/photz/repos/renderer/gradient.ppm");
}