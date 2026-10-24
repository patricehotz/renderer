use std::fs::File;
use std::io::{self, BufRead};
use crate::coordinate::Coordinate;
use crate::framebuffer::FrameBuffer;
use crate::point::Point;
use crate::rgb::RGB;
use crate::triangle::{triangle, Triangle};
use crate::vec3::Vec3;

pub struct Model {
    v: Vec<Vec3>,
    f: Vec<[usize; 3]>
}

impl Model {
    pub fn load(path: &str, width: usize, height: usize) -> Model {
        let mut model = Model {v: Vec::new(), f: Vec::new()};
        let file = File::open(path).unwrap();
        let lines = io::BufReader::new(file).lines();
        for line in lines.map_while(Result::ok) {
            if line.starts_with("v") {
                let res = line.split_whitespace().collect::<Vec<&str>>();
                let x = res[1].parse::<f32>().unwrap();
                let y = res[2].parse::<f32>().unwrap();
                let z = res[3].parse::<f32>().unwrap();
                model.v.push(Vec3::new(x, y, z))
            } else if line.starts_with("f") {
                let res = line.split_whitespace().collect::<Vec<&str>>();
                let a = res[1].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap()-1;
                let b = res[2].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap()-1;
                let c = res[3].split('/').collect::<Vec<&str>>()[0].parse::<usize>().unwrap()-1;
                model.f.push( [a,b,c])
            }
        }
        model
    }

    pub fn draw (self, buffer: &mut FrameBuffer) {
        let centre_x = buffer.width as f32 * 0.5;
        let centre_y = buffer.height as f32 * 0.5;

        for f in self.f {
            let a = Coordinate::new((self.v[f[0]].x * centre_x + centre_x).round() as i32, (self.v[f[0]].y * centre_y * -1.0 + centre_y).round() as i32, self.v[f[0]].z);
            let b = Coordinate::new((self.v[f[1]].x * centre_x + centre_x).round() as i32, (self.v[f[1]].y * centre_y * -1.0 + centre_y).round() as i32, self.v[f[1]].z);
            let c = Coordinate::new((self.v[f[2]].x * centre_x + centre_x).round() as i32, (self.v[f[2]].y * centre_y * -1.0 + centre_y).round() as i32, self.v[f[2]].z);

            triangle(
                buffer,
                Triangle::new(
                    Point::new(
                        a,
                        RGB::new_grayscale((127.5 * a.z + 175.5).round() as u8)
                    ),
                    Point::new(
                        b,
                        RGB::new_grayscale((127.5 * b.z + 175.5).round() as u8)
                    ),
                    Point::new(
                        c,
                        RGB::new_grayscale((127.5 * c.z + 175.5).round() as u8)
                    )
                )
            );

        }
    }

    pub fn rotate (&mut self, alpha: f32) {
        for p in &mut self.v {
            let sin =  alpha.to_radians().sin();
            let cos =  alpha.to_radians().cos();
            let x = p.z * -sin + p.x * cos;
            let z = p.x * sin + p.z * cos;
            p.z = z;
            p.x = x;
        }
    }
}
