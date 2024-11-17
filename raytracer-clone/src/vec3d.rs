use rand::Rng;
use serde::{Serialize, Deserialize};
use std::cmp::PartialEq;
use std::f64;
use std::ops::{Add, Mul, Neg, Sub};

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn random(min: f64, max: f64) -> Vec3D {
        let mut rng = rand::thread_rng();

        Vec3D::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3D {
        let length = self.length();
        Vec3D::new(
            self.x / length,
            self.y / length,
            self.z / length,
        )
    }

    pub fn distance(&self, other: &Vec3D) -> f64 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        let dz = self.z - other.z();

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn dot(&self, other: &Vec3D) -> f64 {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }

    pub fn cross(&self, other: &Vec3D) -> Vec3D {
        Vec3D::new(
            self.y * other.z() - self.z * other.y(),
            self.z * other.x() - self.x * other.z(),
            self.x * other.y() - self.y * other.x(),
        )
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, other: Vec3D) -> Vec3D {
        Vec3D::new(
            self.x + other.x(),
            self.y + other.y(),
            self.z + other.z(),
        )
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, other: Vec3D) -> Vec3D {
        Vec3D::new(
            self.x - other.x(),
            self.y - other.y(),
            self.z - other.z(),
        )
    }
}

impl Neg for Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Vec3D {
        Vec3D::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, scalar: f64) -> Vec3D {
        Vec3D::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

impl Mul<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn mul(self, other: Vec3D) -> Vec3D {
        Vec3D::new(
            self.x * other.x(),
            self.y * other.y(),
            self.z * other.z(),
        )
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, other: &Vec3D) -> bool {
        self.x == other.x() && self.y == other.y() && self.z == other.z()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3D::new(1.0, 2.0, 2.0);
        assert_approx_eq!(v.length(), 3.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3D::new(1.0, 2.0, 2.0);
        let unit_v = v.unit_vector();
        assert_approx_eq!(unit_v.length(), 1.0);
    }

    #[test]
    fn test_distance() {
        let v1 = Vec3D::new(1.0, 2.0, 3.0);
        let v2 = Vec3D::new(4.0, 5.0, 6.0);
        assert_approx_eq!(v1.distance(&v2), 5.196152422706632);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3D::new(1.0, 2.0, 3.0);
        let v2 = Vec3D::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3D::new(1.0, 2.0, 3.0);
        let v2 = Vec3D::new(4.0, 5.0, 6.0);
        let cross_product = v1.cross(&v2);
        assert_eq!(cross_product, Vec3D::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_add() {
        let v1 = Vec3D::new(1.0, 2.0, 3.0);
        let v2 = Vec3D::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum, Vec3D::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3D::new(4.0, 5.0, 6.0);
        let v2 = Vec3D::new(1.0, 2.0, 3.0);
        let difference = v1 - v2;
        assert_eq!(difference, Vec3D::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        let neg_v = -v;
        assert_eq!(neg_v, Vec3D::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        let product = v * 2.0;
        assert_eq!(product, Vec3D::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vector() {
        let v1 = Vec3D::new(1.0, 2.0, 3.0);
        let v2 = Vec3D::new(4.0, 5.0, 6.0);
        let product = v1 * v2;
        assert_eq!(product, Vec3D::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_partial_eq() {
        let v1 = Vec3D::new(1.0, 2.0, 3.0);
        let v2 = Vec3D::new(1.0, 2.0, 3.0);
        let v3 = Vec3D::new(4.0, 5.0, 6.0);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
}
