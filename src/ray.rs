use crate::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn empty() -> Self {
        Ray { origin: Vec3::zero(), direction: Vec3::zero() }
    }

    pub fn set(&mut self, origin: Vec3, direction: Vec3) {
        self.origin = origin;
        self.direction = direction;
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }
}