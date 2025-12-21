use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.000, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    format!("{} {} {}", rbyte, gbyte, bbyte)
}
