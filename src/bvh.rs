use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;

pub enum BVHNode {
    Leaf {
        object: Box<dyn Hittable>,
        bbox: AABB,
    },
    Internal {
        left: Box<BVHNode>,
        right: Box<BVHNode>,
        bbox: AABB,
    },
}

impl BVHNode {
    pub fn from_list(list: HittableList) -> Self {
        Self::node(list.objects)
    }

    pub fn node(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut bbox = AABB::empty();
        for object in objects.iter() {
            bbox = AABB::from_aabb(bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();

        match objects.len() {
            1 => {
                let object = objects.pop().unwrap();
                let bbox = object.bounding_box();

                Self::Leaf { object, bbox }
            }
            2 => {
                let left = objects.pop().unwrap();
                let right = objects.pop().unwrap();
                let left_bbox = left.bounding_box();
                let right_bbox = right.bounding_box();

                Self::Internal {
                    left: Box::new(Self::Leaf {
                        object: left,
                        bbox: left_bbox,
                    }),
                    right: Box::new(Self::Leaf {
                        object: right,
                        bbox: right_bbox,
                    }),
                    bbox: AABB::from_aabb(left_bbox, right_bbox),
                }
            }
            _ => {
                objects.sort_by(|a, b| {
                    let a_min = a.bounding_box().axis_interval(axis).min;
                    let b_min = b.bounding_box().axis_interval(axis).min;
                    a_min.partial_cmp(&b_min).unwrap()
                });
                let mid = objects.len() / 2;
                let right_half = objects.split_off(mid);
                let left = Self::node(objects);
                let right = Self::node(right_half);
                let bbox = AABB::from_aabb(left.bounding_box(), right.bounding_box());

                return Self::Internal {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox,
                };
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        match self {
            BVHNode::Leaf { object, bbox } => {
                if !bbox.hit(r, ray_t) {
                    return None;
                }
                object.hit(r, ray_t)
            }
            BVHNode::Internal { left, right, bbox } => {
                if !bbox.hit(r, ray_t) {
                    return None;
                }
                let mut closest_so_far = ray_t.max;
                let mut closest_hit: Option<HitRecord> = None;

                if let Some(rec) = left.hit(r, ray_t) {
                    closest_so_far = rec.t;
                    closest_hit = Some(rec);
                }

                if let Some(rec) = right.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                    closest_hit = Some(rec);
                }

                closest_hit
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            BVHNode::Leaf { bbox, .. } => *bbox,
            BVHNode::Internal { bbox, .. } => *bbox,
        }
    }
}
