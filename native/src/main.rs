use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use renderer_core::coordinate::Coordinate;
use renderer_core::export::ppm;
use renderer_core::framebuffer::FrameBuffer;
use renderer_core::model::Model;
use renderer_core::point::Point;
use renderer_core::rgb::{BLACK, RGB, WHITE};
use renderer_core::triangle::{triangle, Triangle};

const WIDTH:usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let mut depth_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let mut model = Model::load("/Users/photz/repos/renderer/obj/diablo3_pose/diablo3_pose.obj", WIDTH, HEIGHT);
    // let mut model = Model::load("/Users/photz/repos/renderer/obj/african_head/african_head.obj", WIDTH, HEIGHT);
    // model.rotate(237.0);
    model.draw(&mut depth_buffer);

    ppm(&depth_buffer, "/Users/photz/repos/renderer/depth.ppm");
}