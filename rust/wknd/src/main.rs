fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let fnx = nx as f32;
    let fny = ny as f32;
    for j in (0 .. ny).rev() {
        for i in 0 .. nx {
            let r = (i as f32) / fnx;
            let g = (j as f32) / fny;
            let b : f32 = 0.2;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
