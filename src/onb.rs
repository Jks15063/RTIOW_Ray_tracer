use crate::vec3::{self, Vec3};

pub struct Onb {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Onb {
    pub fn new(n: Vec3) -> Self {
        let w = vec3::unit_vector(n);
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = vec3::unit_vector(vec3::cross(w, a));
        let u = vec3::cross(w, v);

        Self { u, v, w }
    }

    pub fn u(&self) -> Vec3 {
        self.u
    }

    pub fn v(&self) -> Vec3 {
        self.v
    }

    pub fn w(&self) -> Vec3 {
        self.w
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        (v.x() * self.u()) + (v.y() * self.v()) + (v.z() * self.w())
    }
}
