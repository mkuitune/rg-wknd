#![allow(dead_code)]
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x : f32, pub y: f32, pub z:f32
}
impl Vec3 {
    fn length2(self) -> f32{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(self) -> f32{
        self.length2().sqrt()
    }
}


impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t:f32) -> Vec3 {
            Vec3 {x: self.x * t , y: self.y * t, z: self.z * t}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;
    fn mul(self, t:Vec3) -> f32 {
            self.x * t.x + self.y * t.y + self.z * t.z
    }
}

//impl Mul<Vec3> for f32{
//    type Output = Vec3;
//    fn mul(self, t:Vec3) -> Vec3 {
//            Vec3{x : self * t.x, y: self * t.y, z:self * t.z
//    }
//}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other:Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other:Self) -> Self {
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}


pub fn vec3(x : f32, y:f32, z:f32)->Vec3 {
    Vec3{x:x, y:y, z:z}
}

pub fn unit_vector(v:Vec3) -> Vec3{
    let il = 1.0 / v.length();
    v * il
}

pub fn lerp3(a:Vec3, b:Vec3, t : f32) -> Vec3{
    let u = 1.0 - t;
    (a * u) + (b * t)
}

//
// Ray3
//
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray3{
    pub orig : Vec3, pub dir : Vec3
}

impl Ray3{
    pub fn origin(&self) -> Vec3 {self.orig} 
    pub fn direction(&self) -> Vec3 {self.dir} 
    pub fn at(&self, t:f32) -> Vec3 {
        self.orig + (self.dir * t)
    }
    pub fn new(origin:Vec3, direction:Vec3) -> Ray3{
        Ray3{orig: origin, dir: direction}
    }
}

pub fn hit_sphere(center:Vec3, radius:f32, r:&Ray3) -> f32{
    let oc = r.origin() - center;
    let dir = r.direction();
    let a = dir * dir;
    let b = 2.0 * (oc * dir);
    let c = (oc * oc) - radius*radius;
    let discrm = b * b - 4.0 * a * c;
    if discrm < 0.0{
        -1.0
    }else{
        (-b - discrm.sqrt())/ (2.0 * a)
    }

}

use Vec3 as ColorRGB;
use std::fs::File;
use std::io::Write;

pub fn write_color_stdout(col : ColorRGB){
    let ir = (255.99 * col.x) as i32;
    let ig = (255.99 * col.y) as i32;
    let ib = (255.99 * col.z) as i32;
    println!("{} {} {}", ir, ig, ib); 
}

pub fn write_color_file(file : &mut File, col : ColorRGB){
    let ir = (255.99 * col.x) as i32;
    let ig = (255.99 * col.y) as i32;
    let ib = (255.99 * col.z) as i32;
    writeln!(file, "{} {} {}", ir, ig, ib); 
}