mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec;
use hittable::Hittable;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::unit_vector;
use crate::vec::{Color, Point, Vec3};
use rand::Rng;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn ray_color(r: &Ray, world: &HittableList, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            return ray_color(&scattered, world, depth - 1) * attenuation;
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_IMAGE: u32 = 200;
    const MAX_DEPTH: u64 = 50;
    const PIXEL_COUNT: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
    // World
    let mut world = HittableList::new();
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.add(Arc::new(sphere_ground));
    world.add(Arc::new(sphere_center));
    world.add(Arc::new(sphere_left));
    world.add(Arc::new(sphere_right));

    // Camera
    let cam = Camera::new();

    let start = Instant::now();

    let mut s = String::new();
    write!(s, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    let pixels = Arc::new(Mutex::new(vec![
        Color::new(0.0, 0.0, 0.0);
        PIXEL_COUNT as usize
    ]));
    (0..PIXEL_COUNT).into_par_iter().for_each(|n| {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        // calculate current coordinate
        let x = (n % IMAGE_WIDTH) as f64;
        let y = (IMAGE_HEIGHT - 1 - n / IMAGE_WIDTH) as f64;
        for _ in 0..SAMPLES_PER_IMAGE {
            let u = (x + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = (y + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let r = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, MAX_DEPTH);
        }
        pixels.lock().unwrap()[n as usize] = pixel_color;
    });

    let unlocked_pixels = pixels.lock().unwrap();
    for pixel in unlocked_pixels.iter() {
        write!(s, "{}\n", pixel.write_color(SAMPLES_PER_IMAGE as f64)).unwrap()
    }

    println!("{}", s);
    let duration = start.elapsed();
    eprintln!("\nDone in {:?}", duration)
}
