use std::ops::{Add, Mul, Sub};
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const ESCAPE: f64 = 2.0;
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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

pub fn mandelbulb_sdf(point: Vec3, power: f64, max_steps: usize) -> f64 {
    let mut z = point;
    let mut dr = 1.0;
    for _ in 0..max_steps {
        let r = z.length();
        if r > ESCAPE {
            break;
        }
        dr = r.powf(power - 1.0) * power * dr + 1.0;
        let theta = power * (z.z / r).acos();
        let phi = power * z.y.atan2(z.x);
        let zr = r.powf(power);
        let new_z = Vec3::new(
            zr * theta.sin() * phi.cos(),
            zr * theta.sin() * phi.sin(),
            zr * theta.cos(),
        );

        z = point + new_z;
    }
    let r = z.length();
    0.5 * r.ln() * (r / dr)
}
