use ray_tracing::{random_double, random_int};
use crate::vec3::Vec3;

const PERLIN_POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct Perlin {
    random_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut random_vec: Vec<Vec3> = vec![];
        for _ in 0..PERLIN_POINT_COUNT {
            random_vec.push(Vec3::random(-1., 1.));
        }

        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();

        Perlin { random_vec, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vec[(
                            self.perm_x[((i + di as i32) & 255) as usize] ^
                            self.perm_y[((j + dj as i32) & 255) as usize] ^
                            self.perm_z[((k + dk as i32) & 255) as usize]
                    ) as usize]
                }
            }
        }

        Perlin::trilinear_interpolate(&c, u, v, w)
    }

    pub fn generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = vec![];
        for i in 0..PERLIN_POINT_COUNT {
            p.push(i as i32);
        }

        Perlin::permute(&mut p);

        p
    }

    pub fn permute(p: &mut Vec<i32>) {
        for i in (1..PERLIN_POINT_COUNT-1).rev() {
            let target: usize = random_int(0, i as i32) as usize;
            let temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    pub fn trilinear_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i * uu + (1. - i) * (1. - uu))
                            * (j * vv + (1. - j) * (1. - vv))
                            * (k * ww + (1. - k) * (1. - ww))
                            * Vec3::dot(&c[i as usize][j as usize][k as usize], &weight_v);
                }
            }
        }

        accum
    }

    pub fn turbulence(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.;
        let mut temp_p = p.clone();
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }
}