
#![allow(dead_code)]

mod vec3;

use vec3::{Vec3, Ray3, vec3, lerp3, unit_vector, 
    hit_sphere, write_color_stdout};

use std::fs::File;
use std::io::Write;

use crate::vec3::write_color_file;

fn test_ppm(){

    let image_width = 200;
    let image_height = 100;
    println!("P3\n{} {}\n255", image_width, image_height);
    let fnx = image_width as f32;
    let fny = image_height as f32;
    for j in (0 .. image_height).rev() {
        for i in 0 .. image_width
 {
            let col = vec3((i as f32) / fnx, (j as f32) / fny, 0.2);
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
        let N = unit_vector(r.at(t) - vec3(0.0,0.0,-1.0));
        return vec3(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5;        
    }
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
}

fn ray_color(r : &Ray3) -> Vec3 {

    let t = hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let N = unit_vector(r.at(t) - vec3(0.0,0.0,-1.0));
        return vec3(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5;        
    }
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
}

fn gradient_bgr(){
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let mut file = File::create("out.ppm").unwrap();
    //println!("P3\n{} {}\n255", image_width, image_height);
    writeln!(file, "P3\n{} {}\n255", image_width, image_height);

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3(0.0,0.0,0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal * 0.5) - (vertical * 0.5) - vec3(0.0, 0.0, focal_length);

    let f_w = (image_width - 1) as f32;
    let f_h = (image_height -1) as f32;
    for j in (0 .. image_height).rev() {
        for i in 0 .. image_width
        {
            let u = (i as f32) / f_w;
            let v = (j as f32) / f_h;
            let r = Ray3::new(origin, lower_left_corner + (horizontal * u) + (vertical * v));
            let col = ray_color(&r);
            //write_color_stdout(col);
            write_color_file(&mut file, col);
        }
    }
}

fn main() {
    let a = vec3(0.1, 0.1,0.1);
    let b = vec3(1.0, 1.0, 1.0);
    let c = a + b;

    println!("a {:?}, b {:?}, c {:?}",a,b,c );

    //test_ppm();

    gradient_bgr();
}
