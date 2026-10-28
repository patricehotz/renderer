use std::fs::File;
use std::io::{self, BufRead};
use crate::coordinate::Coordinate;
use crate::framebuffer::FrameBuffer;
use crate::point::Point;
use crate::rgb::RGB;
use crate::triangle::{triangle, Triangle};
use crate::vec3::{Matrix3, Vec3};
use crate::vec4::{Matrix4, Vec4};

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
        let h = buffer.height as f32;
        let w = buffer.height as f32;

        let viewport = viewport(0., 0., w, h);

        for f in &self.f {
            let va = viewport * pers(self.v[f[0]]).extend(1.);
            let vb = viewport * pers(self.v[f[1]]).extend(1.);
            let vc = viewport * pers(self.v[f[2]]).extend(1.);

            triangle(
                buffer,
                Triangle::new(
                    Point::new(
                        Coordinate::new(va.x.round() as i32, va.y.round() as i32, va.z),
                        RGB::new_grayscale(va.z.round() as u8)
                    ),
                    Point::new(
                        Coordinate::new(vb.x.round() as i32, vb.y.round() as i32, vb.z),
                        RGB::new_grayscale(vb.z.round() as u8)
                    ),
                    Point::new(
                        Coordinate::new(vc.x.round() as i32, vc.y.round() as i32, vc.z),
                        RGB::new_grayscale( vc.z.round() as u8)
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

pub fn viewport(x: f32, y: f32, w: f32, h: f32) -> Matrix4 {
     Matrix4([
        Vec4::new(w * 0.5, 0., 0., 0.),
        Vec4::new(0., -h * 0.5, 0., 0.),
        Vec4::new(0., 0., 127.5, 0.),
        Vec4::new(x + w * 0.5, y + w * 0.5, 127.5, 1.)
    ])
}