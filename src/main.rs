mod vec;

use std::io::{stderr, Write};

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let r: f64 = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let ir: u64 = (255.999 * r) as u64;
            let ig: u64 = (255.999 * g) as u64;
            let ib: u64 = (255.999 * b) as u64;

            println!("{} {} {}", ir, ig, ib)
        }
    }

    eprintln!("\nDone.")
}
