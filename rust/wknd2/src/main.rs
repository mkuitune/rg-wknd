#![allow(dead_code)]

mod vec3;
use vec3::{Vec3, Ray3, vec3, lerp3, unit_vector, 
    hit_sphere};

fn test_ppm(){

    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let fnx = nx as f32;
    let fny = ny as f32;
    for j in (0 .. ny).rev() {
        for i in 0 .. nx {
            let col = vec3((i as f32) / fnx, (j as f32) / fny, 0.2);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
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
        let N = unit_vector(r.point_at_parameter(t) - vec3(0.0,0.0,-1.0));
        return vec3(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5;        
    }
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t)
}

fn gradient_bgr(){
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);
    let fnx = nx as f32;
    let fny = ny as f32;
    for j in (0 .. ny).rev() {
        for i in 0 .. nx {
            let u = (i as f32) / fnx;
            let v = (j as f32) / fny;
            let r = Ray3::new(origin, lower_left + (horizontal * u) + (vertical * v));
            let col = color4(&r);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    let a = vec3(0.1, 0.1,0.1);
    let b = vec3(1.0, 1.0, 1.0);
    let _c = a + b;
    //println!("{:?}",c );
    //test_ppm();
    gradient_bgr();
}
