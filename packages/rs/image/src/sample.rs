use mrly_error::{Result, overflow_error, shape_error, value_error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    Nearest,
    Linear,
    Box,
}

pub const HEX_RATIO: f64 = 0.866_025_403_784_438_6;

pub fn hex_size(width: usize, height: usize, vertical: bool) -> (usize, usize) {
    let squash = |value: usize| ((value as f64 * HEX_RATIO) as usize).max(1);
    match vertical {
        true => (squash(width), height),
        false => (width, squash(height)),
    }
}

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
    if Some(pixels.len()) != width.checked_mul(height) {
        return value_error("pixels length must equal width * height.");
    }
    if out_w.checked_mul(out_h).is_none() {
        return overflow_error("resample output size overflows usize.");
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

pub fn block<T: Copy>(src: &[T], width: usize, height: usize, scale: usize) -> Result<Vec<T>> {
    if Some(src.len()) != width.checked_mul(height) {
        return shape_error("block source length must equal width * height.");
    }
    if scale == 1 {
        return Ok(src.to_vec());
    }
    let Some(size) = width
        .checked_mul(scale)
        .and_then(|w| height.checked_mul(scale).and_then(|h| w.checked_mul(h)))
    else {
        return overflow_error("block output size overflows usize.");
    };
    let mut out = Vec::with_capacity(size);
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
    Ok(out)
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
