
#![allow(dead_code)]

mod raymath;

use raymath::{Vec3, Ray3, vec3, lerp3, unit_vector, 
    hit_sphere, write_color_stdout, 
    HitRecord, HittableObject, constants, SamplingCfg, Sphere, HitRay, MaterialCollection};

use std::fs::File;
use std::io::Write;

use crate::raymath::{write_color_file, Camera, random_f64_normalized, write_color_file_multi, Material};

extern crate pbr;
use pbr::ProgressBar;

fn ray_color(mut r : Ray3, world:&dyn HitRay, mats:&MaterialCollection, mut depth:i32) -> Vec3 {
    let mut col = Vec3::zeros();
    let mut transmissibility = Vec3::ones();
    while depth > 0 {
        let cfg= SamplingCfg::new(0.001, constants::INFINITY_F64);
        let rec = world.hit(&r, cfg);
        match rec {
            Some(hit) => {
                //let target = hit.p + hit.normal + Vec3::random_unit_vector();
                let scatteredResult = mats.materials[hit.mat].scatter(r, hit);
                match scatteredResult {
                    Some(scattered)=>{
                        transmissibility = scattered.attenuation.mul_elements(transmissibility);
                        r = scattered.scattered;
                    },
                    None=>{
                        transmissibility = Vec3::zeros();
                        break;
                    }
                }

                //let target = hit.p + hit.normal + Vec3::random_in_hemisphere(hit.normal);
                //r = Ray3::new(hit.p, target - hit.p);
                //let v = (hit.normal + vec3(1.0, 1.0, 1.0)) * 0.5;
                //transmissibility = transmissibility * 0.5;
            },
            None =>{
                let udir = unit_vector(r.direction());
                let t = 0.5 * (udir.y + 1.0);
                let v = lerp3(vec3(1.0,1.0,1.0), vec3(0.5, 0.7, 1.0), t);
                col = v.mul_elements(transmissibility);
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
    //let image_width = 600;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    // World
    let mut mats = MaterialCollection::new();
    let material_ground = mats.add(Material::mk_lambert(vec3(0.8, 0.8, 0.0))); // 0
    let material_center = mats.add(Material::mk_lambert(vec3(0.7, 0.3, 0.3))); // 1
    let material_left = mats.add(Material::mk_metal(vec3(0.8, 0.8, 0.8))); // 2
    let material_right = mats.add(Material::mk_metal(vec3(0.8, 0.6, 0.2))); // 3

    let mut world = HittableObject::mk_list();
    world.push(HittableObject::Sphere(Sphere{center:vec3(0.0,-100.5,-1.0), radius:100.0, material:material_ground}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(0.0, 0.0,-1.0), radius:0.5, material:material_center}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(-1.0,0.0,-1.0), radius:0.5, material:material_left}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(1.0,0.0,-1.0), radius:0.5, material:material_right}));

    let world_obj = HittableObject::wrap(world);

    // camera
    let mut cam = Camera::default();

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
                pixel_color = pixel_color + ray_color(r, &world_obj, &mats, max_depth);
            }
            write_color_file_multi(&mut file,pixel_color, samples_per_pixel );
        }
    }
}

fn main() {
    do_draw();
}
