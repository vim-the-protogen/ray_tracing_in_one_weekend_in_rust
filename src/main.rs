#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod color;
mod vec3;

use crate::vec3::vec3::Vec3;
use crate::color::color::utils::write_color;
use crate::color::color::Color;

fn main() {
    // image
    let image_width = 256;
    let image_height = 256;

    // render
    println!("P3\n {} {}\n255", image_width, image_height);

    for j in (0..(image_height - 1)).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color: Color  = Vec3::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64), 
                0.25
            ) as Color;

            write_color(pixel_color);
        }
    }
}
