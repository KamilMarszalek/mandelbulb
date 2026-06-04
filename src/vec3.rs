use std::ops::{Add, Mul, Sub};

const EPSILON: f64 = 1e-12;
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length < EPSILON {
            return Self::new(0.0, 0.0, 0.0);
        }
        Self::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-12,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_vec_close(actual: Vec3, expected: Vec3) {
        assert_close(actual.x, expected.x);
        assert_close(actual.y, expected.y);
        assert_close(actual.z, expected.z);
    }

    #[test]
    fn length_returns_euclidean_norm() {
        assert_close(Vec3::new(3.0, 4.0, 12.0).length(), 13.0);
    }

    #[test]
    fn normalize_returns_unit_vector() {
        let normalized = Vec3::new(3.0, 0.0, 4.0).normalize();

        assert_close(normalized.length(), 1.0);
        assert_vec_close(normalized, Vec3::new(0.6, 0.0, 0.8));
    }

    #[test]
    fn normalize_handles_zero_length_vector() {
        assert_vec_close(Vec3::new(0.0, 0.0, 0.0).normalize(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_arithmetic_works() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -2.0, 0.5);

        assert_vec_close(a + b, Vec3::new(5.0, 0.0, 3.5));
        assert_vec_close(a - b, Vec3::new(-3.0, 4.0, 2.5));
        assert_vec_close(a * 2.5, Vec3::new(2.5, 5.0, 7.5));
    }

    #[test]
    fn dot_product_works() {
        let a = Vec3::new(1.0, 3.0, -5.0);
        let b = Vec3::new(4.0, -2.0, -1.0);

        assert_close(a.dot_product(&b), 3.0);
    }

    #[test]
    fn cross_product_works() {
        let x_axis = Vec3::new(1.0, 0.0, 0.0);
        let y_axis = Vec3::new(0.0, 1.0, 0.0);

        assert_vec_close(x_axis.cross_product(&y_axis), Vec3::new(0.0, 0.0, 1.0));
    }
}
