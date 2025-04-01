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
    const VIEWPORT_HEIGHT : f64 = 2.0;
    const VIEWPORT_WIDTH : f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH : f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // render
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // constant ranges encompassing each quanta of the image's
    // axises
    const Y_COORDS: Range<i64> = 0..(IMAGE_HEIGHT - 1);
    const X_COORDS: Range<i64> = 0..IMAGE_WIDTH; 

    // functions to create all x/y pairs of the image's coordinates
    let map_to_axis = | p: f64, axis_size: i64| p / ((axis_size - 1) as f64);
    let select_x = | coord: (f64, _) | map_to_axis(coord.0, IMAGE_WIDTH);
    let select_y = | coord: (_, f64) | map_to_axis(coord.1, IMAGE_HEIGHT);
    let select_coords = | y | X_COORDS.map(move | x | (x as f64, y as f64));

    // functions for getting the direction of the ray at a particular pixel
    let scale_horizontal = | coord: (f64, _) | (horizontal * coord.0) as Vec3;
    let scale_vertical = | coord: (_, f64) | (vertical * coord.1) as Vec3;
    let direction = | coord: (f64, f64) |   lower_left_corner
                                          + scale_horizontal(coord)
                                          + scale_vertical(coord)
                                          - origin;

    Y_COORDS.rev()
            .flat_map(select_coords)
            .map(| coord | (select_x(coord), select_y(coord)))
            .map(| coord | Ray::new(origin, direction(coord)))
            .map(ray_color)
            .for_each(write_color);
}
