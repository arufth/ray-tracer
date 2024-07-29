use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::utils;

#[derive(Debug, Copy, Clone)]

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions
        let s = 1e-8_f64;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }

    pub fn dot(lhs: &Vector3, rhs: &Vector3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vector3, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn unit_vector(vector: &Vector3) -> Self {
        vector / vector.length()
    }

    pub fn random() -> Self {
        Self {
            x: utils::canonical_random_number(),
            y: utils::canonical_random_number(),
            z: utils::canonical_random_number(),
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self {
            x: utils::random_number_in_range(min, max),
            y: utils::random_number_in_range(min, max),
            z: utils::random_number_in_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        Vector3::unit_vector(&Vector3::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
        let on_unit_sphere = Vector3::random_unit_vector();
        if Vector3::dot(&on_unit_sphere, &normal) > 0.0 {
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Self {
        v - &(2.0 * Vector3::dot(v, n) * n)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
