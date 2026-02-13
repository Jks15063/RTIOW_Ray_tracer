use core::f64;

use rand::Rng;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Quad {
    Q: Point3, //corner point of quad
    u: Vec3,   // edge vector
    v: Vec3,   // edge vector
    w: Vec3,   // used for hit test
    mat: Box<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64, // plane equation constant
    area: f64,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Self {
        let bbox = set_bounding_box(Q, u, v);
        let n = vec3::cross(u, v);
        let normal = vec3::unit_vector(n);
        let D = vec3::dot(normal, Q);
        let w = n / vec3::dot(n, n);
        let area = n.length();

        Self {
            Q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            D,
            area,
        }
    }
}

impl Hittable for Quad {
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
        let planar_hitpt_vector = intersection - self.Q;
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

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        if let Some(rec) = self.hit(
            &Ray::new(origin, direction, 0.0),
            Interval::new(0.001, f64::INFINITY),
        ) {
            let distance_squared = rec.t * rec.t * direction.length_squared();
            let cosine = (vec3::dot(direction, rec.normal) / direction.length()).abs();

            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let p = self.Q
            + (rand::rng().random::<f64>() * self.u)
            + (rand::rng().random::<f64>() * self.v);

        p - origin
    }
}

pub fn make_box<F>(a: Point3, b: Point3, mut make_mat: F) -> Box<dyn Hittable>
where
    F: FnMut() -> Box<dyn Material>,
{
    let mut sides = HittableList::new();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    //front
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        make_mat(),
    )));

    //right
    sides.add(Box::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        make_mat(),
    )));

    //back
    sides.add(Box::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        make_mat(),
    )));

    //left
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        make_mat(),
    )));

    //top
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        make_mat(),
    )));

    //bottom
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        make_mat(),
    )));

    Box::new(sides)
}

fn set_bounding_box(Q: Point3, u: Vec3, v: Vec3) -> AABB {
    let bbox_diagonal1 = AABB::from_points(Q, Q + u + v);
    let bbox_diagonal2 = AABB::from_points(Q + u, Q + v);

    AABB::from_aabb(bbox_diagonal1, bbox_diagonal2)
}

fn is_interior(a: f64, b: f64) -> bool {
    let unit_interval = Interval::new(0.0, 1.0);

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }

    true
}
