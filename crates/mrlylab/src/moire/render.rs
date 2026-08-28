use super::field::Field;
use mrlycore::errors::{value_error, Result};
use mrlycore::ramp::Colorizer;

/// Quantizes a field into colored levels and encodes PNG bytes, or an error at scale zero.
pub fn render(
    field: &Field,
    colorizer: &Colorizer,
    levels: usize,
    symmetric: bool,
    invert: bool,
    scale: usize,
) -> Result<Vec<u8>> {
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
        let c = colorizer.color(bucket + 1, levels);
        rgba[i] = [c.r, c.g, c.b, 255];
    }
    mrlycore::io::png(&rgba, size, size, scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moire::{stack, Combine, Lattice, Spec};
    #[test]
    fn renders_png_bytes() {
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
        let png = render(&f, &Colorizer::fire(), 64, false, false, 2).unwrap();
        assert_eq!(&png[1..4], b"PNG");
    }
    #[test]
    fn png_pixels_stay_pinned() {
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
                3_537_984,
                [255, 255, 220, 255],
                [218, 86, 0, 255],
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
                606_408,
                [220, 40, 40, 255],
                [227, 101, 101, 255],
            ),
        ];
        for (bytes, side, sum, corner, centre) in &cases {
            let (w, h, pixels) = mrlycore::unpng(bytes).unwrap();
            assert_eq!((w, h), (*side, *side));
            let bytes: u64 = pixels.iter().flatten().map(|&b| u64::from(b)).sum();
            assert_eq!(bytes, *sum);
            assert_eq!(pixels[0], *corner);
            assert_eq!(pixels[side * side - 1], *corner);
            assert_eq!(pixels[(side / 2) * side + side / 2], *centre);
        }
    }
}
