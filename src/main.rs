#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::vec3::vec3::*;
use std::ops::Range;
use crate::color::color::{Color, utils::write_color};
use crate::ray::ray::{Ray, Point3};
use crate::sphere::sphere::Sphere;

// checks to see if a sphere is hit by a ray
// simplified because b = -2h and the dot product 
fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = dot(ray.direction(), ray.direction());
    let half_b = dot(oc, ray.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

// linearly blends white to blue depend on the heigt of the y coordinat
// after scalin the ray direction to unit length
fn ray_color(ray: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let n = unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
    } else {
        let unit_direction = unit_vector(ray.direction());
        t = 0.5 * (unit_direction.y() + 1.0);
        (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) +
            (Color::new(0.5, 0.7, 1.0) * t)
    }
}

fn main() {
    // image
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : i64 = 400;
    const IMAGE_HEIGHT : i64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // render
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    const Y_COORDS : Range<i64> = 0..(IMAGE_HEIGHT - 1);
    const X_COORDS : Range<i64> = 0..IMAGE_WIDTH;
    let foo = | x, y | (x as f64, y as f64);
    let pair_x_y = | y | X_COORDS.map(move | x | foo(x, y));
    let x_pixel = | x : f64 | x / ((IMAGE_WIDTH - 1) as f64);
    let y_pixel = | y : f64 | y / ((IMAGE_HEIGHT - 1) as f64);
    let direction = | coord : (f64, f64) |  lower_left_corner + horizontal * coord.0 + vertical * coord.1 - origin;

    let _ = Y_COORDS.rev()
                    .flat_map(| x | pair_x_y(x))
                    .map(| coord | (x_pixel(coord.0), y_pixel(coord.1)))
                    .map(| coord | Ray::new(origin, direction(coord)))
                    .map(| ray | ray_color(ray))
                    .map(| pixel_color | write_color(pixel_color))
                    .collect::<Vec<_>>();
}
