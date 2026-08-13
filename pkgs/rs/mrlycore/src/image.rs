use super::codec;
use super::colors::Color;
use super::errors::{MrlyError, Result};
use super::resample::{self, Filter};
use crate::json::{field, usize_at};
use crate::{json, Json};

/// A paletted image: rows of palette indices and the palette they point into.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    /// The width in pixels.
    pub width: usize,
    /// The height in pixels.
    pub height: usize,
    /// The palette index of every pixel, row by row.
    pub rows: Vec<Vec<usize>>,
    /// The colors the rows index.
    pub palette: Vec<Color>,
}

impl Image {
    /// Builds an image from its four parts.
    pub fn new(width: usize, height: usize, rows: Vec<Vec<usize>>, palette: Vec<Color>) -> Image {
        Image {
            width,
            height,
            rows,
            palette,
        }
    }
    /// Builds a paletted image from raw rgba pixels, growing the palette as new colors appear.
    pub fn from_pixels(width: usize, height: usize, pixels: &[[u8; 4]]) -> Image {
        let mut palette: Vec<Color> = Vec::new();
        let mut rows = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let px = pixels.get(y * width + x).copied().unwrap_or([0, 0, 0, 0]);
                let color = Color::rgba(px[0], px[1], px[2], px[3]);
                let id = match palette.iter().position(|&c| c == color) {
                    Some(id) => id,
                    None => {
                        palette.push(color);
                        palette.len() - 1
                    }
                };
                row.push(id);
            }
            rows.push(row);
        }
        Image::new(width, height, rows, palette)
    }
    /// Serializes the image to json, palette as hex strings.
    pub fn to_json(&self) -> Json {
        json!({
            "width": self.width,
            "height": self.height,
            "rows": self.rows.clone(),
            "palette": self.palette.iter().map(|c| c.to_hex()).collect::<Vec<_>>(),
        })
    }
    /// Rebuilds an image from its json, or an error where a field is missing, ragged, or off the palette.
    pub fn from_json(value: &Json) -> Result<Image> {
        let width = usize_at(value, "width")?;
        let height = usize_at(value, "height")?;
        let palette = palette_list(value, "palette")?;
        let rows = row_list(value, "rows", width, height, palette.len())?;
        Ok(Image::new(width, height, rows, palette))
    }
    /// Returns the flat rgba pixels, transparent wherever an index misses the palette.
    pub fn colors(&self) -> Vec<[u8; 4]> {
        let mut out = Vec::with_capacity(self.width * self.height);
        for row in &self.rows {
            for &id in row {
                let c = self
                    .palette
                    .get(id)
                    .copied()
                    .unwrap_or(Color::rgba(0, 0, 0, 0));
                out.push([c.r, c.g, c.b, c.a]);
            }
        }
        out
    }
    /// Encodes the image as a png at the given scale.
    pub fn png(&self, scale: usize) -> Result<Vec<u8>> {
        codec::png(&self.colors(), self.width, self.height, scale)
    }
    /// Resamples the image to a new size, its palette rebuilt from the blended pixels.
    pub fn resample(&self, width: usize, height: usize, filter: Filter) -> Result<Image> {
        let pixels = resample::resample(
            &self.colors(),
            self.width,
            self.height,
            width,
            height,
            filter,
        )?;
        Ok(Image::from_pixels(width, height, &pixels))
    }
}

/// Encodes indexed frames sharing one size and palette as an animated gif.
pub fn gif(frames: &[Image], scale: usize, delay: usize) -> Result<Vec<u8>> {
    let first = match frames.first() {
        Some(first) => first,
        None => return Err(MrlyError::Value("gif needs at least one frame.".into())),
    };
    let palette: Vec<[u8; 4]> = first.palette.iter().map(|c| [c.r, c.g, c.b, c.a]).collect();
    let mut indices = Vec::with_capacity(frames.len());
    for frame in frames {
        if (frame.width, frame.height) != (first.width, first.height) {
            return Err(MrlyError::Value("every frame must share one size.".into()));
        }
        if frame.palette != first.palette {
            return Err(MrlyError::Value(
                "every frame must share one palette.".into(),
            ));
        }
        indices.push(
            frame
                .rows
                .iter()
                .flat_map(|row| row.iter().map(|&id| id as u8))
                .collect::<Vec<u8>>(),
        );
    }
    let views: Vec<&[u8]> = indices.iter().map(|f| f.as_slice()).collect();
    codec::gif(&views, &palette, first.width, first.height, scale, delay)
}

fn palette_list(value: &Json, key: &str) -> Result<Vec<Color>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    array
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| MrlyError::Value(format!("field {key:?} must hold hex strings.")))
                .and_then(Color::from_hex)
        })
        .collect()
}

fn row_list(
    value: &Json,
    key: &str,
    width: usize,
    height: usize,
    colors: usize,
) -> Result<Vec<Vec<usize>>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    if array.len() != height {
        return Err(MrlyError::Value(format!(
            "field {key:?} must hold {height} rows."
        )));
    }
    array
        .iter()
        .map(|row| {
            let row = row
                .as_array()
                .ok_or_else(|| MrlyError::Value(format!("field {key:?} must hold lists.")))?;
            if row.len() != width {
                return Err(MrlyError::Value(format!(
                    "field {key:?} must hold rows of {width}."
                )));
            }
            row.iter()
                .map(|v| {
                    v.as_u64()
                        .map(|n| n as usize)
                        .filter(|&id| id < colors)
                        .ok_or_else(|| {
                            MrlyError::Value(format!("field {key:?} must index the palette."))
                        })
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Image {
        Image::new(
            2,
            2,
            vec![vec![0, 1], vec![1, 2]],
            vec![
                Color::rgb(255, 0, 0),
                Color::rgb(0, 0, 0),
                Color::rgba(0, 140, 255, 128),
            ],
        )
    }

    #[test]
    fn json_round_trips() {
        let image = sample();
        let json = image.to_json();
        assert_eq!(json["palette"][0], json!("#ff0000"));
        assert_eq!(json["palette"][2], json!("#008cff80"));
        let back = Image::from_json(&json).unwrap();
        assert_eq!(image, back);
    }

    #[test]
    fn from_json_rejects_garbage() {
        assert!(Image::from_json(&json!(null)).is_err());
        assert!(Image::from_json(&json!({ "width": 1, "height": 1 })).is_err());
        let ragged = json!({ "width": 2, "height": 2, "rows": [[0]], "palette": ["#ffffff"] });
        assert!(Image::from_json(&ragged).is_err());
        let short = json!({ "width": 1, "height": 2, "rows": [[0]], "palette": ["#ffffff"] });
        assert!(Image::from_json(&short).is_err());
        let loose = json!({ "width": 1, "height": 1, "rows": [[9]], "palette": ["#ffffff"] });
        assert!(Image::from_json(&loose).is_err());
        let murky = json!({ "width": 1, "height": 1, "rows": [[0]], "palette": ["soup"] });
        assert!(Image::from_json(&murky).is_err());
    }

    #[test]
    fn pixels_round_trip() {
        let pixels = vec![
            [255, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 0, 0, 255],
            [255, 0, 0, 255],
        ];
        let image = Image::from_pixels(2, 2, &pixels);
        assert_eq!(image.rows, vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(image.palette.len(), 2);
        assert_eq!(image.colors(), pixels);
    }

    #[test]
    fn png_delegates_to_the_codec() {
        let bytes = sample().png(4).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        assert!(sample().png(0).is_err());
    }

    #[test]
    fn resample_keeps_the_palette_on_a_nearest_upscale() {
        let image = sample().resample(4, 4, Filter::Nearest).unwrap();
        assert_eq!((image.width, image.height), (4, 4));
        assert_eq!(image.palette.len(), 3);
        assert_eq!(image.rows[0], vec![0, 0, 1, 1]);
        assert_eq!(image.rows[3], vec![1, 1, 2, 2]);
        assert!(sample().resample(0, 4, Filter::Nearest).is_err());
    }

    #[test]
    fn gif_encodes_frames_sharing_one_palette() {
        let first = sample();
        let mut second = sample();
        second.rows = vec![vec![2, 1], vec![1, 0]];
        let bytes = gif(&[first.clone(), second], 2, 5).unwrap();
        assert_eq!(&bytes[0..6], b"GIF89a");
        assert_eq!(&bytes[6..8], &4u16.to_le_bytes());
        let mut odd = sample();
        odd.palette.pop();
        odd.rows = vec![vec![0, 1], vec![1, 0]];
        assert!(gif(&[first.clone(), odd], 1, 5).is_err());
        let mut wide = first.clone();
        wide.width = 4;
        wide.rows = vec![vec![0, 1, 0, 1], vec![1, 2, 1, 2]];
        assert!(gif(&[first, wide], 1, 5).is_err());
        assert!(gif(&[], 1, 5).is_err());
    }
}
