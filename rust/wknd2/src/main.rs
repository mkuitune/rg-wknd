
#![allow(dead_code)]

mod raymath;

use raymath::{Vec3, Ray3, vec3, lerp3, unit_vector, 
    hit_sphere, write_color_stdout, 
    HitRecord, HittableObject, constants, SamplingCfg, Sphere, HitRay, MaterialCollection};

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rayon::prelude::*;

use crate::raymath::{write_color_file, Camera, random_f64_normalized, write_color_file_multi, Material, write_color_to_buf, write_color_file_vec};

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

struct Cfg{
    pub aspect_ratio : f64,
    pub image_width:i32,
    pub image_height:i32,
    pub samples_per_pixel:i32,
    pub max_depth:i32
}

//fn render_line(pixels:&mut Vec<i32>, cfg: &Cfg, cam:&Camera, hittable:&HittableObject, mats:&MaterialCollection, y:i32, ){
fn render_line(pixels:&mut [i32], cfg: &Cfg, cam:&Camera, hittable:&HittableObject, mats:&MaterialCollection, y:i32, ){
        let fj = (cfg.image_height - y -1) as f64;
        let f_w = (cfg.image_width - 1) as f64;
        let f_h = (cfg.image_height -1) as f64;
        for i in 0 .. cfg.image_width
        {
            let fi = i as f64;
            let mut pixel_color = Vec3::zeros();
            for s in 0 .. cfg.samples_per_pixel {
                let u = (fi + random_f64_normalized()) / f_w;
                let v = (fj + random_f64_normalized()) / f_h;
                let r = cam.get_ray(u, v);
                //let r = Ray3::new(origin, lower_left_corner + (horizontal * u) + (vertical * v));
                pixel_color = pixel_color + ray_color(r, hittable, &mats, cfg.max_depth);
            }
            //let idx = ((cfg.image_height - y - 1) * cfg.image_width + i) as usize;
            let idx = (i) as usize;
            write_color_to_buf(pixels,idx,pixel_color, cfg.samples_per_pixel );
        }
}

fn build_world_1() -> (HittableObject, MaterialCollection) {
    // World
    let mut mats = MaterialCollection::new();
    let material_ground = mats.add(Material::mk_lambert(vec3(0.8, 0.8, 0.0))); // 0
    //let material_ground = mats.add(Material::mk_metal(vec3(0.8, 0.8, 0.0),0.5)); // 0
    //let material_center = mats.add(Material::mk_lambert(vec3(0.7, 0.3, 0.3))); // 1
    let material_center = mats.add(Material::mk_lambert(vec3(0.1, 0.2, 0.5))); // 1
    //let material_center = mats.add(Material::mk_dielectric(1.5)); // 1
    //let material_left = mats.add(Material::mk_metal(vec3(0.8, 0.8, 0.8), 0.3)); // 2
    let material_left = mats.add(Material::mk_dielectric(1.5)); // 2
    //let material_right = mats.add(Material::mk_metal(vec3(0.8, 0.6, 0.2),1.0)); // 3
    let material_right = mats.add(Material::mk_metal(vec3(0.8, 0.6, 0.2),0.0)); // 3

    let mut world = HittableObject::mk_list();
    world.push(HittableObject::Sphere(Sphere{center:vec3(0.0,-100.5,-1.0), radius:100.0, material:material_ground}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(0.0, 0.0,-1.0), radius:0.5, material:material_center}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(-1.0,0.0,-1.0), radius:0.5, material:material_left}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(-1.0,0.0,-1.0), radius:-0.4, material:material_left}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(1.0,0.0,-1.0), radius:0.5, material:material_right}));

    (HittableObject::wrap(world),mats)
}

fn do_draw(){
    // Image
    //let image_width =600;
    //let image_width =3000;
    //let image_width =1920;
    let image_width =1200;
    let aspect_ratio = 16.0 / 9.0;
    let cfg = Cfg{
        aspect_ratio : aspect_ratio,
        image_width : image_width,
        image_height : (image_width as f64 / aspect_ratio) as i32,
        samples_per_pixel : 100,
        max_depth : 50
    };

    //let world_obj = HittableObject::wrap(world);
    let (world_obj, mats) = build_world_1();

    // camera
    let mut cam = Camera::default();

    let f_w = (cfg.image_width - 1) as f64;
    let f_h = (cfg.image_height -1) as f64;

    // Render

    let mut pb = ProgressBar::new(cfg.image_height as u64);
    //pb.format("╢▌▌░╟");

    let mut pixels = vec![0; (cfg.image_width * cfg.image_height * 3) as usize];
    let chunk_size = (cfg.image_width * 3) as usize;
    let mut bands: Vec<(usize, &mut [i32])> = pixels.chunks_mut(chunk_size).enumerate().collect();


    // for j in (0 .. cfg.image_height){
    //     let fj = j as f64;
    //     let ju = j as usize;
    //     pb.inc();

    //     //render_line(&mut pixels,&cfg, &cam, &world_obj, &mats, j);
    //     render_line(bands[ju].1,&cfg, &cam, &world_obj, &mats, j);
    // }

    let start = Instant::now();
    bands.into_par_iter().for_each(|(i,band)| {
        render_line(band,&cfg, &cam, &world_obj, &mats, i as i32);
    });
    println!("Frame time: {}ms", start.elapsed().as_millis());
    let mut file = File::create("out.ppm").unwrap();
    write_color_file_vec(&mut file, cfg.image_width as usize, cfg.image_height as usize, pixels);
}

fn main() {
    do_draw();
}
