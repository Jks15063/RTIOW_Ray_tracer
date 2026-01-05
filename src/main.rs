use bvh::BVHNode;
use material::Dielectric;
use rand::Rng;
use rtw_stb_image::ImageTexture;
use texture::{CheckerTexture, PerlinNoise};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use core::f64;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod ray;
mod rtw_stb_image;
mod sphere;
mod texture;
mod vec3;

fn main() {
    match 4 {
        1 => {
            earth();
        }
        2 => {
            checkered_spheres();
        }
        3 => {
            bouncing_spheres();
        }
        4 => {
            perlin_spheres();
        }
        _ => {
            hello();
        }
    }
}

fn hello() {
    eprintln!("Hello")
}

fn perlin_spheres() {
    let mut world = HittableList::new();
    let pertext1 = Box::new(PerlinNoise::new(4.0));
    let pertext2 = Box::new(PerlinNoise::new(4.0));
    let mat1 = Box::new(Lambertian::new(pertext1));
    let mat2 = Box::new(Lambertian::new(pertext2));
    let sphere1 = Box::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat1,
    ));
    let sphere2 = Box::new(Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, mat2));

    world.add(sphere1);
    world.add(sphere2);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Render

    cam.render(&world);
}

fn earth() {
    let mut world = HittableList::new();
    let earth_texture = Box::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Box::new(Lambertian::new(earth_texture));
    let globe = Box::new(Sphere::new_static(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    world.add(globe);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20;
    let lookfrom = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Render

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker_material1 = Box::new(Lambertian::new(Box::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));

    let checker_material2 = Box::new(Lambertian::new(Box::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));

    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker_material1,
    )));

    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker_material2,
    )));

    // Camera

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Render

    cam.render(&world);
}

fn bouncing_spheres() {
    // World

    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Box::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));

    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::rng().random();
            let center = Point3::new(
                a as f64 + 0.9 * rand::rng().random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::rng().random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = color::random() * color::random();
                    let sphere_material = Box::new(Lambertian::from_color(albedo));
                    let center2 = center + Vec3::new(0.0, rand::rng().random_range(0.0..0.5), 0.0);
                    world.add(Box::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = color::random_range(0.5, 1.0);
                    let fuzz = rand::rng().random_range(0.0..0.5);
                    let sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new_static(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material = Box::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new_static(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Box::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let bvh = BVHNode::from_list(world);

    // Camera

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Render

    cam.render(&bvh);
}
