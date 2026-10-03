use super::error::{shape_error, value_error, Error, Result};
use super::resample::{self, block, Filter};
use serde::{Deserialize, Serialize};

/// An image: its width, its height and the rgba color of every pixel, row by row from the top.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Parts")]
pub struct Image {
    /// The width in pixels.
    pub width: usize,
    /// The height in pixels.
    pub height: usize,
    /// The rgba color of every pixel, row by row.
    pub colors: Vec<[u8; 4]>,
}

impl Image {
    /// Builds an image from its width, its height and its colors.
    ///
    /// # Errors
    ///
    /// Errs when the colors do not count width by height.
    ///
    /// ```
    /// let image = mrlyrs::core::Image::new(2, 1, vec![[255, 0, 0, 255], [0, 0, 255, 255]])?;
    /// assert_eq!((image.width, image.height, image.colors[1]), (2, 1, [0, 0, 255, 255]));
    /// assert!(mrlyrs::core::Image::new(2, 2, vec![[0, 0, 0, 255]]).is_err());
    /// # Ok::<(), mrlyrs::Error>(())
    /// ```
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
    /// Draws every pixel as a scale by scale block, growing both sides by scale.
    ///
    /// # Errors
    ///
    /// Errs on a scale below one, colors that do not count width by height, or a grown size that overflows.
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
    /// Resamples the image to a new size.
    ///
    /// # Errors
    ///
    /// Errs on a side of zero, colors that do not count width by height, or a size that overflows.
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

/// Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::json;

    fn sample() -> Image {
        Image::new(
            2,
            2,
            vec![
                [255, 0, 0, 255],
                [0, 0, 0, 255],
                [0, 0, 0, 255],
                [0, 140, 255, 128],
            ],
        )
        .unwrap()
    }

    fn carpet_pixels() -> Vec<[u8; 4]> {
        use crate::core::cell::{mapping, Cell, Mode};
        Cell::new(crate::math::atoms::carpet_2d(3))
            .paint(&mapping(), Mode::Type, None)
            .unwrap()
            .colors
            .unwrap()
    }

    #[test]
    fn blur_averages_the_edge_padded_window() {
        let blurred = blur(&carpet_pixels(), 3, 3, 1);
        assert_eq!(blurred.len(), 9);
        assert_eq!(blurred[4], [28, 28, 28, 113]);
        assert_eq!(blur(&carpet_pixels(), 3, 3, 0), carpet_pixels());
    }

    #[test]
    fn json_round_trips() {
        let image = sample();
        let json = serde_json::to_value(&image).unwrap();
        assert_eq!(json["colors"][3], json!([0, 140, 255, 128]));
        let back: Image = serde_json::from_value(json).unwrap();
        assert_eq!(image, back);
    }

    #[test]
    fn refuses_from_json() {
        let read = |value| serde_json::from_value::<Image>(value);
        assert!(read(json!(null)).is_err());
        assert!(read(json!({ "width": 1, "height": 1 })).is_err());
        let short = json!({ "width": 1, "height": 2, "colors": [[0, 0, 0, 255]] });
        assert!(read(short).is_err());
        let long = json!({ "width": 1, "height": 1, "colors": [[0, 0, 0, 255], [0, 0, 0, 255]] });
        assert!(read(long).is_err());
        let thin = json!({ "width": 1, "height": 1, "colors": [[0, 0, 0]] });
        assert!(read(thin).is_err());
        let wide = json!({ "width": usize::MAX, "height": 2, "colors": [] });
        assert!(read(wide).is_err());
    }

    #[test]
    fn scale_draws_every_pixel_as_a_block() {
        let image = sample().scale(4).unwrap();
        assert_eq!((image.width, image.height), (8, 8));
        assert_eq!(image.colors.len(), 64);
        assert_eq!(image.colors[0], [255, 0, 0, 255]);
        assert_eq!(image.colors[3], [255, 0, 0, 255]);
        assert_eq!(image.colors[4], [0, 0, 0, 255]);
        assert_eq!(image.colors[63], [0, 140, 255, 128]);
        assert_eq!(sample().scale(1).unwrap(), sample());
        assert!(sample().scale(0).is_err());
        assert!(sample().scale(usize::MAX).is_err());
    }

    #[test]
    fn resample_keeps_the_colors_on_a_nearest_upscale() {
        let image = sample().resample(4, 4, Filter::Nearest).unwrap();
        assert_eq!((image.width, image.height), (4, 4));
        assert_eq!(image, sample().scale(2).unwrap());
        assert!(sample().resample(0, 4, Filter::Nearest).is_err());
    }
}
