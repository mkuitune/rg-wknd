#![allow(dead_code)]

mod vec3;
use vec3::vec3::Vec3;
use vec3::vec3::Ray3;
use vec3::vec3::vec3;
use vec3::vec3::lerp3;
use vec3::vec3::unit_vector;

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

fn color(r : Ray3) -> Vec3 {
    let udir = unit_vector(r.direction());
    let t = 0.5 * (udir.y + 1.0);
    lerp3(Vec3{x:1.0,y:1.0,z:1.0}, Vec3{x:0.5, y:0.7, z:1.0}, t)
}

fn main() {
    let a = Vec3{x:0.1, y:0.1, z:0.1};
    let b = Vec3{x:1.0, y:1.0, z:1.0};
    let _c = a + b;
    //println!("{:?}",c );
    test_ppm();
}
