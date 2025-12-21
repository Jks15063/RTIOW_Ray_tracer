use core::f64;

use hittable_list::HittableList;
use sphere::Sphere;

use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}

pub fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = vec3::dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

pub fn ray_color(r: Ray, world: Box<&dyn Hittable>) -> Color {
    if let Some(rec) = world.hit(&r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;

    // Calculate the image height, ensure at least 1
    let image_height = (image_width / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width as i32 {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r, Box::new(&world));
            println!("{}", color::write_color(pixel_color));
        }
    }

    eprintln!("Done.");
}
