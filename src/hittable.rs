use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use core::f64;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
}

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

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        self.object.hit(&rotated_r, ray_t).map(|mut rec| {
            rec.p = Point3::new(
                (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
                rec.p.y(),
                (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
            );

            rec.normal = Vec3::new(
                (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
                rec.normal.y(),
                (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
            );

            rec
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}
