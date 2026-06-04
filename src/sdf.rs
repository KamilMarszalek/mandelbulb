use crate::vec3::Vec3;

pub const ESCAPE: f64 = 2.0;
const NORMAL_EPSILON: f64 = 0.001;

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

pub fn estimate_normal(p: Vec3, power: f64, max_steps: usize) -> Vec3 {
    let n_x = mandelbulb_sdf(Vec3::new(p.x + NORMAL_EPSILON, p.y, p.z), power, max_steps)
        - mandelbulb_sdf(Vec3::new(p.x - NORMAL_EPSILON, p.y, p.z), power, max_steps);

    let n_y = mandelbulb_sdf(Vec3::new(p.x, p.y + NORMAL_EPSILON, p.z), power, max_steps)
        - mandelbulb_sdf(Vec3::new(p.x, p.y - NORMAL_EPSILON, p.z), power, max_steps);

    let n_z = mandelbulb_sdf(Vec3::new(p.x, p.y, p.z + NORMAL_EPSILON), power, max_steps)
        - mandelbulb_sdf(Vec3::new(p.x, p.y, p.z - NORMAL_EPSILON), power, max_steps);

    Vec3::new(n_x, n_y, n_z).normalize()
}
