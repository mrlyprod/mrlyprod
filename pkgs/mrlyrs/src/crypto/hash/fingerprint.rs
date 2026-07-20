use super::hasher::Digest;
use crate::core::colors::Color;
use crate::core::errors::{value_error, Result};
use crate::core::ramp::Colorizer;

const BACKGROUND: Color = Color::rgb(13, 17, 23);

pub fn fingerprint_cell(digest: &Digest, side: usize) -> Vec<u8> {
    let needed = side * side;
    let bits = &digest.bits;
    let get = |i: usize| -> u8 {
        if bits.is_empty() {
            0
        } else {
            bits[i % bits.len()] & 1
        }
    };
    let half = side.div_ceil(2);
    let mut grid = vec![0u8; needed];
    for r in 0..side {
        for c in 0..half {
            let v = get(r * half + c);
            grid[r * side + c] = v;
            grid[r * side + (side - 1 - c)] = v;
        }
    }
    grid
}

fn ink(digest: &Digest) -> Color {
    let bytes = digest.to_bytes();
    let b0 = *bytes.first().unwrap_or(&0) as u32;
    let b1 = *bytes.get(1).unwrap_or(&0) as u32;
    let hue = ((b0 << 8 | b1) % 360) as f64;
    hsl_to_color(hue, 0.62, 0.58)
}

fn hsl_to_color(h: f64, s: f64, l: f64) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color::rgb(
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

pub fn fingerprint(digest: &Digest, side: usize, scale: usize) -> Result<Vec<u8>> {
    if side == 0 || scale == 0 {
        return value_error("fingerprint side and scale must be >= 1.");
    }
    let grid = fingerprint_cell(digest, side);
    let colorizer = Colorizer::two_tone(BACKGROUND, ink(digest));
    let colors = crate::io::colorize(&grid, &colorizer, 1);
    crate::io::png(&colors, side, side, scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash::{digest, Config};
    #[test]
    fn fingerprint_grid_is_symmetric() {
        let d = digest(b"carlo", &Config::default()).unwrap();
        let side = 8;
        let grid = fingerprint_cell(&d, side);
        for r in 0..side {
            for c in 0..side {
                assert_eq!(
                    grid[r * side + c],
                    grid[r * side + (side - 1 - c)],
                    "not mirrored at ({r},{c})"
                );
            }
        }
    }
    #[test]
    fn fingerprint_is_deterministic_and_png() {
        let d = digest(b"moire", &Config::default()).unwrap();
        let a = fingerprint(&d, 8, 16).unwrap();
        let b = fingerprint(&d, 8, 16).unwrap();
        assert_eq!(a, b);
        assert_eq!(&a[1..4], b"PNG");
    }
    #[test]
    fn different_digests_give_different_faces() {
        let a = digest(b"alice", &Config::default()).unwrap();
        let b = digest(b"alicf", &Config::default()).unwrap();
        assert_ne!(fingerprint_cell(&a, 8), fingerprint_cell(&b, 8));
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::crypto::hash::{digest, Config};
    #[test]
    fn golden_parity_with_pre_io_bytes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let cases: Vec<Vec<u8>> = vec![
            fingerprint(&digest(b"carlo", &Config::default()).unwrap(), 8, 16).unwrap(),
            fingerprint(&digest(b"mrly", &Config::default()).unwrap(), 5, 3).unwrap(),
            fingerprint(&digest(b"", &Config::default()).unwrap(), 12, 1).unwrap(),
        ];
        let pins: [(usize, u64); 3] = [
            (3469, 16288488783745636041),
            (343, 14718624200827264728),
            (535, 16076660125785947179),
        ];
        for (png, (len, hash)) in cases.iter().zip(pins) {
            let mut h = DefaultHasher::new();
            png.hash(&mut h);
            assert_eq!(png.len(), len, "byte length drifted from pre-io::png bytes");
            assert_eq!(h.finish(), hash, "bytes drifted from pre-io::png rendering");
        }
    }
}
