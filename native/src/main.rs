use minifb::{Key, Window, WindowOptions};
use renderer_core::export::ppm;
use renderer_core::framebuffer::FrameBuffer;
use renderer_core::model::Model;

const WIDTH:usize = 700;
const HEIGHT: usize = 700;

fn main() {
    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let clean = FrameBuffer::new(WIDTH, HEIGHT);
    let mut frame_buffer_converted: Vec<u32>;

    //let mut model = Model::load("obj/african_head/african_head.obj");
    let mut model = Model::load("obj/diablo3_pose/diablo3_pose.obj");


    let mut window = Window::new("reeeenderrrreeeerr", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    while window.is_open() {
        frame_buffer_converted = frame_buffer.pixels.iter().map(|&p| (p.r as u32) << 16 | (p.g as u32) << 8 | p.b as u32).collect();
        window.update_with_buffer(
            &frame_buffer_converted,
            WIDTH,
            HEIGHT
        ).unwrap();
        if window.is_key_down(Key::Left) {
            model.rotate(5.0);
        }
        if window.is_key_down(Key::Right) {
            model.rotate(-5.0);
        }
        frame_buffer.clear();
        model.draw(&mut frame_buffer);
    }


    ppm(&frame_buffer, "depth.ppm");
}