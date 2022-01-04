#![allow(dead_code)]
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub x : f64, pub y: f64, pub z:f64
}
impl Vec3 {
    fn length2(self) -> f64{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(self) -> f64{
        self.length2().sqrt()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t:f64) -> Vec3 {
            Vec3 {x: self.x * t , y: self.y * t, z: self.z * t}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, t:Vec3) -> f64 {
            self.x * t.x + self.y * t.y + self.z * t.z
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t:f64) -> Vec3 {
            Vec3 {x: self.x / t , y: self.y / t, z: self.z / t}
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

pub fn vec3(x : f64, y:f64, z:f64)->Vec3 {
    Vec3{x:x, y:y, z:z}
}

pub fn unit_vector(v:Vec3) -> Vec3{
    let il = 1.0 / v.length();
    v * il
}

pub fn lerp3(a:Vec3, b:Vec3, t : f64) -> Vec3{
    let u = 1.0 - t;
    (a * u) + (b * t)
}

pub fn dot(a:Vec3, b:Vec3) -> f64{a * b}

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
    pub fn at(&self, t:f64) -> Vec3 {
        self.orig + (self.dir * t)
    }
    pub fn new(origin:Vec3, direction:Vec3) -> Ray3{
        Ray3{orig: origin, dir: direction}
    }
}

pub fn hit_sphere_OLD(center:Vec3, radius:f64, r:&Ray3) -> f64{
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

pub fn hit_sphere(center:Vec3, radius:f64, r:&Ray3) -> f64{
    let oc = r.origin() - center;
    let a = r.dir.length2();
    let half_b = dot(oc, r.dir);
    let c = oc.length2() - radius*radius;
    let discrm = half_b * half_b - a * c;
    if discrm < 0.0{
        -1.0
    }else{
        (-half_b - discrm.sqrt())/ a
    }

}

// sampling cfg
#[derive(Debug, Copy, Clone)]
struct sampling_cfg{
    t_min : f64, t_max : f64
}

impl sampling_cfg{
    pub fn inrange(&self, t:f64)->bool{t >= self.t_min && t <= self.t_max}
    pub fn inrange32(&self, t:f32)->bool{
        let tl = t as f64;
        tl >= self.t_min && tl <= self.t_max
    }
}

// Hittable
#[derive(Debug,Default,Copy, Clone)]
struct hit_record{
    pub p : Vec3,
    pub normal : Vec3,
    pub t : f64,
    pub front_face : bool
}
impl hit_record{
    pub fn set_face_normal(&mut self, r:&Ray3, outward_normal:Vec3){
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal} else {outward_normal * -1.0};
    }
}

trait hittable{
    fn hit(&self, r:&Ray3, cfg:sampling_cfg)  -> Option<hit_record>;
}

struct sphere {
    center : Vec3,
    radius : f64
}

impl sphere {
    pub fn new(cen:Vec3, r:f64) -> sphere{
        sphere{center:cen, radius:r}
    }
}

impl hittable for sphere {
    fn hit(&self, r:&Ray3, cfg:sampling_cfg)  -> Option<hit_record>{
        let oc = r.origin() - self.center;
        let a = r.dir.length2();
        let half_b = dot(oc, r.dir);
        let r2 = self.radius.powf(2.0);
        let c = oc.length2() - r2;
        let discrm = half_b * half_b - a * c;

        if discrm < 0.0 {
            return None;
        }

        let sqrtd = discrm.sqrt();
        let rootmin = (-half_b - sqrtd) / a;
        let mut root = rootmin;
        if ! cfg.inrange(rootmin) {
            let rootmax = (-half_b + sqrtd) / a;
            root = rootmax;
            if ! cfg.inrange(rootmax){
                return None;
            }
        }

        let mut record : hit_record = hit_record::default();
        record.t = root;
        record.p = r.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        Some(record)
    }
}

// hittable list
struct hittable_list{
    pub objects : Vec<Box<dyn hittable>>
}

impl hittable for hittable_list{

    fn hit(&self, r:&Ray3, mut cfg:sampling_cfg)  -> Option<hit_record>{
        let mut res = None;
        for obj in &self.objects {
            let hitresult = obj.hit(r, cfg);
            match hitresult {
                Some(hit) => {
                    cfg.t_max = hit.t;
                    res = hitresult;
                },
                None => {}
            }
        }
        res
    }
}

// Color

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

// constants
mod constants{
    pub const infinity_f64 : f64= f64::MAX;
    pub const pi_f64 : f64= 3.1415926535897932385;
}

// utilities
pub fn degrees_to_radians(degrees:f64) -> f64{
   degrees * constants::pi_f64 / 180.0 
}
