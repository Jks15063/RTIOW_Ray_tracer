use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight};
use crate::material::{Lambertian, Metal};
use crate::obj_loader::load_obj;
use crate::quad::Quad;
use crate::rtw_stb_image::ImageTexture;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, PerlinNoise};
use crate::vec3::{Point3, Vec3};
use core::f64;
use hittable::Hittable;
use rand::Rng;
use std::sync::Arc;
use std::time::Instant;

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod obj_loader;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod sphere;
mod texture;
mod triangle;
mod vec3;

fn main() {
    match 12 {
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
        9 => {
            cornell_smoke();
        }
        10 => {
            final_scene(800, 10000, 50);
        }
        11 => {
            teapot_box();
        }
        12 => {
            buddha_box();
        }
        _ => {
            ();
        }
    }
}

fn buddha_box() {
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white1 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let white2 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let white3 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Box::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Box::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    let quad1 = Box::new(Quad::new(
        Point3::new(2.0, 0.0, -5.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        green,
    ));

    let quad2 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, -5.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        red,
    ));

    let quad3 = Box::new(Quad::new(
        Point3::new(-4.0, 9.99, 1.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -2.0),
        light,
    ));

    let quad4 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, -5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        white1,
    ));

    let quad5 = Box::new(Quad::new(
        Point3::new(-8.0, 10.0, -5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        white2,
    ));

    let quad6 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, 5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        white3,
    ));

    let box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(3.0, 8.0, 3.0),
        // || Box::new(Lambertian::from_color(Color::new(0.33, 0.33, 0.73))),
        || Box::new(Metal::new(Color::new(0.73, 0.73, 0.73), 0.02)),
    );
    let box1 = Box::new(Translate::new(box1, Vec3::new(-1.6, 0.0, 0.3)));
    let box1 = Box::new(RotateY::new(box1, -49.0));

    let buddha_mat = Arc::new(Lambertian::from_color(Color::new(0.8, 0.5, 0.2)));
    let buddha = Box::new(load_obj("buddha/buddha.obj", 8.0, buddha_mat));
    let buddha = Box::new(Translate::new(buddha, Vec3::new(-5.0, 3.0, -1.0)));
    eprintln!("Buddha bbox: {:?}", buddha.bounding_box());

    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(box1);
    world.add(buddha);
    eprintln!("Num objects: {}", world.objects.len());
    eprintln!("Building BVH...");
    let start = Instant::now();
    let bvh = BVHNode::from_list(world);
    eprintln!("BVH built in {:?}", start.elapsed());

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 600.0;
    let samples_per_pixel = 2000;
    let max_depth = 50;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 50;
    let lookfrom = Point3::new(-3.0, 5.5, -15.0);
    let lookat = Point3::new(-3.0, 5.0, 0.0);
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

    cam.render(&bvh);
}

fn teapot_box() {
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white1 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let white2 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let white3 = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Box::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Box::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    let quad1 = Box::new(Quad::new(
        Point3::new(2.0, 0.0, -5.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        green,
    ));

    let quad2 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, -5.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        red,
    ));

    let quad3 = Box::new(Quad::new(
        Point3::new(-1.0, 9.9, 1.0),
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -2.0),
        light,
    ));

    let quad4 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, -5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        white1,
    ));

    let quad5 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, 5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        white2,
    ));

    let quad6 = Box::new(Quad::new(
        Point3::new(-8.0, 0.0, 5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        white3,
    ));

    let teapot_mat = Arc::new(Lambertian::from_color(Color::new(0.8, 0.5, 0.2)));
    // let teapot_mat = Arc::new(Dielectric::new(1.5));
    let teapot = Box::new(load_obj("teapot.obj", 1.0, teapot_mat));
    let teapot = Box::new(Translate::new(teapot, Vec3::new(-3.0, 0.0, -2.5)));

    let teapot_mat2 = Arc::new(Dielectric::new(1.5));
    let teapot2 = Box::new(load_obj("teapot.obj", 1.0, teapot_mat2));
    let teapot2 = Box::new(Translate::new(teapot2, Vec3::new(-3.0, 0.0, -2.5)));
    world.add(Box::new(ConstantMedium::from_color(
        teapot2,
        2.0,
        Color::new(0.2, 0.4, 0.9),
    )));

    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(teapot);
    let bvh = BVHNode::from_list(world);

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 600.0;
    let samples_per_pixel = 1000;
    let max_depth = 40;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 40;
    let lookfrom = Point3::new(-3.0, 5.5, -12.0);
    let lookat = Point3::new(-3.0, 2.0, 0.0);
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

    cam.render(&bvh);
}

fn final_scene(image_width: i32, samples_per_pixel: i32, max_depth: i32) {
    let mut boxes1 = HittableList::new();

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1: f64 = rand::rng().random_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(quad::make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                || Box::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53))),
            ));
        }
    }

    let bvh = BVHNode::from_list(boxes1);
    let mut world = HittableList::new();
    world.add(Box::new(bvh));

    let light = Box::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    let light_quad = Box::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));
    world.add(light_quad);

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Point3::new(30.0, 0.0, 0.0);
    let sphere_material = Box::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1)));
    let sphere = Box::new(Sphere::new_moving(center1, center2, 50.0, sphere_material));
    world.add(sphere);

    world.add(Box::new(Sphere::new_static(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Box::new(Dielectric::new(1.5)),
    )));

    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Box::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary1 = Box::new(Sphere::new_static(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Box::new(Dielectric::new(1.5)),
    ));

    let boundary2 = Box::new(Sphere::new_static(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Box::new(Dielectric::new(1.5)),
    ));
    world.add(boundary1);

    world.add(Box::new(ConstantMedium::from_color(
        boundary2,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Box::new(Sphere::new_static(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Box::new(Dielectric::new(1.5)),
    ));
    world.add(Box::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Box::new(Lambertian::new(Box::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Box::new(Sphere::new_static(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertex = Box::new(PerlinNoise::new(0.2));
    world.add(Box::new(Sphere::new_static(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Box::new(Lambertian::new(pertex)),
    )));

    let mut boxes2 = HittableList::new();

    for _i in 0..1000 {
        let white = Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
        boxes2.add(Box::new(Sphere::new_static(
            vec3::random_range(0.0, 165.0),
            10.0,
            white,
        )));
    }

    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(Box::new(BVHNode::from_list(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width = image_width;
    let samples_per_pixel = samples_per_pixel;
    let max_depth = max_depth;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 40;
    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width as f64,
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

fn cornell_smoke() {
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

    let glass_box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        || Box::new(Dielectric::new(1.5)),
    );
    let glass_box1 = Box::new(RotateY::new(glass_box1, 15.0));
    let glass_box1 = Box::new(Translate::new(glass_box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(glass_box1);

    let box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        || Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73))),
    );
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let box1 = Box::new(ConstantMedium::from_color(
        box1,
        0.10,
        Color::new(0.0, 0.0, 0.0),
    ));

    let glass_box2 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        || Box::new(Dielectric::new(1.5)),
    );
    let glass_box2 = Box::new(RotateY::new(glass_box2, -18.0));
    let glass_box2 = Box::new(Translate::new(glass_box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(glass_box2);

    let box2 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        || Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73))),
    );
    let box2 = Box::new(RotateY::new(box2, -18.0));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    let box2 = Box::new(ConstantMedium::from_color(
        box2,
        0.10,
        Color::new(0.2, 0.3, 0.8),
    ));

    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(box1);
    world.add(box2);
    let bvh = BVHNode::from_list(world);

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 600.0;
    let samples_per_pixel = 1000;
    let max_depth = 500;
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

    cam.render(&bvh);
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

    let box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        // || Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73))),
        || Box::new(Metal::new(Color::new(0.73, 0.73, 0.73), 0.0)),
    );
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        || Box::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73))),
    );
    let box2 = Box::new(RotateY::new(box2, -18.0));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(box1);
    world.add(box2);

    let aspect_ratio: f64 = 1.0;
    let image_width: f64 = 600.0;
    let samples_per_pixel = 10000;
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
        // red,
        // Box::new(Dielectric::new(1.5)),
        Box::new(Metal::new(Color::new(0.75, 0.2, 0.2), 0.0)),
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
    let samples_per_pixel = 400;
    let max_depth = 400;
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
    let samples_per_pixel = 1000;
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
