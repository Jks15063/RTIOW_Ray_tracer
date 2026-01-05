use crate::vec3::Point3;
use rand::Rng;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_float: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let rand_float: [f64; POINT_COUNT] = std::array::from_fn(|_| rand::rng().random());
        let perm_x = generate_perm();
        let perm_y = generate_perm();
        let perm_z = generate_perm();

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn generate_perm() -> [usize; POINT_COUNT] {
    let mut p: [usize; POINT_COUNT] = std::array::from_fn(|i| i as usize);

    for i in (1..POINT_COUNT).rev() {
        let target = rand::rng().random_range(0..=i);
        p.swap(i, target);
    }

    p
}
