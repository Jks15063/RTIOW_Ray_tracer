use crate::color::Color;
use crate::interval::Interval;
use crate::texture::Texture;
use crate::vec3::Point3;
use image::{GenericImageView, Pixel};

pub struct ImageTexture {
    image: image::DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = image::open(filename).expect("Failed to load image");

        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = ((u * self.image.width() as f64) as u32).min(self.image.width() - 1);
        let j = ((v * self.image.height() as f64) as u32).min(self.image.width() - 1);
        let pixel = self.image.get_pixel(i, j);
        let [r, g, b, _] = pixel.0;

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * r as f64,
            color_scale * g as f64,
            color_scale * b as f64,
        )
    }
}
