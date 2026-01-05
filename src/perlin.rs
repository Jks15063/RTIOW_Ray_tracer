use crate::vec3::{self, Point3, Vec3};
use rand::Rng;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let rand_vec: [Vec3; POINT_COUNT] = std::array::from_fn(|_| vec3::random_unit_vector());
        let perm_x = generate_perm();
        let perm_y = generate_perm();
        let perm_z = generate_perm();

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2_i32 {
            for dj in 0..2_i32 {
                for dk in 0..2_i32 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]]
                }
            }
        }
        trilinear_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * vec3::dot(c[i][j][k], weight_v);
            }
        }
    }

    accum
}

fn generate_perm() -> [usize; POINT_COUNT] {
    let mut p: [usize; POINT_COUNT] = std::array::from_fn(|i| i as usize);

    for i in (1..POINT_COUNT).rev() {
        let target = rand::rng().random_range(0..=i);
        p.swap(i, target);
    }

    p
}
