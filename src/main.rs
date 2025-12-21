use crate::camera::Camera;
use core::f64;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;
    let cam = Camera::new(aspect_ratio, image_width);

    // Render

    cam.render(&world);
}
