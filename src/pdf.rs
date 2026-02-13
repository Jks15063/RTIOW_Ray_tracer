use rand::Rng;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::vec3::{self, Point3, Vec3};
use core::f64;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        vec3::random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = vec3::dot(vec3::unit_vector(direction), self.uvw.w());

        (cosine_theta / f64::consts::PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(vec3::random_cosine_direction())
    }
}

pub struct HittablePdf<'a> {
    objects: &'a dyn Hittable,
    origin: Point3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}

pub struct MixturePdf<'a> {
    p0: &'a dyn Pdf,
    p1: Box<dyn Pdf>,
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: Box<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if rand::rng().random::<f64>() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
