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

#[cfg(test)]
mod tests {
    use super::{Colorizer, GrayscaleColorizer, RgbColorizer};

    #[test]
    fn rgb_colorizer_maps_t_and_intensity_to_red_green_channels() {
        let colorizer = RgbColorizer;

        assert_eq!(colorizer.get_color(0.25, 0.5), [31, 95, 0]);
    }

    #[test]
    fn grayscale_colorizer_sets_all_channels_to_same_intensity() {
        let colorizer = GrayscaleColorizer;

        assert_eq!(colorizer.get_color(0.75, 0.5), [127, 127, 127]);
    }
}
