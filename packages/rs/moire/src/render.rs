use super::field::Field;
use mrly_color::ramp::Colorizer;
use mrly_error::{Result, value_error};
use mrly_image::Image;

pub fn render(
    field: &Field,
    colorizer: &Colorizer,
    levels: usize,
    symmetric: bool,
    invert: bool,
    scale: usize,
) -> Result<Image> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    let levels = levels.max(2);
    let size = field.size;
    let norm = field.normalized(symmetric);
    let max_val = levels - 1;
    let mut rgba = vec![[0u8; 4]; size * size];
    for (i, &v) in norm.iter().enumerate() {
        let t = if invert { 1.0 - v } else { v };
        let bucket = ((t * max_val as f32).round() as usize).min(max_val);
        let c = mrly_color::ramp::color(colorizer, bucket + 1, levels);
        rgba[i] = [c.r, c.g, c.b, 255];
    }
    Image::new(size, size, rgba)?.scale(scale)
}
