use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64, //distance along the ray the hit occurs
    pub u: f64, //texture coords
    pub v: f64, //texture coords
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        mat: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
}

// pub fn set_face_normal(r: &Ray, outward_normal: Vec3) -> Vec3 {
//     let front_face = vec3::dot(r.direction(), outward_normal) < 0.0;

//     if front_face {
//         outward_normal
//     } else {
//         -outward_normal
//     }
// }
