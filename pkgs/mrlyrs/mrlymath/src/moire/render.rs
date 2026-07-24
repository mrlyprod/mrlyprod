use super::field::Field;
use mrlycore::errors::{value_error, Result};
use mrlycore::ramp::Colorizer;

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
    fn golden_parity_with_pre_migration_bytes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
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
        let cases: Vec<Vec<u8>> = vec![
            render(&f, &Colorizer::fire(), 64, false, false, 2).unwrap(),
            render(&f, &Colorizer::heat(), 32, true, true, 3).unwrap(),
            render(&f, &Colorizer::diverge(), 8, true, false, 1).unwrap(),
        ];
        let pins: [(usize, u64); 3] = [
            (1407, 8353179821263645952),
            (2064, 17222011625158757863),
            (791, 10207858607915258133),
        ];
        for (png, (len, hash)) in cases.iter().zip(pins) {
            let mut h = DefaultHasher::new();
            png.hash(&mut h);
            assert_eq!(png.len(), len, "byte length drifted from pre-io::png bytes");
            assert_eq!(h.finish(), hash, "bytes drifted from pre-io::png rendering");
        }
    }
}
