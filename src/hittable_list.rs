use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use rand::Rng;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
            bbox: AABB::empty(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        let object_box = object.bounding_box();
        self.objects.push(object);
        self.bbox = AABB::from_aabb(self.bbox, object_box);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut closest_so_far = ray_t.max;
        let mut closest_hit: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                closest_hit = Some(rec);
            }
        }

        closest_hit
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: crate::vec3::Point3, direction: crate::vec3::Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;

        self.objects.iter().fold(0.0, |sum, object| {
            sum + weight * object.pdf_value(origin, direction)
        })
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let obj_len = self.objects.len();
        let rand_index = rand::rng().random_range(0..obj_len);

        self.objects[rand_index].random(origin)
    }
}
