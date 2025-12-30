use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
            bbox: AABB::new(),
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
}
