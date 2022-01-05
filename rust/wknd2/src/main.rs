
#![allow(dead_code)]

mod vec3;

use vec3::{Vec3, Ray3, vec3, lerp3, unit_vector, 
    hit_sphere, write_color_stdout, 
    HitRecord, HittableObject, constants, SamplingCfg, Sphere, HitRay};

use std::fs::File;
use std::io::Write;

use crate::vec3::{write_color_file, Camera, random_f64_normalized, write_color_file_multi};

extern crate pbr;
use pbr::ProgressBar;

fn test_ppm(){

    let image_width = 200;
    let image_height = 100;
    println!("P3\n{} {}\n255", image_width, image_height);
    let fnx = image_width as f64;
    let fny = image_height as f64;
    for j in (0 .. image_height).rev() {
        for i in 0 .. image_width {
            let col = vec3((i as f64) / fnx, (j as f64) / fny, 0.2);
            write_color_stdout(col);
        }
    }
}

fn color2(r : Ray3) -> Vec3 {
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(Vec3{x:1.0,y:1.0,z:1.0}, Vec3{x:0.5, y:0.7, z:1.0}, t)
}

fn color3(r : &Ray3) -> Vec3 {
    if hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, &r)  > 0.0 {
        return vec3(1.0, 0.0, 0.0);        
    }
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
}

fn color4(r : &Ray3) -> Vec3 {

    let t = hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - vec3(0.0,0.0,-1.0));
        return vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;        
    }
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
}

fn ray_color_1(r : &Ray3) -> Vec3 {

    let t = hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - vec3(0.0,0.0,-1.0));
        return vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;        
    }
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
}

fn ray_color_2(r : &Ray3, world:&dyn HitRay) -> Vec3 {
    let cfg= SamplingCfg::new(0.0, constants::INFINITY_F64);
    let rec = world.hit(r, cfg);
    match rec {
        Some(hit) => {
            (hit.normal + vec3(1.0, 1.0, 1.0)) * 0.5
        },
        None =>{
            let udir = unit_vector(r.direction());
            let t = 0.5 * (udir.y + 1.0);
            lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
        }
    }
}

fn ray_color(mut r : Ray3, world:&dyn HitRay, mut depth:i32) -> Vec3 {
    let mut col = Vec3::zeros();
    let mut transmissibility = Vec3::ones();
    while depth > 0 {
        let cfg= SamplingCfg::new(0.001, constants::INFINITY_F64);
        let rec = world.hit(&r, cfg);
        match rec {
            Some(hit) => {
                let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
                r = Ray3::new(hit.p, target - hit.p);
                //let v = (hit.normal + vec3(1.0, 1.0, 1.0)) * 0.5;
                transmissibility = transmissibility * 0.5;
            },
            None =>{
                let udir = unit_vector(r.direction());
                let t = 0.5 * (udir.y + 1.0);
                let v = lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t);
                col = Vec3::mul_elements(v, transmissibility);
                break;
            }
        }
        depth = depth - 1;
    }
    col
}

fn do_draw(){
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    // World
    let mut world = HittableObject::mk_list();
    world.push(HittableObject::Sphere(Sphere{center:vec3(0.0,0.0,-1.0), radius:0.5}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(0.0,-100.5,-1.0), radius:100.0}));
    let world_obj = HittableObject::wrap(world);

    // camera
    let mut cam = Camera::default();
    //let viewport_height = 2.0;
    //let viewport_width = aspect_ratio * viewport_height;
    //let focal_length = 1.0;

    //let origin = vec3(0.0,0.0,0.0);
    //let horizontal = vec3(viewport_width, 0.0, 0.0);
    //let vertical = vec3(0.0, viewport_height, 0.0);
    //let lower_left_corner = origin - (horizontal * 0.5) - (vertical * 0.5) - vec3(0.0, 0.0, focal_length);

    let f_w = (image_width - 1) as f64;
    let f_h = (image_height -1) as f64;

    // Render
    let mut pb = ProgressBar::new(image_height as u64);
    //pb.format("╢▌▌░╟");

    let mut file = File::create("out.ppm").unwrap();
    //println!("P3\n{} {}\n255", image_width, image_height);
    writeln!(file, "P3\n{} {}\n255", image_width, image_height);

    for j in (0 .. image_height).rev() {
        let fj = j as f64;
        pb.inc();
        for i in 0 .. image_width
        {
            let fi = i as f64;
            let mut pixel_color = Vec3::zeros();
            for s in 0 .. samples_per_pixel {
                let u = (fi + random_f64_normalized()) / f_w;
                let v = (fj + random_f64_normalized()) / f_h;
                let r = cam.get_ray(u, v);
                //let r = Ray3::new(origin, lower_left_corner + (horizontal * u) + (vertical * v));
                pixel_color = pixel_color + ray_color(r, &world_obj, max_depth);
            }
            //write_color_stdout(col);
            write_color_file_multi(&mut file,pixel_color, samples_per_pixel );
        }
    }
}

fn main() {
    let a = vec3(0.1, 0.1,0.1);
    let b = vec3(1.0, 1.0, 1.0);
    let c = a + b;

    println!("a {:?}, b {:?}, c {:?}",a,b,c );

    //test_ppm();

    do_draw();
}
