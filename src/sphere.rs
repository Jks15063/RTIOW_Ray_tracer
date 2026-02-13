use core::f64;

use rand::Rng;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Box<dyn Material>,
    pub bbox: AABB,
}

impl Sphere {
    pub fn new_static(center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - rvec, center + rvec);

        Sphere {
            center: Ray::new(center, Vec3::new(0.0, 0.0, 0.0), 0.0),
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Box<dyn Material>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1, 0.0);
        let box1 = AABB::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = AABB::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = AABB::from_aabb(box1, box2);

        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - current_center) / self.radius;
        let front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        let (u, v) = get_sphere_uv(outward_normal);

        Some(HitRecord::new(
            p,
            normal,
            self.mat.as_ref(),
            root,
            u,
            v,
            front_face,
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        if let Some(_rec) = self.hit(
            &Ray::new(origin, direction, 0.0),
            Interval::new(0.001, f64::INFINITY),
        ) {
            let dist_squared = (self.center.at(0.0) - origin).length_squared();
            let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
            let solid_angle = 2.0 * f64::consts::PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center.at(0.0) - origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(direction);

        uvw.transform(random_to_sphere(self.radius, distance_squared))
    }
}

fn get_sphere_uv(p: Point3) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z().atan2(p.x())) + f64::consts::PI;

    let u = phi / (2.0 * f64::consts::PI);
    let v = theta / f64::consts::PI;

    (u, v)
}

fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = rand::rng().random::<f64>();
    let r2 = rand::rng().random::<f64>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * f64::consts::PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
