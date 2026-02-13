use core::f64;
use rand::Rng;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
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
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
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

        let sqrt_spp = samples_per_pixel.isqrt();
        let pixel_samples_scale = 1.0 / (sqrt_spp as f64 * sqrt_spp as f64);
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

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
            sqrt_spp,
            recip_sqrt_spp,
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
        let completed = AtomicUsize::new(0);
        let total_lines = self.image_height as usize;
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let pixels: Vec<Color> = (0..self.image_height)
            .into_par_iter()
            .flat_map_iter(|j| {
                let row: Vec<Color> = (0..self.image_width as i32)
                    .map(move |i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                        for s_j in 0..self.sqrt_spp {
                            for s_i in 0..self.sqrt_spp {
                                let r = self.get_ray(i, j, s_i, s_j);
                                pixel_color += ray_color(r, self.max_depth, world, self.background);
                            }
                        }

                        self.pixel_samples_scale * pixel_color
                    })
                    .collect();

                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("Progress: {:.1}%", 100.0 * done as f64 / total_lines as f64);
                row
            })
            .collect();

        for pixel in pixels {
            println!("{}", color::write_color(pixel));
        }

        let duration = start.elapsed();
        eprintln!("Done.");
        eprintln!("Render time: {:?}", duration);
    }

    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
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

    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px: f64 = (s_i as f64 + rand::rng().random::<f64>()) * self.recip_sqrt_spp - 0.5;
        let py: f64 = (s_j as f64 + rand::rng().random::<f64>()) * self.recip_sqrt_spp - 0.5;

        Vec3::new(px, py, 0.0)
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
        let color_from_emission = rec.mat.emitted(r, &rec, rec.u, rec.v, rec.p);

        if let Some((attenuation, scattered, pdf_value)) = rec.mat.scatter(r, &rec) {
            let on_light = Point3::new(
                rand::rng().random_range(213.0..343.0),
                554.0,
                rand::rng().random_range(227.0..332.0),
            );
            let to_light = on_light - rec.p;
            let distance_squared = to_light.length_squared();
            let to_light = vec3::unit_vector(to_light);

            if vec3::dot(to_light, rec.normal) < 0.0 {
                return color_from_emission;
            }

            let light_area = (343.0 - 213.0) * (332.0 - 227.0);
            let light_cosine = to_light.y().abs();

            if light_cosine < 0.000001 {
                return color_from_emission;
            }

            let pdf_value = distance_squared / (light_cosine * light_area);
            let scattered = Ray::new(rec.p, to_light, r.time());
            let scattering_pdf = rec.mat.scattering_pdf(r, &rec, scattered);

            let color_from_scatter =
                (attenuation * scattering_pdf * ray_color(scattered, depth - 1, world, background))
                    / pdf_value;

            return color_from_emission + color_from_scatter;
        }
        return color_from_emission;
    }

    background
}
