use super::errors::{value_error, Result};

/// The way a resampling weighs the source pixels it reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter {
    /// The single nearest source pixel, palettes and hard edges kept.
    Nearest,
    /// The linear blend of the four source pixels around the target centre.
    Linear,
    /// The mean of the source box the target pixel covers.
    Box,
}

/// The height of an equilateral triangle over its side, the squash a hex rendering wears.
pub const HEX_RATIO: f64 = 0.866_025_403_784_438_6;

/// Returns the size a hex rendering wears, the named axis squashed by the triangle ratio.
///
/// ```
/// assert_eq!(mrlycore::resample::hex_size(100, 100, true), (86, 100));
/// assert_eq!(mrlycore::resample::hex_size(100, 100, false), (100, 86));
/// ```
pub fn hex_size(width: usize, height: usize, vertical: bool) -> (usize, usize) {
    let squash = |value: usize| ((value as f64 * HEX_RATIO) as usize).max(1);
    match vertical {
        true => (squash(width), height),
        false => (width, squash(height)),
    }
}

/// Squashes rgba pixels to the hex aspect, returning the new width, height and pixels.
pub fn hex_fit(
    pixels: &[[u8; 4]],
    width: usize,
    height: usize,
    vertical: bool,
    filter: Filter,
) -> Result<(usize, usize, Vec<[u8; 4]>)> {
    let (out_w, out_h) = hex_size(width, height, vertical);
    let out = resample(pixels, width, height, out_w, out_h, filter)?;
    Ok((out_w, out_h, out))
}

/// Resamples rgba pixels to a new size, or an error on an empty side or a length mismatch.
///
/// ```
/// let pixels = [[255, 0, 0, 255], [0, 0, 255, 255]];
/// let filter = mrlycore::resample::Filter::Nearest;
/// let wide = mrlycore::resample::resample(&pixels, 2, 1, 4, 1, filter).unwrap();
/// assert_eq!(wide, vec![pixels[0], pixels[0], pixels[1], pixels[1]]);
/// ```
pub fn resample(
    pixels: &[[u8; 4]],
    width: usize,
    height: usize,
    out_w: usize,
    out_h: usize,
    filter: Filter,
) -> Result<Vec<[u8; 4]>> {
    if width == 0 || height == 0 || out_w == 0 || out_h == 0 {
        return value_error("resample sides must be at least 1.");
    }
    if pixels.len() != width * height {
        return value_error("pixels length must equal width * height.");
    }
    if (out_w, out_h) == (width, height) {
        return Ok(pixels.to_vec());
    }
    Ok(match filter {
        Filter::Nearest => nearest(pixels, width, height, out_w, out_h),
        Filter::Linear => linear(pixels, width, height, out_w, out_h),
        Filter::Box => boxed(pixels, width, height, out_w, out_h),
    })
}

/// Draws every source element as a scale by scale block, growing both sides by scale.
///
/// ```
/// let out = mrlycore::resample::block(&[1u8, 2], 2, 1, 2);
/// assert_eq!(out, vec![1, 1, 2, 2, 1, 1, 2, 2]);
/// ```
pub fn block<T: Copy>(src: &[T], width: usize, height: usize, scale: usize) -> Vec<T> {
    if scale == 1 {
        return src.to_vec();
    }
    let mut out = Vec::with_capacity(width * height * scale * scale);
    for y in 0..height {
        let start = out.len();
        for x in 0..width {
            for _ in 0..scale {
                out.push(src[y * width + x]);
            }
        }
        let row = start..out.len();
        for _ in 1..scale {
            out.extend_from_within(row.clone());
        }
    }
    out
}

fn nearest(
    pixels: &[[u8; 4]],
    width: usize,
    height: usize,
    out_w: usize,
    out_h: usize,
) -> Vec<[u8; 4]> {
    let mut out = Vec::with_capacity(out_w * out_h);
    for y in 0..out_h {
        let row = y * height / out_h * width;
        for x in 0..out_w {
            out.push(pixels[row + x * width / out_w]);
        }
    }
    out
}

fn linear(
    pixels: &[[u8; 4]],
    width: usize,
    height: usize,
    out_w: usize,
    out_h: usize,
) -> Vec<[u8; 4]> {
    let span = |target: usize, out: usize, source: usize| {
        let centre = (target as f64 + 0.5) * source as f64 / out as f64 - 0.5;
        let clamped = centre.clamp(0.0, (source - 1) as f64);
        let low = clamped.floor() as usize;
        (low, (low + 1).min(source - 1), clamped - low as f64)
    };
    let mut out = Vec::with_capacity(out_w * out_h);
    for y in 0..out_h {
        let (top, bottom, dy) = span(y, out_h, height);
        for x in 0..out_w {
            let (left, right, dx) = span(x, out_w, width);
            let corners = [
                pixels[top * width + left],
                pixels[top * width + right],
                pixels[bottom * width + left],
                pixels[bottom * width + right],
            ];
            let weights = [
                (1.0 - dx) * (1.0 - dy),
                dx * (1.0 - dy),
                (1.0 - dx) * dy,
                dx * dy,
            ];
            let mut blend = [0u8; 4];
            for (channel, slot) in blend.iter_mut().enumerate() {
                let sum: f64 = corners
                    .iter()
                    .zip(weights)
                    .map(|(c, w)| c[channel] as f64 * w)
                    .sum();
                *slot = sum.round().clamp(0.0, 255.0) as u8;
            }
            out.push(blend);
        }
    }
    out
}

fn boxed(
    pixels: &[[u8; 4]],
    width: usize,
    height: usize,
    out_w: usize,
    out_h: usize,
) -> Vec<[u8; 4]> {
    let span = |target: usize, out: usize, source: usize| {
        let low = target * source / out;
        (low, ((target + 1) * source).div_ceil(out).max(low + 1))
    };
    let mut out = Vec::with_capacity(out_w * out_h);
    for y in 0..out_h {
        let (top, bottom) = span(y, out_h, height);
        for x in 0..out_w {
            let (left, right) = span(x, out_w, width);
            let count = ((bottom - top) * (right - left)) as u32;
            let mut sums = [0u32; 4];
            for row in top..bottom {
                for col in left..right {
                    let pixel = pixels[row * width + col];
                    for (slot, &value) in sums.iter_mut().zip(pixel.iter()) {
                        *slot += u32::from(value);
                    }
                }
            }
            let mut mean = [0u8; 4];
            for (slot, &sum) in mean.iter_mut().zip(sums.iter()) {
                *slot = ((sum + count / 2) / count) as u8;
            }
            out.push(mean);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILTERS: [Filter; 3] = [Filter::Nearest, Filter::Linear, Filter::Box];

    fn ramp(width: usize, height: usize) -> Vec<[u8; 4]> {
        (0..width * height)
            .map(|i| {
                let v = (i * 7 % 251) as u8;
                [v, 255 - v, v / 2, 255]
            })
            .collect()
    }

    #[test]
    fn every_filter_keeps_the_same_size_untouched() {
        let pixels = ramp(5, 3);
        for filter in FILTERS {
            let out = resample(&pixels, 5, 3, 5, 3, filter).unwrap();
            assert_eq!(out, pixels, "{filter:?} moved an unchanged size");
        }
    }

    #[test]
    fn every_filter_keeps_a_flat_field_flat() {
        let pixels = vec![[17, 34, 51, 255]; 64];
        for filter in FILTERS {
            for (w, h) in [(3, 3), (8, 8), (17, 5), (1, 1)] {
                let out = resample(&pixels, 8, 8, w, h, filter).unwrap();
                assert_eq!(out.len(), w * h);
                assert!(
                    out.iter().all(|&p| p == [17, 34, 51, 255]),
                    "{filter:?} smeared a flat field at {w}x{h}"
                );
            }
        }
    }

    #[test]
    fn nearest_upscale_matches_block_replication() {
        let pixels = ramp(4, 3);
        let out = resample(&pixels, 4, 3, 12, 9, Filter::Nearest).unwrap();
        for y in 0..9 {
            for x in 0..12 {
                assert_eq!(out[y * 12 + x], pixels[(y / 3) * 4 + x / 3], "at {x},{y}");
            }
        }
    }

    #[test]
    fn box_downscale_averages_the_block() {
        let pixels = vec![
            [0, 0, 0, 255],
            [10, 20, 30, 255],
            [100, 100, 100, 255],
            [200, 60, 40, 255],
        ];
        let out = resample(&pixels, 2, 2, 1, 1, Filter::Box).unwrap();
        assert_eq!(out, vec![[78, 45, 43, 255]]);
    }

    #[test]
    fn box_halving_is_a_two_by_two_mean() {
        let pixels = ramp(8, 8);
        let out = resample(&pixels, 8, 8, 4, 4, Filter::Box).unwrap();
        for y in 0..4 {
            for x in 0..4 {
                let block = [
                    pixels[2 * y * 8 + 2 * x],
                    pixels[2 * y * 8 + 2 * x + 1],
                    pixels[(2 * y + 1) * 8 + 2 * x],
                    pixels[(2 * y + 1) * 8 + 2 * x + 1],
                ];
                let mean = (0..4)
                    .map(|c| ((block.iter().map(|p| p[c] as u32).sum::<u32>() + 2) / 4) as u8)
                    .collect::<Vec<u8>>();
                assert_eq!(out[y * 4 + x].to_vec(), mean, "at {x},{y}");
            }
        }
    }

    #[test]
    fn linear_upscale_pins_the_corners() {
        let pixels = ramp(4, 4);
        let out = resample(&pixels, 4, 4, 16, 16, Filter::Linear).unwrap();
        assert_eq!(out[0], pixels[0]);
        assert_eq!(out[15], pixels[3]);
        assert_eq!(out[15 * 16], pixels[12]);
        assert_eq!(out[15 * 16 + 15], pixels[15]);
    }

    #[test]
    fn linear_stays_between_its_neighbors() {
        let pixels = vec![[0, 0, 0, 255], [255, 255, 255, 255]];
        let out = resample(&pixels, 2, 1, 9, 1, Filter::Linear).unwrap();
        assert_eq!(out[0], [0, 0, 0, 255]);
        assert_eq!(out[8], [255, 255, 255, 255]);
        for pair in out.windows(2) {
            assert!(pair[1][0] >= pair[0][0], "linear ramp fell back");
        }
    }

    #[test]
    fn resample_rejects_bad_sizes() {
        let pixels = ramp(2, 2);
        assert!(resample(&pixels, 2, 2, 0, 4, Filter::Nearest).is_err());
        assert!(resample(&pixels, 2, 2, 4, 0, Filter::Nearest).is_err());
        assert!(resample(&pixels, 0, 2, 4, 4, Filter::Nearest).is_err());
        assert!(resample(&pixels, 3, 2, 4, 4, Filter::Nearest).is_err());
    }

    #[test]
    fn block_replicates_every_source_element() {
        let src: Vec<u8> = (0..6).collect();
        assert_eq!(block(&src, 3, 2, 1), src);
        let grown = block(&src, 3, 2, 3);
        assert_eq!(grown.len(), 54);
        for y in 0..6 {
            for x in 0..9 {
                assert_eq!(grown[y * 9 + x], src[(y / 3) * 3 + x / 3], "at {x},{y}");
            }
        }
    }

    #[test]
    fn block_matches_a_nearest_upscale() {
        let pixels = ramp(4, 3);
        let out = resample(&pixels, 4, 3, 12, 9, Filter::Nearest).unwrap();
        assert_eq!(block(&pixels, 4, 3, 3), out);
    }

    #[test]
    fn hex_fit_squashes_one_axis_only() {
        let pixels = ramp(20, 10);
        let (w, h, out) = hex_fit(&pixels, 20, 10, true, Filter::Box).unwrap();
        assert_eq!((w, h), (17, 10));
        assert_eq!(out.len(), 170);
        let (w, h, out) = hex_fit(&pixels, 20, 10, false, Filter::Nearest).unwrap();
        assert_eq!((w, h), (20, 8));
        assert_eq!(out.len(), 160);
        assert_eq!(hex_size(1, 1, true), (1, 1));
    }
}
