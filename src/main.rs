use bvh::BVHNode;
use material::{Dielectric, DiffuseLight};
use quad::Quad;
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
mod quad;
mod ray;
mod rtw_stb_image;
mod sphere;
mod texture;
mod vec3;

fn main() {
    match 8 {
        1 => {
            bouncing_spheres();
        }
        2 => {
            checkered_spheres();
        }
        3 => {
            earth();
        }
        4 => {
            perlin_spheres();
        }
        5 => {
            quads();
        }
        6 => {
            mirrors();
        }
        7 => {
            simple_light();
        }
        8 => {
            cornell_box();
        }
        _ => {
            ();
        }
    }
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white1 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let white2 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let white3 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Box::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Box::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    let quad1 = Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));

    let quad2 = Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));

    let quad3 = Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));

    let quad4 = Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white1,
    ));

    let quad5 = Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white2,
    ));

    let quad6 = Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white3,
    ));

    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 600.0;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 40;
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
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

fn simple_light() {
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

    let difflight_mat1 = Box::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    let difflight_mat2 = Box::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));

    let quad_light = Box::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight_mat1,
    ));

    let sphere_light = Box::new(Sphere::new_static(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight_mat2,
    ));

    world.add(sphere1);
    world.add(sphere2);
    world.add(quad_light);
    world.add(sphere_light);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 20;
    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
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

fn mirrors() {
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::from_color(Color::new(1.0, 0.0, 0.0)));
    let left_mirror = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let back_mirror = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let right_mirror = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let upper_mirror = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let lower_mirror = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, 0.0, 3.0),
        1.0,
        red,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_mirror,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_mirror,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_mirror,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_mirror,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_mirror,
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 80;
    let lookfrom = Point3::new(2.0, 1.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
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

fn quads() {
    let mut world = HittableList::new();

    let left_red = Box::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Box::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Box::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Box::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Box::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 80;
    let lookfrom = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        background,
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
