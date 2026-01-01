use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
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

        Some(HitRecord::new(
            p,
            normal,
            self.mat.as_ref(),
            root,
            0.0, //TODO replace later
            0.0, //TODO replace later
            front_face,
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
