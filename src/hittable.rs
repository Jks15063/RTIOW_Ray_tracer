use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
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

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;

        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        self.object.hit(&offset_r, ray_t).map(|mut rec| {
            rec.p += self.offset;
            rec
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
}
