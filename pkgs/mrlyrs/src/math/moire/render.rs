use super::field::Field;
use crate::core::error::{value_error, Result};
use crate::core::image::Image;
use crate::core::ramp::Colorizer;

/// Quantizes a field into colored levels and renders them as an image, each sample a scale by scale block.
///
/// # Errors
///
/// Errors at scale zero.
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
        let c = crate::core::ramp::color(colorizer, bucket + 1, levels);
        rgba[i] = [c.r, c.g, c.b, 255];
    }
    Image::new(size, size, rgba)?.scale(scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::moire::{stack, Combine, Lattice, Spec};
    #[test]
    fn image_pixels_stay_pinned() {
        let f = stack(
            Spec::new(7, 2, 2),
            &[1, 3, 5],
            Combine::Sum,
            1,
            Lattice::Square,
            32,
            &[],
        )
        .unwrap();
        let cases = [
            (
                render(&f, &Colorizer::fire(), 64, false, false, 2).unwrap(),
                64,
                3_230_592,
                [255, 232, 128, 255],
                [205, 91, 41, 255],
            ),
            (
                render(&f, &Colorizer::heat(), 32, true, true, 3).unwrap(),
                96,
                9_073_080,
                [255, 255, 255, 255],
                [214, 214, 214, 255],
            ),
            (
                render(&f, &Colorizer::diverge(), 8, true, false, 1).unwrap(),
                32,
                684_184,
                [255, 61, 64, 255],
                [255, 119, 121, 255],
            ),
        ];
        for (sheet, side, sum, corner, centre) in &cases {
            let pixels = &sheet.colors;
            assert_eq!((sheet.width, sheet.height), (*side, *side));
            let bytes: u64 = pixels.iter().flatten().map(|&b| u64::from(b)).sum();
            assert_eq!(bytes, *sum);
            assert_eq!(pixels[0], *corner);
            assert_eq!(pixels[side * side - 1], *corner);
            assert_eq!(pixels[(side / 2) * side + side / 2], *centre);
        }
    }
}
