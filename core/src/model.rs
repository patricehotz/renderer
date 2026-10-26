use std::fs::File;
use std::io::{self, BufRead};
use crate::coordinate::Coordinate;
use crate::framebuffer::FrameBuffer;
use crate::point::Point;
use crate::rgb::RGB;
use crate::triangle::{triangle, Triangle};
use crate::vec3::{Matrix3, Vec3};

pub struct Model {
    v: Vec<Vec3>,
    f: Vec<[usize; 3]>
}

impl Model {
    pub fn load(path: &str) -> Model {
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

    pub fn draw (&mut self, buffer: &mut FrameBuffer) {
        let centre_x = buffer.width as f32 * 0.5;
        let centre_y = buffer.height as f32 * 0.5;

        for f in &self.f {
            let va = pers(self.v[f[0]]);
            let vb = pers(self.v[f[1]]);
            let vc = pers(self.v[f[2]]);
            let a = Coordinate::new((va.x * centre_x + centre_x).round() as i32, (va.y * centre_y * -1.0 + centre_y).round() as i32, va.z);
            let b = Coordinate::new((vb.x * centre_x + centre_x).round() as i32, (vb.y * centre_y * -1.0 + centre_y).round() as i32, vb.z);
            let c = Coordinate::new((vc.x * centre_x + centre_x).round() as i32, (vc.y * centre_y * -1.0 + centre_y).round() as i32, vc.z);

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

            let rotation: Matrix3 = Matrix3([
                Vec3::new(cos, 0.0, sin),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-sin, 0.0, cos)
            ]);

            *p = *p * rotation;
        }
    }


    pub fn clear(&mut self) {
        self.f = Vec::new();
    }

}

pub fn pers(v: Vec3) -> Vec3{
    const C: f32 = 3.0;
    v/(1.0-v.z/C)
}
