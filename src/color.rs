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
