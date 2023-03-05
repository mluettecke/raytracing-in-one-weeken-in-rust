mod vec;

use std::io::{stderr, Write};
use vec::Color;
fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let color: Color = Color::new(
                (i as f64) / (IMAGE_WIDTH - 1) as f64,
                (j as f64) / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            println!("{}", color.write_color());
        }
    }

    eprintln!("\nDone.")
}
