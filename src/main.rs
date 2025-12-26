use material::Dielectric;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Point3;
use core::f64;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World

    let mut world = HittableList::new();

    let R = (f64::consts::PI / 4.0).cos();

    let material_left = Box::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Box::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    // Camera

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let cam = Camera::new(aspect_ratio, image_width, 100, 50, 90);

    // Render

    cam.render(&world);
}
