use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Copy, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }

    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };

        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };

        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        Self { x, y, z }
    }

    pub fn from_aabb(box0: AABB, box1: AABB) -> Self {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        Self { x, y, z }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 {
            return &self.y;
        } else if n == 2 {
            return &self.z;
        } else {
            return &self.x;
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        let mut ray_t_min = ray_t.min;
        let mut ray_t_max = ray_t.max;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t_min {
                    ray_t_min = t0;
                }
                if t1 < ray_t_max {
                    ray_t_max = t1;
                }
            } else {
                if t1 > ray_t_min {
                    ray_t_min = t1;
                }
                if t0 < ray_t_max {
                    ray_t_max = t0;
                }
            }

            if ray_t_max <= ray_t_min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        if x_size > y_size && x_size > z_size {
            0
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }
}
