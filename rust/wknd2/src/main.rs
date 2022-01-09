
#![allow(dead_code)]

mod raymath;
use std::sync::{Arc, Mutex};
use std::thread;
use raymath::{Vec3, Ray3, vec3, lerp3, unit_vector, 
    hit_sphere, write_color_stdout, 
    HitRecord, HittableObject, constants, SamplingCfg, Sphere, HitRay, MaterialCollection, mk_sphere, random_f64, mk_sphere2};

use std::{fs::File, f64::consts::PI};
use std::io::{Write, Stdout};
use std::time::Instant;
use rayon::prelude::*;

use raymath::{write_color_file, Camera, random_f64_normalized, write_color_file_multi, Material, write_color_to_buf, write_color_file_vec};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

extern crate pbr;
use pbr::ProgressBar;

use crate::raymath::vec3g;

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

fn build_world_2() -> (HittableObject, MaterialCollection) {
    // World
    let R = f64::cos(PI / 4.0);

    let mut mats = MaterialCollection::new();
    let material_left = mats.add(Material::mk_lambert(vec3(0.0,0.0,1.0))); // 2
    let material_right = mats.add(Material::mk_lambert(vec3(1.0,0.0,0.0))); // 2

    let mut world = HittableObject::mk_list();
    world.push(HittableObject::Sphere(Sphere{center:vec3(-R,0.0,-1.0), radius:R, material:material_left}));
    world.push(HittableObject::Sphere(Sphere{center:vec3(R,0.0,-1.0), radius:R, material:material_right}));

    (HittableObject::wrap(world),mats)
}

fn build_world_3() -> (HittableObject, MaterialCollection) {
    // World
    let R = f64::cos(PI / 4.0);
    let mut world = HittableObject::mk_list();
    let mut mats = MaterialCollection::new();

    let ground_material = mats.add(Material::mk_lambert(vec3(0.5,0.5,0.5))); 
    world.push(mk_sphere(0.0, -1000.0, 0.0, 1000.0, ground_material));

    let rnd = || random_f64_normalized();
    for a in (-11 .. 11){
        for b in (-11 .. 11){
            let af = a as f64;
            let bf = b as f64;
            let choose_mat = rnd(); 
            let center = vec3(af + 0.9 * rnd() , 0.2, bf + 0.9 * rnd());
            if (center - vec3(4.0, 0.2, 0.0)).length() <= 0.9 {continue;}

            let sphere_mat = if choose_mat < 0.8 {
                let albedo = Vec3::random(0.0, 1.0).mul_elements(Vec3::random(0.0,1.0));
                mats.add_lambert(albedo)
            }
            else if choose_mat < 0.95 {
                let albedo = Vec3::random(0.5, 1.0);
                let fuzz = random_f64(0.0, 0.5);
                mats.add_metal(albedo, fuzz)
            }
            else {
                mats.add_dielectric(1.5)
            };
            world.push(mk_sphere2(center, 0.2, sphere_mat));
        }
    }

    let mat1 = mats.add_dielectric(1.5);
    world.push(mk_sphere(0.0, 1.0, 0.0, 1.0, mat1));
    let mat2 = mats.add_lambert(vec3(0.4, 0.2, 0.1));
    world.push(mk_sphere(-4.0, 1.0, 0.0, 1.0, mat2));
    let mat3 = mats.add_metal(vec3(0.7, 0.6, 0.5), 0.0);
    world.push(mk_sphere(4.0, 1.0, 0.0, 1.0, mat3));

    (HittableObject::wrap(world),mats)
}

fn do_draw(){
    // Image
    let image_width =600;
    //let image_width =3000;
    //let image_width =1920;
    //let image_width =1200;
    //let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 3.0 / 2.0;
    let cfg = Cfg{
        aspect_ratio : aspect_ratio,
        image_width : image_width,
        image_height : (image_width as f64 / aspect_ratio) as i32,
        //samples_per_pixel : 100,
        samples_per_pixel : 100,
        max_depth : 50
    };

    //let world_obj = HittableObject::wrap(world);
    //let (world_obj, mats) = build_world_1();
    let (world_obj, mats) = build_world_3();//fin
    //let (world_obj, mats) = build_world_2();

    // camera
    //let mut cam = Camera::default();
    //let mut cam = Camera::new_simple(90.0, aspect_ratio);
    //let mut cam = Camera::new(vec3g(-2, 2, 1), vec3g(0, 0, -1), vec3g(0,1,0), 90.0, aspect_ratio);
    //let mut cam = Camera::new(vec3g(-2, 2, 1), vec3g(0, 0, -1), vec3g(0,1,0), 20.0, aspect_ratio);

    //let lookfrom = vec3g(3,3,2);
    let lookfrom = vec3g(13,2,3);
    //let lookat = vec3g(0,0,-1);
    let lookat = vec3g(0,0,0);
    let vup = vec3g(0,1,0);
    //let dist_to_focust = (lookat - lookfrom).length();
    let dist_to_focust = 10.0;
    //let aperture = 2.0;
    let aperture = 0.1;

    let mut cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focust);

    let f_w = (cfg.image_width - 1) as f64;
    let f_h = (cfg.image_height -1) as f64;

    // Render

    //pb.format("╢▌▌░╟");

    let mut pixels = vec![0; (cfg.image_width * cfg.image_height * 3) as usize];
    let chunk_size = (cfg.image_width * 3) as usize;
    let mut bands: Vec<(usize, &mut [i32])> = pixels.chunks_mut(chunk_size).enumerate().collect();
    //let mut pb : Arc<ProgressBar<Stdout>> = Arc::new(ProgressBar::new(bands.len() as u64));
    let mut pb  = Mutex::new(ProgressBar::new(bands.len() as u64));


    // for j in (0 .. cfg.image_height){
    //     let fj = j as f64;
    //     let ju = j as usize;
    //     pb.inc();

    //     //render_line(&mut pixels,&cfg, &cam, &world_obj, &mats, j);
    //     render_line(bands[ju].1,&cfg, &cam, &world_obj, &mats, j);
    // }

    let start = Instant::now();
    bands.into_par_iter().for_each(|(i,band)| {
        pb.lock().unwrap().inc();
        render_line(band,&cfg, &cam, &world_obj, &mats, i as i32);
    });
    println!("Frame time: {}ms", start.elapsed().as_millis());
    //let mut file = File::create("out.ppm").unwrap();
    write_color_file_vec("out.png", cfg.image_width as usize, cfg.image_height as usize, pixels);
}

fn main() {
    do_draw();
}
