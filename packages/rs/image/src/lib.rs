pub mod sample;

use crate::sample::{Filter, block};
use mrly_error::{Error, Result, shape_error, value_error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
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
            sample::resample(&self.colors, self.width, self.height, width, height, filter)?;
        Ok(Image {
            width,
            height,
            colors,
        })
    }
}

pub fn to_png(image: &Image) -> Result<Vec<u8>> {
    mrly_png::encode(&image.colors, image.width, image.height).map_err(Error::Value)
}

pub fn from_png(bytes: &[u8]) -> Result<Image> {
    let (width, height, colors) = mrly_png::decode(bytes).map_err(Error::Value)?;
    Image::new(width, height, colors)
}

fn paint(attr: &str, [r, g, b, a]: [u8; 4]) -> String {
    let mut out = format!("{attr}=\"#{r:02x}{g:02x}{b:02x}\"");
    if a < 255 {
        out += &format!(" {attr}-opacity=\"{:.2}\"", a as f64 / 255.0);
    }
    out
}

pub fn svg(images: &[Image], gap: usize, scale: usize) -> String {
    let width = images
        .iter()
        .map(|i| i.width + gap)
        .sum::<usize>()
        .saturating_sub(gap);
    let height = images.iter().map(|i| i.height).max().unwrap_or(0);
    let ground = images.first().and_then(|i| i.colors.first()).copied();
    let mut paths: Vec<([u8; 4], String)> = Vec::new();
    let mut left = 0;
    for image in images {
        for (y, row) in image.colors.chunks(image.width.max(1)).enumerate() {
            let mut x = 0;
            while x < row.len() {
                let run = row[x..].iter().take_while(|&&c| c == row[x]).count();
                if row[x][3] > 0 && Some(row[x]) != ground {
                    let line = format!("M{} {y}.5h{run}", left + x);
                    match paths.iter_mut().find(|(c, _)| *c == row[x]) {
                        Some((_, d)) => d.push_str(&line),
                        None => paths.push((row[x], line)),
                    }
                }
                x += run;
            }
        }
        left += image.width + gap;
    }
    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{}\" shape-rendering=\"crispEdges\">\n",
        width * scale
    );
    if let Some(ground) = ground.filter(|c| c[3] > 0) {
        out += &format!(
            "<rect width=\"{width}\" height=\"{height}\" {}/>\n",
            paint("fill", ground)
        );
    }
    for (color, d) in paths {
        out += &format!("<path {} d=\"{d}\"/>\n", paint("stroke", color));
    }
    out + "</svg>"
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
            let at = |dy: usize, dx: usize| -> [f32; 4] {
                if y + dy == 0 || x + dx == 0 {
                    [0.0; 4]
                } else {
                    sums[(y + dy - 1) * pw + x + dx - 1]
                }
            };
            let (tl, tr, bl, br) = (
                at(2 * radius + 1, 2 * radius + 1),
                at(2 * radius + 1, 0),
                at(0, 2 * radius + 1),
                at(0, 0),
            );
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

#[derive(Serialize)]
struct PlainRef<'a> {
    width: usize,
    height: usize,
    colors: &'a [[u8; 4]],
}

struct Flat<'a>(&'a [[u8; 4]]);

impl Serialize for Flat<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        mrly_wire::write_ref(s, self.0.as_flattened())
    }
}

struct Wired<'a>(&'a Image);

impl Serialize for Wired<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut out = s.serialize_struct("Image", 2)?;
        out.serialize_field("shape", &[self.0.height, self.0.width])?;
        out.serialize_field("colors", &Flat(&self.0.colors))?;
        out.end()
    }
}

impl Serialize for Image {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        if mrly_wire::on() {
            mrly_wire::form(s, "$i", &Wired(self))
        } else {
            PlainRef {
                width: self.width,
                height: self.height,
                colors: &self.colors,
            }
            .serialize(s)
        }
    }
}

struct Incoming;

impl<'de> serde::de::Visitor<'de> for Incoming {
    type Value = Image;
    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("an image")
    }
    fn visit_map<A: serde::de::MapAccess<'de>>(
        self,
        mut map: A,
    ) -> std::result::Result<Image, A::Error> {
        use mrly_wire::Prim;
        use serde::de::Error as _;
        let mut shape: Option<Vec<usize>> = None;
        let mut colors = None;
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "shape" => shape = Some(map.next_value()?),
                "colors" | "data" => colors = Some(map.next_value::<mrly_wire::Raw>()?),
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }
        let shape = shape.ok_or_else(|| A::Error::custom("an image wants a shape"))?;
        let &[height, width, ..] = shape.as_slice() else {
            return Err(A::Error::custom(
                "an image wants a shape of two lengths, the height and the width.",
            ));
        };
        let colors = colors.ok_or_else(|| A::Error::custom("an image wants colors"))?;
        let flat = u8::from_blob(colors.0);
        if flat.len() % 4 != 0 {
            return Err(A::Error::custom("image colors want four bytes a pixel."));
        }
        Image::new(width, height, mrly_wire::flat_rgba(&flat)).map_err(A::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        use serde::de::Error as _;
        if mrly_wire::on() {
            d.deserialize_any(Incoming)
        } else {
            let parts = Parts::deserialize(d)?;
            Image::try_from(parts).map_err(D::Error::custom)
        }
    }
}
