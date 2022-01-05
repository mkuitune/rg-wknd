#![allow(dead_code)]
use std::iter::OnceWith;
use std::ops::{Add, Sub, Mul, Div};
use num::{NumCast, cast};

//rand

use rand::{Rng, thread_rng};

pub fn random_f64_normalized() -> f64{
    thread_rng().gen::<f64>()
}

pub fn random_f64(min:f64, max:f64) -> f64{
    thread_rng().gen_range(min .. max)
}

//vec3
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub x : f64, pub y: f64, pub z:f64
}
impl Vec3 {
    pub fn zeros() ->Vec3{
        Vec3{x:0.0, y:0.0,z:0.0}
    }
    pub fn ones() ->Vec3{
        Vec3{x:1.0, y:1.0,z:1.0}
    }
    pub fn mul_elements(a:Vec3, b:Vec3)->Vec3{
        Vec3{x:a.x * b.x, y : a.y * b.y, z:a.z * b.z}
    }
    pub fn length2(self) -> f64{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f64{
        self.length2().sqrt()
    }
    pub fn random_normalized() ->Vec3{
        Vec3{x:random_f64_normalized(), y:random_f64_normalized(), z:random_f64_normalized()}
    }
    pub fn random(min:f64, max:f64) ->Vec3{
        Vec3{x:random_f64(min, max), y:random_f64(min, max), z:random_f64(min, max)}
    }
    pub fn random_in_unit_sphere() -> Vec3{
        while true {
            let v = Vec3::random(-1.0, 1.0);
            if v.length2() < 1.0 {
                return v;
            }
        }
        return Vec3::zeros();
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
pub struct SamplingCfg{
    t_min : f64, t_max : f64
}

impl SamplingCfg{
    pub fn new(minv:f64, maxv:f64) -> SamplingCfg{SamplingCfg{t_min:minv, t_max:maxv}}
    pub fn inrange(&self, t:f64)->bool{t >= self.t_min && t <= self.t_max}
    pub fn inrange32(&self, t:f32)->bool{
        let tl = t as f64;
        tl >= self.t_min && tl <= self.t_max
    }
}

// Hittable
#[derive(Debug,Default,Copy, Clone)]
pub struct HitRecord{
    pub p : Vec3,
    pub normal : Vec3,
    pub t : f64,
    pub front_face : bool
}
impl HitRecord{
    pub fn set_face_normal(&mut self, r:&Ray3, outward_normal:Vec3){
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal} else {outward_normal * -1.0};
    }
}

pub trait HitRay{
    fn hit(&self, r:&Ray3, cfg:SamplingCfg)  -> Option<HitRecord>;
}

pub struct Sphere {
    pub center : Vec3,
    pub radius : f64
}

impl Sphere {
    pub fn new(cen:Vec3, r:f64) -> Sphere{
        Sphere{center:cen, radius:r}
    }
    
    pub fn new2<T:NumCast>(cx:T,cy:T,cz:T, r:T) -> Sphere {
        let vx = cast(cx).unwrap_or_default();
        let vy = cast(cy).unwrap_or_default();
        let vz = cast(cz).unwrap_or_default();
        let sr = cast(r).unwrap_or_default();
        Sphere{center:vec3(vx, vy,vz), radius:sr}
    }
}

impl HitRay for Sphere {
    fn hit(&self, r:&Ray3, cfg:SamplingCfg)  -> Option<HitRecord>{
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

        let mut record : HitRecord = HitRecord::default();
        record.t = root;
        record.p = r.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        Some(record)
    }
}

// hittable list
pub enum HittableObject{
    Sphere(Sphere),
    List(Vec<HittableObject>)
}

impl HittableObject{
    pub fn mk_list() ->Vec<HittableObject> {Vec::new()}
    pub fn wrap(v:Vec<HittableObject>) -> HittableObject{
        HittableObject::List(v)
    }
}

impl HitRay for HittableObject{
    fn hit(&self, r:&Ray3, mut cfg:SamplingCfg)  -> Option<HitRecord>{
        let mut res = None;
        match self {
            HittableObject::Sphere(sphere) => {sphere.hit(r, cfg)}
            HittableObject::List(objs) => {
                for obj in objs.iter() {
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
    }
}

// Color

use Vec3 as ColorRGB;
use std::fs::File;
use std::io::Write;

pub fn clampf64(x:f64, min:f64, max:f64)->f64{
    if x < min {min}
    else if x > max {max}
    else {x}
}

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

pub fn write_color_file_multi(file : &mut File, col : ColorRGB, samples_per_pixel:i32){
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = clampf64((col.x * scale).sqrt(), 0.0, 0.999);
    let g = clampf64((col.y * scale).sqrt(), 0.0, 0.999);
    let b = clampf64((col.z * scale).sqrt(), 0.0, 0.999);

    let ir = (256.0 * r) as i32;
    let ig = (256.0 * g) as i32;
    let ib = (256.0 * b) as i32;
    writeln!(file, "{} {} {}", ir, ig, ib); 
}

// constants
pub mod constants{
    pub const INFINITY_F64 : f64= f64::MAX; 
    pub const PI_F64 : f64= 3.1415926535897932385;
}

// utilities
pub fn degrees_to_radians(degrees:f64) -> f64{
   degrees * constants::PI_F64 / 180.0 
}

// Camera

pub struct Camera{
    pub origin : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub lower_left_corner : Vec3
}
impl Default for Camera {
    fn default() -> Camera{
        let aspect_ratio:f64 = 16.0 / 9.0;
        let viewport_height:f64 = 2.0;
        let viewport_width:f64 = aspect_ratio * viewport_height;
        let focal_length:f64 = 1.0; 
    
        let origin = vec3(0.0, 0.0, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical  = vec3(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - (horizontal/2.0)  - (vertical/2.0) - vec3(0.0, 0.0, focal_length);

        Camera{
            origin:origin, horizontal:horizontal, vertical:vertical, lower_left_corner:lower_left_corner
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u:f64, v:f64) -> Ray3{
        let raydir = self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin;
        Ray3::new(self.origin,raydir)
    }
}
