use crate::vec3::Vec3;

pub const ESCAPE: f64 = 2.0;
const NORMAL_EPSILON: f64 = 0.001;
const SDF_EPSILON: f64 = 1e-12;

pub fn mandelbulb_sdf(point: Vec3, power: f64, max_steps: usize) -> f64 {
    let mut z = point;
    let mut dr = 1.0;

    for _ in 0..max_steps {
        let r = z.length();

        if r > ESCAPE {
            break;
        }

        let safe_r = r.max(SDF_EPSILON);
        dr = safe_r.powf(power - 1.0) * power * dr + 1.0;

        let theta = power * (z.z / safe_r).clamp(-1.0, 1.0).acos();
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
    if r < SDF_EPSILON || dr.abs() < SDF_EPSILON {
        return 0.0;
    }

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

#[cfg(test)]
mod tests {
    use super::{estimate_normal, mandelbulb_sdf};
    use crate::vec3::Vec3;

    #[test]
    fn mandelbulb_sdf_returns_finite_values_for_representative_points() {
        let points = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.25, -0.1, 0.4),
            Vec3::new(1.5, 0.5, -0.25),
            Vec3::new(3.0, 0.0, 0.0),
        ];

        for point in points {
            let distance = mandelbulb_sdf(point, 8.0, 8);
            assert!(distance.is_finite(), "distance was {distance} for {point:?}");
        }
    }

    #[test]
    fn estimate_normal_returns_finite_components() {
        let normal = estimate_normal(Vec3::new(0.75, 0.25, -0.5), 8.0, 8);

        assert!(normal.x.is_finite());
        assert!(normal.y.is_finite());
        assert!(normal.z.is_finite());
    }
}
