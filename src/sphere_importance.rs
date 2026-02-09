use crate::vec3::{self, Vec3};
use core::f64;

fn f(d: Vec3) -> f64 {
    let cosine_squared = d.z() * d.z();

    cosine_squared
}

fn pdf(d: Vec3) -> f64 {
    1.0 / (4.0 * f64::consts::PI)
}

pub fn calc() {
    let N = 10_000_000;
    let mut sum = 0.0;

    for _ in 0..N {
        let d = vec3::random_unit_vector();
        let f_d = f(d);
        sum += f_d / pdf(d);
    }

    println!("I = {}", (sum / N as f64));
}
