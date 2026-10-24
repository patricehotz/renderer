use renderer_core::export::ppm;
use renderer_core::framebuffer::FrameBuffer;
use renderer_core::model::Model;

const WIDTH:usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let mut depth_buffer = FrameBuffer::new(WIDTH, HEIGHT);
    //let mut model = Model::load("obj/african_head/african_head.obj");
    let mut model = Model::load("obj/diablo3_pose/diablo3_pose.obj");
    model.rotate(330.0);
    model.pers();
    model.draw(&mut depth_buffer);

    ppm(&depth_buffer, "depth.ppm");
}