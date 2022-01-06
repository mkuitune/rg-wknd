#![allow(dead_code)]
use std::iter::OnceWith;
use std::ops::{Add, Sub, Mul, Div, Deref};
use num::traits::Pow;
use num::{NumCast, cast};
use std::{rc::Rc, cmp, io::BufWriter};
use ordered_float::OrderedFloat;
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

pub fn vec3(x : f64, y:f64, z:f64)->Vec3 {
    Vec3{x:x, y:y, z:z}
}

pub fn unit_vector(v:Vec3) -> Vec3{
    let il = 1.0 / v.length();
    v * il
}

impl Vec3 {
    pub fn reflect(&self, n:Vec3) -> Vec3{
        *self - (n*(*self * n)*2.0)
    }
    pub fn near_zero(&self)->bool{
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn zeros() ->Vec3{
        Vec3{x:0.0, y:0.0,z:0.0}
    }

    pub fn ones() ->Vec3{
        Vec3{x:1.0, y:1.0,z:1.0}
    }

    pub fn mul_elements(&self, b:Vec3)->Vec3{
        Vec3{x:self.x * b.x, y : self.y * b.y, z:self.z * b.z}
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

    pub fn random_in_hemisphere(normal:Vec3) -> Vec3{
        let v = Vec3::random_in_unit_sphere();
        if normal * v > 0.0 {v}
        else {v * (-1.0)}
    }

    pub fn random_unit_vector()->Vec3{
        unit_vector(Vec3::random_in_unit_sphere())        
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


pub fn lerp3(a:Vec3, b:Vec3, t : f64) -> Vec3{
    let u = 1.0 - t;
    (a * u) + (b * t)
}

pub fn dot(a:Vec3, b:Vec3) -> f64{a * b}

pub fn minf(a:f64, b:f64) -> f64{
    //*cmp::min(OrderedFloat(a), OrderedFloat(b)).deref()
    if a < b {a} else {b}
}

pub fn maxf(a:f64, b:f64) -> f64{
    if a > b {a} else {b}
}

pub fn refract(uv:Vec3, n:Vec3, etai_over_etat:f64) -> Vec3{
    let cos_theta = minf(-1.0 *dot(uv, n), 1.0);
    let r_out_perp =  (uv + n * cos_theta)* etai_over_etat;
    let parn = (1.0 - r_out_perp.length2()).abs().sqrt() * (-1.0);
    let r_out_parallel = n * parn;
    r_out_perp + r_out_parallel
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

// Material
pub struct ScatterResult{
    pub attenuation :Vec3,
    pub scattered : Ray3
}

#[derive(Debug)]
struct Lambertian{albedo:Vec3}
impl Lambertian{
    fn scatter(&self, rec:HitRecord) -> ScatterResult{
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        ScatterResult{attenuation:self.albedo, scattered : Ray3::new(rec.p, scatter_direction)}
    }
}
#[derive(Debug)]
struct Metal{
    albedo:Vec3,
    fuzz:f64
}
impl Metal{
    pub fn new(albedo:Vec3, f:f64) -> Metal {
        let fuzz = if f < 1.0 {f} else{1.0};
        Metal{albedo:albedo, fuzz:fuzz}
    }
    fn scatter(&self, r_in:Ray3,rec:HitRecord) -> Option<ScatterResult>{
        let reflected = unit_vector(r_in.dir).reflect(rec.normal);
        let scattered = Ray3::new(rec.p, reflected + (Vec3::random_in_unit_sphere() * self.fuzz));

        if scattered.dir * rec.normal > 0.0
        {
            let res = ScatterResult{attenuation:self.albedo, scattered : scattered};
            Some(res)
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric{
    ir : f64
}

impl Dielectric{
    pub fn new(ir:f64) ->Dielectric{Dielectric{ir:ir}}
    // Schlick reflectance
    fn reflectance(cosine:f64, ref_idx:f64) -> f64{
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx).powf(2.0);
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
    fn scatter(&self, r_in:Ray3,rec:HitRecord) -> Option<ScatterResult>{
        let attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0 / self.ir} else {self.ir};

        let unit_direction = unit_vector(r_in.dir);
        let cos_theta = minf(dot(unit_direction * (-1.0), rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta); 
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let this_ray_reflects = Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64_normalized();
        let direction = if cannot_refract || this_ray_reflects {unit_direction.reflect(rec.normal)} else{refract(unit_direction, rec.normal, refraction_ratio)};
        let scattered = Ray3::new(rec.p, direction);
        Some(ScatterResult{attenuation:attenuation, scattered:scattered})
    }
}

#[derive(Debug)]
pub enum Material{
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

impl Material{
    pub fn mk_lambert(albedo:Vec3)->Material{Material::Lambertian(Lambertian{albedo:albedo})}
    pub fn mk_metal(albedo:Vec3, fuzz:f64)->Material{Material::Metal(Metal::new(albedo, fuzz))}
    pub fn mk_dielectric(ir:f64)->Material{Material::Dielectric(Dielectric::new(ir))}

    pub fn scatter(&self, r_in:Ray3, rec:HitRecord) ->Option<ScatterResult>{
        match self {
            Material::Lambertian(lamb) =>{
                Some(lamb.scatter(rec))
            }
            Material::Metal(metal) =>{
                metal.scatter(r_in, rec)
            }
            Material::Dielectric(dielectric) =>{
                dielectric.scatter(r_in, rec)
            }
            _ => None
        }
    }
}

pub struct MaterialCollection{
    pub materials:Vec<Material>
}
pub type MaterialId = usize;
impl MaterialCollection{
    pub fn new()->MaterialCollection{MaterialCollection{materials:vec![]}}
    pub fn add(&mut self, mat:Material)->MaterialId{
        self.materials.push(mat);
        self.materials.len() - 1
    }
}

// Hittable
//#[derive(Debug,Default,Copy, Clone)]
#[derive(Debug,Default, Copy, Clone)]
pub struct HitRecord{
    pub p : Vec3,
    pub normal : Vec3,
    pub mat : MaterialId,
    pub t : f64,
    pub front_face : bool
}
impl HitRecord{
    pub fn new_default(mat:MaterialId)->HitRecord{
        HitRecord{p:Vec3::zeros(), normal:Vec3::zeros(), mat:mat, t:0.0, front_face:false}
    }
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
    pub radius : f64,
    pub material : MaterialId
}

impl Sphere {
    pub fn new(cen:Vec3, r:f64, mat:MaterialId) -> Sphere{
        Sphere{center:cen, radius:r, material:mat}
    }
    
    pub fn new2<T:NumCast>(cx:T,cy:T,cz:T, r:T, mat:MaterialId) -> Sphere {
        let vx = cast(cx).unwrap_or_default();
        let vy = cast(cy).unwrap_or_default();
        let vz = cast(cz).unwrap_or_default();
        let sr = cast(r).unwrap_or_default();
        Sphere{center:vec3(vx, vy,vz), radius:sr, material:mat}
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

        let mut record : HitRecord = HitRecord::new_default(self.material);
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

pub fn write_color_file_vec(file : &mut File,w:usize, h:usize, pixels : Vec<i32>){
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", w, h);
    for cl in (0 .. pixels.len()).step_by(3) {
        writeln!(writer, "{} {} {}", pixels[cl],pixels[cl+1], pixels[cl+2]); 
    }

//     writeln!(file, "P3\n{} {}\n255", w, h);
//     for cl in (0 .. pixels.len()).step_by(3) {
//         writeln!(file, "{} {} {}", pixels[cl],pixels[cl+1], pixels[cl+2]); 
//     }
}

//pub fn write_color_to_buf(pixels : &mut Vec<i32>, idx:usize,col : ColorRGB, samples_per_pixel:i32){
pub fn write_color_to_buf(pixels : &mut [i32], idx:usize,col : ColorRGB, samples_per_pixel:i32){
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = clampf64((col.x * scale).sqrt(), 0.0, 0.999);
    let g = clampf64((col.y * scale).sqrt(), 0.0, 0.999);
    let b = clampf64((col.z * scale).sqrt(), 0.0, 0.999);

    let ir = (256.0 * r) as i32;
    let ig = (256.0 * g) as i32;
    let ib = (256.0 * b) as i32;
    pixels[idx*3] = ir;
    pixels[idx*3 + 1] = ig;
    pixels[idx*3 + 2] = ib;
    //writeln!(file, "{} {} {}", ir, ig, ib); 
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
    pub fn new(vfov:f64, aspect_ratio:f64)->Camera{
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Vec3::zeros();
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical  = vec3(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - (horizontal/2.0)  - (vertical/2.0) - vec3(0.0, 0.0, focal_length);
        Camera{origin:origin, horizontal:horizontal, vertical:vertical, lower_left_corner:lower_left_corner}
    }

    pub fn get_ray(&self, u:f64, v:f64) -> Ray3{
        let raydir = self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin;
        Ray3::new(self.origin,raydir)
    }
}