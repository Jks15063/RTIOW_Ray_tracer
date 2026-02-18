use core::f64;

use rand::Rng;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::Pdf;
use crate::pdf::{CosinePdf, SpherePdf};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{self, Point3};

pub enum ScatterRecord {
    Pdf {
        attenuation: Color,
        pdf_ptr: Box<dyn Pdf>,
    },
    SkipPdf {
        attenuation: Color,
        ray: Ray,
    },
}

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _r_in: Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord, _scattered: Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    tex: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Box<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Box::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        let pdf_ptr = Box::new(CosinePdf::new(rec.normal));
        Some(ScatterRecord::Pdf {
            attenuation,
            pdf_ptr,
        })
    }

    fn scattering_pdf(&self, _r_in: Ray, rec: &HitRecord, scattered: Ray) -> f64 {
        let cos_theta = vec3::dot(rec.normal, vec3::unit_vector(scattered.direction()));

        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / f64::consts::PI
        }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(r_in.direction(), rec.normal);
        let reflected = vec3::unit_vector(reflected) + (self.fuzz * vec3::random_unit_vector());
        let attenuation = self.albedo;
        let ray = Ray::new(rec.p, reflected, r_in.time());

        Some(ScatterRecord::SkipPdf { attenuation, ray })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand::rng().random() {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, ri)
        };

        let ray = Ray::new(rec.p, direction, r_in.time());

        Some(ScatterRecord::SkipPdf { attenuation, ray })
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index).powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct DiffuseLight {
    tex: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Box<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Box::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        } else {
            self.tex.value(u, v, p)
        }
    }
}

pub struct Isotropic {
    tex: Box<dyn Texture>,
    emit: bool,
}

impl Isotropic {
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Box::new(SolidColor::new(albedo)),
            emit: false,
        }
    }

    pub fn from_texture(tex: Box<dyn Texture>) -> Self {
        Self { tex, emit: false }
    }

    pub fn from_color_emit(albedo: Color) -> Self {
        Self {
            tex: Box::new(SolidColor::new(albedo)),
            emit: true,
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        let pdf_ptr = Box::new(SpherePdf {});

        Some(ScatterRecord::Pdf {
            attenuation,
            pdf_ptr,
        })
    }

    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord, _scattered: Ray) -> f64 {
        1.0 / (4.0 * f64::consts::PI)
    }

    fn emitted(&self, _r_in: Ray, _rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if self.emit {
            self.tex.value(u, v, p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}
