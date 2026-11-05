use super::error::{Error, Result, shape_error, value_error};
use super::resample::{self, Filter, block};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Parts")]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub colors: Vec<[u8; 4]>,
}

impl Image {
    pub fn new(width: usize, height: usize, colors: Vec<[u8; 4]>) -> Result<Image> {
        if Some(colors.len()) != width.checked_mul(height) {
            return shape_error("colors length must equal width * height.");
        }
        Ok(Image {
            width,
            height,
            colors,
        })
    }
    pub fn scale(&self, scale: usize) -> Result<Image> {
        if scale < 1 {
            return value_error("scale must be at least 1.");
        }
        let colors = block(&self.colors, self.width, self.height, scale)?;
        Ok(Image {
            width: self.width * scale,
            height: self.height * scale,
            colors,
        })
    }
    pub fn resample(&self, width: usize, height: usize, filter: Filter) -> Result<Image> {
        let colors =
            resample::resample(&self.colors, self.width, self.height, width, height, filter)?;
        Ok(Image {
            width,
            height,
            colors,
        })
    }
}

pub fn blur(pixels: &[[u8; 4]], width: usize, height: usize, radius: usize) -> Vec<[u8; 4]> {
    let read = |y: usize, x: usize| -> [u8; 4] {
        pixels.get(y * width + x).copied().unwrap_or([0, 0, 0, 0])
    };
    if radius == 0 || width == 0 || height == 0 {
        return (0..width * height)
            .map(|i| read(i / width, i % width))
            .collect();
    }
    let (pw, ph) = (width + 2 * radius, height + 2 * radius);
    let mut sums = vec![[0f32; 4]; pw * ph];
    for y in 0..ph {
        let sy = y.saturating_sub(radius).min(height - 1);
        for x in 0..pw {
            let sx = x.saturating_sub(radius).min(width - 1);
            let px = read(sy, sx);
            sums[y * pw + x] = [px[0] as f32, px[1] as f32, px[2] as f32, px[3] as f32];
        }
    }
    for y in 1..ph {
        for x in 0..pw {
            let above = sums[(y - 1) * pw + x];
            for (sum, up) in sums[y * pw + x].iter_mut().zip(above) {
                *sum += up;
            }
        }
    }
    for y in 0..ph {
        for x in 1..pw {
            let left = sums[y * pw + x - 1];
            for (sum, prev) in sums[y * pw + x].iter_mut().zip(left) {
                *sum += prev;
            }
        }
    }
    let area = ((2 * radius + 1) * (2 * radius + 1)) as f32;
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let tl = sums[(y + 2 * radius) * pw + x + 2 * radius];
            let tr = sums[(y + 2 * radius) * pw + x];
            let bl = sums[y * pw + x + 2 * radius];
            let br = sums[y * pw + x];
            let mut px = [0u8; 4];
            for c in 0..4 {
                px[c] = (((tl[c] - tr[c] - bl[c] + br[c]) / area).clamp(0.0, 255.0)) as u8;
            }
            out.push(px);
        }
    }
    out
}

#[derive(Deserialize)]
struct Parts {
    width: usize,
    height: usize,
    colors: Vec<[u8; 4]>,
}

impl TryFrom<Parts> for Image {
    type Error = Error;

    fn try_from(parts: Parts) -> Result<Image> {
        Image::new(parts.width, parts.height, parts.colors)
    }
}
