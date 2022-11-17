use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use ray_tracing::random_double;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn one() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn new<X,Y,Z> (x: X, y: Y, z: Z) -> Vec3
    where
        X: Into<f64>,
        Y: Into<f64>,
        Z: Into<f64>
    {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max)
        }
    }

    pub fn copy_vector(&mut self, vector: &Vec3) {
        self.x = vector.x;
        self.y = vector.y;
        self.z = vector.z;
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double(0.0, 1.0), random_double(0.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn modulus(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(&v, &n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, eta: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&-uv, &n), 1.0);
        let r_out_perpendicular = eta * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(
            1.0 - r_out_perpendicular.length_squared())) * n;
        r_out_perpendicular + r_out_parallel
    }

    pub fn unit(self) -> Vec3 {
        self / self.modulus()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Copy for Vec3 { }

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}