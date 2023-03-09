mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec;
use hittable::Hittable;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::unit_vector;
use crate::vec::{Color, Point, Vec3};
use rand::Rng;
use std::fmt::Write;
use std::io::stderr;
use std::io::Write as IoWrite;
use std::time::Instant;

fn ray_color(r: &Ray, world: &HittableList, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
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
    const SAMPLES_PER_IMAGE: u32 = 100;
    const MAX_DEPTH: u64 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    write!(s, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_IMAGE {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write!(s, "{}\n", pixel_color.write_color(SAMPLES_PER_IMAGE as f64)).unwrap()
        }
    }
    println!("{}", s);
    let duration = start.elapsed();
    eprintln!("\nDone in {:?}", duration)
}
