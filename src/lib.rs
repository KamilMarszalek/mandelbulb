use pyo3::{
    Bound, PyResult, pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use rayon::prelude::*;
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

#[pymodule]
fn mandelbulb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_mandelbulb, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn render_mandelbulb(width: usize, height: usize, power: f64, max_steps: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; width * height * 3];
    buffer.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
        let x = (i % width) as f64;
        let y = (i / width) as f64;
        let ray_origin = Vec3::new(0.0, 0.0, -2.5);
        let aspect = width as f64 / height as f64;
        let x_norm = (2.0 * x / width as f64 - 1.0) * aspect;
        let y_norm = 2.0 * y / height as f64 - 1.0;
        let ray_dir = Vec3::new(x_norm, y_norm, 1.0).normalize();
        let mut total_distance = 0.0;
        for _ in 0..256 {
            let p = ray_origin + ray_dir * total_distance;
            let sdf_dist = mandelbulb_sdf(p, power, max_steps);
            if total_distance > 100.0 || sdf_dist < 0.001 {
                break;
            }
            total_distance += sdf_dist;
        }
        if total_distance > 100.0 {
            pixel.iter_mut().for_each(|v| *v = 0u8);
        } else {
            let p = ray_origin + ray_dir * total_distance;
            let normal = estimate_normal(p, power, max_steps);
            let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();
            let intensity = normal.dot_product(&light_dir).max(0.0);
            pixel
                .iter_mut()
                .for_each(|v| *v = (255.0 * intensity) as u8);
        }
    });
    buffer
}

pub fn estimate_normal(p: Vec3, power: f64, max_steps: usize) -> Vec3 {
    let n_x = mandelbulb_sdf(Vec3::new(p.x + 0.001, p.y, p.z), power, max_steps)
        - mandelbulb_sdf(Vec3::new(p.x - 0.001, p.y, p.z), power, max_steps);
    let n_y = mandelbulb_sdf(Vec3::new(p.x, p.y + 0.001, p.z), power, max_steps)
        - mandelbulb_sdf(Vec3::new(p.x, p.y - 0.001, p.z), power, max_steps);
    let n_z = mandelbulb_sdf(Vec3::new(p.x, p.y, p.z + 0.001), power, max_steps)
        - mandelbulb_sdf(Vec3::new(p.x, p.y, p.z - 0.001), power, max_steps);
    Vec3::new(n_x, n_y, n_z).normalize()
}
