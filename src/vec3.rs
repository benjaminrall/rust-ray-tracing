use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use ray_tracing::{ random_range };

/// Object to represent a column vector in 3 dimensions
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    /// Returns a zero vector
    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }

    /// Returns a vector with x, y, and z of 1
    pub fn one() -> Vec3 {
        Vec3 { x: 1., y: 1., z: 1. }
    }

    /// Constructs a new vector with given x, y, and z values
    pub fn new (x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Constructs a vector with random components between a given range
    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_range(min, max),
            y: random_range(min, max),
            z: random_range(min, max)
        }
    }

    /// Returns a random vector in a unit sphere
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    /// Returns a random vector in a unit disk
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_range(-1., 1.), random_range(-1., 1.), 0.);
            if p.length_squared() >= 1. {
                continue
            }
            return p;
        }
    }

    /// Returns a random unit vector
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    /// Returns the squared length of a vector
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length of a vector
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    /// Returns the dot product of two vectors
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Returns the cross product of two vectors
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    /// Returns the unit vector
    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    /// Returns true if all components of the vector are near zero
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s
    }
}

/// Allows vectors to be cloned directly by constructing a vector with the same x, y, and z
impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3::new(self.x, self.y, self.z)
    }
}
impl Copy for Vec3 { }

/// Implements vector addition
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

/// Implements vector addition with the += operator
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// Implements negation of vectors using the unary '-'
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

/// Implements vector subtraction
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

/// Implements vector subtraction with the -= operator
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}

/// Implements element-wise multiplication of two vectors
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

/// Implements multiplication of a vector with a scalar value
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

/// Implements multiplication of a scalar value with a vector
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

/// Implements multiplication of a vector by a scalar value with the *= operator
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Implements division of a vector by a scalar value
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

/// Implements division of a vector by a scalar value with the /= operator
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

/// Provides vector formatting and printing
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}