use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
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
        let outward_normal = (p - self.center) / self.radius;
        let front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord::new(
            p,
            normal,
            self.mat.as_ref(),
            root,
            front_face,
        ))
    }
}
