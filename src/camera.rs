use crate::color::{self, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
use rand::Rng;

pub struct Camera {
    image_height: i32,
    image_width: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64, samples_per_pixel: i32) -> Self {
        let image_height = (image_width / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Camera {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", (self.image_height - j));
            for i in 0..self.image_width as i32 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(r, world);
                }

                println!(
                    "{}",
                    color::write_color(self.pixel_samples_scale * pixel_color)
                );
            }
        }

        eprintln!("Done.");
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    let x: f64 = rand::rng().random();
    let y: f64 = rand::rng().random();

    Vec3::new(x - 0.5, y - 0.5, 0.0)
}

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(&r, Interval::new(0.0, f64::INFINITY)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
