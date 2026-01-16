use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use rand::Rng;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Box<dyn Material>,
}

impl ConstantMedium {
    pub fn from_texture(boundary: Box<dyn Hittable>, density: f64, tex: Box<dyn Texture>) -> Self {
        let neg_inv_density = -1.0 / density;

        Self {
            boundary,
            neg_inv_density,
            phase_function: Box::new(Isotropic::from_texture(tex)),
        }
    }

    pub fn from_color(boundary: Box<dyn Hittable>, density: f64, albedo: Color) -> Self {
        let neg_inv_density = -1.0 / density;

        Self {
            boundary,
            neg_inv_density,
            phase_function: Box::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let rec1 = self.boundary.hit(r, Interval::UNIVERSE)?;
        let rec2 = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY))?;

        let mut t1 = rec1.t.max(ray_t.min);
        let t2 = rec2.t.min(ray_t.max);

        if t1 >= t2 {
            return None;
        }

        if t1 < 0.0 {
            t1 = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance: f64 = self.neg_inv_density * rand::rng().random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t1 + hit_distance / ray_length;

        Some(HitRecord::new(
            r.at(t),
            Vec3::new(1.0, 0.0, 0.0),
            self.phase_function.as_ref(),
            t,
            0.0,
            0.0,
            true,
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
