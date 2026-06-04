use pyo3::pyfunction;
use rayon::prelude::*;

use crate::color::{Colorizer, GrayscaleColorizer, RgbColorizer};
use crate::sdf::{estimate_normal, mandelbulb_sdf};
use crate::vec3::Vec3;

const MAX_DISTANCE: f64 = 100.0;
const HIT_EPSILON: f64 = 0.001;
const COLOR_ITERATION_SCALE: f64 = 50.0;

#[pyfunction]
pub fn render_mandelbulb(
    width: usize,
    height: usize,
    power: f64,
    fractal_iterations: usize,
    ray_steps: usize,
    cam_x: f64,
    cam_y: f64,
    cam_z: f64,
    mode: &str,
    parallel: bool,
) -> Vec<u8> {
    let colorizer: Box<dyn Colorizer> = match mode {
        "gray" | "grayscale" => Box::new(GrayscaleColorizer),
        "rgb" => Box::new(RgbColorizer),
        _ => Box::new(RgbColorizer),
    };

    let mut buffer = vec![0u8; width * height * 3];

    let aspect = width as f64 / height as f64;
    let ray_origin = Vec3::new(cam_x, cam_y, cam_z);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let forward = (target - ray_origin).normalize();
    let world_up = Vec3::new(0.0, 1.0, 0.0);
    let right = forward.cross_product(&world_up).normalize();
    let up = right.cross_product(&forward).normalize();
    if parallel {
        buffer.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
            render_pixel(
                i,
                pixel,
                width,
                height,
                aspect,
                power,
                fractal_iterations,
                ray_steps,
                ray_origin,
                forward,
                right,
                up,
                colorizer.as_ref(),
            )
        });
    } else {
        buffer.chunks_mut(3).enumerate().for_each(|(i, pixel)| {
            render_pixel(
                i,
                pixel,
                width,
                height,
                aspect,
                power,
                fractal_iterations,
                ray_steps,
                ray_origin,
                forward,
                right,
                up,
                colorizer.as_ref(),
            )
        });
    }

    buffer
}

fn render_pixel(
    i: usize,
    pixel: &mut [u8],
    width: usize,
    height: usize,
    aspect: f64,
    power: f64,
    fractal_iterations: usize,
    ray_steps: usize,
    ray_origin: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    colorizer: &dyn Colorizer,
) {
    let x = (i % width) as f64;
    let y = (i / width) as f64;

    let x_norm = (2.0 * x / width as f64 - 1.0) * aspect;
    let y_norm = 2.0 * y / height as f64 - 1.0;

    let ray_dir = (right * x_norm + up * y_norm + forward).normalize();

    let mut total_distance = 0.0;
    let mut hit_iters = 0;
    let mut hit = false;

    for step in 0..ray_steps {
        let p = ray_origin + ray_dir * total_distance;
        let sdf_dist = mandelbulb_sdf(p, power, fractal_iterations);

        if sdf_dist < HIT_EPSILON {
            hit_iters = step;
            hit = true;
            break;
        }

        total_distance += sdf_dist;

        if total_distance > MAX_DISTANCE {
            break;
        }
    }

    if !hit {
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
    } else {
        let p = ray_origin + ray_dir * total_distance;
        let normal = estimate_normal(p, power, fractal_iterations);
        let light_dir = ray_dir * -1.0;
        let intensity = normal.dot_product(&light_dir).max(0.0);
        let t = (hit_iters as f64 / COLOR_ITERATION_SCALE).min(1.0);

        let colors = colorizer.get_color(t, intensity);

        pixel[0] = colors[0];
        pixel[1] = colors[1];
        pixel[2] = colors[2];
    }
}
