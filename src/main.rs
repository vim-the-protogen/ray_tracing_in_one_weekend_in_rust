#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod color;
mod vec3;
mod ray;

use crate::vec3::vec3::*;
use crate::color::color::utils::write_color;
use crate::color::color::Color;
use crate::ray::ray::Ray;
use crate::ray::ray::Point3;

// linearly blends white to blue depend on the heigt of the y coordinat
// after scalin the ray direction to unit length
fn ray_color(r: Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) +
        Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 -
        vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // render
    println!("P3\n {} {}\n255", image_width, image_height);

    for j in (0..(image_height - 1)).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
                
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64); 

            let r = Ray::new(origin, 
                lower_left_corner + horizontal * u + vertical * v - origin);

            let pixel_color: Color = ray_color(r);
            write_color(pixel_color);
        }
    }
}
