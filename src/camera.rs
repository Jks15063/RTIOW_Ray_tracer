use core::f64;
use rand::Rng;
use std::time::Instant;

use crate::color::{self, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Camera {
    image_height: i32,
    image_width: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    background: Color,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        background: Color,
        vfov: i32,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = (image_width / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = lookfrom;

        // Determin viewport dimensions

        let theta = degrees_to_radians(vfov as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width / image_height as f64);

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            background,
            u,
            v,
            w,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let start = Instant::now();
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", (self.image_height - j));
            for i in 0..self.image_width as i32 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(r, self.max_depth, world, self.background);
                }

                println!(
                    "{}",
                    color::write_color(self.pixel_samples_scale * pixel_color)
                );
            }
        }

        let duration = start.elapsed();
        eprintln!("Done.");
        eprintln!("Render time: {:?}", duration);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rand::rng().random();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = vec3::random_in_unit_disk();

        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}

fn sample_square() -> Vec3 {
    let x: f64 = rand::rng().random();
    let y: f64 = rand::rng().random();

    Vec3::new(x - 0.5, y - 0.5, 0.0)
}

fn ray_color(r: Ray, depth: i32, world: &dyn Hittable, background: Color) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, rec.p);

        if let Some((attenuation, scattered)) = rec.mat.scatter(r, rec) {
            let color_from_scatter =
                attenuation * ray_color(scattered, depth - 1, world, background);

            return color_from_emission + color_from_scatter;
        }
        return color_from_emission;
    }

    background
}
