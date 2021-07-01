use image::{ImageBuffer, RgbaImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    time::Instant
};

use raytracing_weekend::*;

pub fn ray_color(r: &Ray, world: &World, depth: u64) -> Vec3 {
    if depth <= 0 {
        return Vec3::ZERO;
    }

    if let Some(record) = world.hit(&r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scatterd)) = record.mat.scatter(&r, &record) {
            return attenuation.to_vec3() * ray_color(&scatterd, &world, depth - 1);
        } else {
            return Vec3::ZERO;
        }
    }

    // gradient for background
    let unit = r.dir.normalize();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::ONE + t * vec3(0.5, 0.7, 1.0)
}

fn main() {
    // config 
    let config = Config::load(std::path::Path::new("config.txt"));

    // image buffer
    let mut img: RgbaImage = ImageBuffer::new(config.width, config.height);

    // scene
    let (world, camera) = scenes::spheres(config.aspect_ratio);
    
    // meta data
    let clock = Instant::now();
    println!("Rendering {} objects", world.len());

    // render stage
    let mut buffer: Vec<Color> = Vec::with_capacity((config.width * config.height) as usize);
    for y in 0..config.height {
        let mut line: Vec<Color> = (0..config.width).into_par_iter().map(|x|{
            let mut color = Vec3::ZERO;
            for _ in 0..config.samples {
                let u = (x as f64 + random()) / ((config.width - 1) as f64);
                let v = (y as f64 + random()) / ((config.height - 1) as f64);
    
                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, config.depth as u64);
            }
            
            Color::from_vec(color, config.samples as u64)
        }).collect();

        buffer.append(&mut line);
    }

    let dt = clock.elapsed().as_secs_f32();
    println!("Render time : {}s", dt);

    // copy buffer to image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba(buffer[(y * config.width + x) as usize].into());
    }

    // save img
    match image::imageops::flip_vertical(&img).save("out.png") {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
