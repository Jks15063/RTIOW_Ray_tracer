use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    format!("{} {} {}", rbyte, gbyte, bbyte)
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}
