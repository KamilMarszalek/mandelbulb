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

pub trait Colorizer: Sync + Send {
    fn get_color(&self, t: f64, intensity: f64) -> [u8; 3];
}

pub struct RgbColorizer;

impl Colorizer for RgbColorizer {
    fn get_color(&self, t: f64, intensity: f64) -> [u8; 3] {
        [
            (255.0 * intensity * t) as u8,
            (255.0 * intensity * (1.0 - t)) as u8,
            0,
        ]
    }
}

pub struct GrayscaleColorizer;

impl Colorizer for GrayscaleColorizer {
    fn get_color(&self, _: f64, intensity: f64) -> [u8; 3] {
        let color = (255.0 * intensity) as u8;
        [color, color, color]
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
pub fn render_mandelbulb(
    width: usize,
    height: usize,
    power: f64,
    max_steps: usize,
    cam_x: f64,
    cam_y: f64,
    cam_z: f64,
    mode: &str,
) -> Vec<u8> {
    let colorizer: Box<dyn Colorizer> = match mode {
        "gray" | "grayscale" => Box::new(GrayscaleColorizer),
        _ => Box::new(RgbColorizer),
    };
    let mut buffer = vec![0u8; width * height * 3];
    buffer.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
        let x = (i % width) as f64;
        let y = (i / width) as f64;
        let aspect = width as f64 / height as f64;
        let x_norm = (2.0 * x / width as f64 - 1.0) * aspect;
        let y_norm = 2.0 * y / height as f64 - 1.0;
        let ray_origin = Vec3::new(cam_x, cam_y, cam_z);
        let target = Vec3::new(0.0, 0.0, 0.0);
        let forward = (target - ray_origin).normalize();
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross_product(&world_up).normalize();
        let up = right.cross_product(&forward).normalize();
        let ray_dir = (right * x_norm + up * y_norm + forward * 1.0).normalize();
        let mut total_distance = 0.0;
        let mut hit_iters = 0;
        for step in 0..256 {
            let p = ray_origin + ray_dir * total_distance;
            let sdf_dist = mandelbulb_sdf(p, power, max_steps);
            if total_distance > 100.0 {
                break;
            }
            if sdf_dist < 0.001 {
                hit_iters = step;
                break;
            }
            total_distance += sdf_dist;
        }
        if total_distance > 100.0 {
            pixel.iter_mut().for_each(|v| *v = 0u8);
        } else {
            let p = ray_origin + ray_dir * total_distance;
            let normal = estimate_normal(p, power, max_steps);
            let light_dir = ray_dir * (-1.0);
            let intensity = normal.dot_product(&light_dir).max(0.0);
            let t = (hit_iters as f64 / 50.0).min(1.0);
            let colors = colorizer.get_color(t, intensity);
            pixel[0] = colors[0];
            pixel[1] = colors[1];
            pixel[2] = colors[2];
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
