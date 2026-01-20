use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
use std::sync::Arc;

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: Arc<dyn Material>) -> Self {
        let u = v1 - v0;
        let v = v2 - v0;
        let bbox = set_bounding_box(v0, v1, v2);
        let n = vec3::cross(u, v);
        let normal = vec3::unit_vector(n);
        let D = vec3::dot(normal, v0);
        let w = n / vec3::dot(n, n);

        Self {
            v0,
            v1,
            v2,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            D,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let denom = vec3::dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.D - vec3::dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.v0;
        let alpha = vec3::dot(self.w, vec3::cross(planar_hitpt_vector, self.v));
        let beta = vec3::dot(self.w, vec3::cross(self.u, planar_hitpt_vector));

        if !is_interior(alpha, beta) {
            return None;
        }

        let front_face = vec3::dot(r.direction(), self.normal) < 0.0;
        let normal = if front_face {
            self.normal
        } else {
            -self.normal
        };

        Some(HitRecord::new(
            intersection,
            normal,
            self.mat.as_ref(),
            t,
            alpha,
            beta,
            front_face,
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

fn set_bounding_box(v0: Point3, v1: Point3, v2: Point3) -> AABB {
    let min = Point3::new(
        v0.x().min(v1.x()).min(v2.x()),
        v0.y().min(v1.y()).min(v2.y()),
        v0.z().min(v1.z()).min(v2.z()),
    );

    let max = Point3::new(
        v0.x().max(v1.x()).max(v2.x()),
        v0.y().max(v1.y()).max(v2.y()),
        v0.z().max(v1.z()).max(v2.z()),
    );

    AABB::from_points(min, max)
}

fn is_interior(a: f64, b: f64) -> bool {
    a >= 0.0 && b >= 0.0 && a + b <= 1.0
}
