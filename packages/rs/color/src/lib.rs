mod palette;
pub mod ramp;

pub use palette::*;
pub use ramp::Colorizer;

use mrly_error::{Error, Result, value_error};
use mrly_rand::Rng;
use mrly_tensor::Tensor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const ALPHA: Color = Color::rgba(0, 0, 0, 0);

pub fn shades(hue: Color) -> [Color; 3] {
    let lerp = |to: Color, t: f64| {
        let step = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * t).round() as u8;
        Color::rgb(step(hue.r, to.r), step(hue.g, to.g), step(hue.b, to.b))
    };
    [lerp(WHITE, 0.5), hue, lerp(BLACK, 0.4)]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    pub ground: Color,
    pub bg: Color,
    pub panel: Color,
    pub deep: Color,
    pub line: Color,
    pub fg: Color,
    pub dim: Color,
    pub accent: Color,
    pub on_accent: Color,
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub mint: Color,
    pub teal: Color,
    pub cyan: Color,
    pub blue: Color,
    pub indigo: Color,
    pub purple: Color,
    pub pink: Color,
    pub brown: Color,
    pub gray: Color,
}

impl Theme {
    pub fn hues(&self) -> [Color; 13] {
        [
            self.red,
            self.orange,
            self.yellow,
            self.green,
            self.mint,
            self.teal,
            self.cyan,
            self.blue,
            self.indigo,
            self.purple,
            self.pink,
            self.brown,
            self.gray,
        ]
    }
    pub fn inks(&self) -> [Color; 6] {
        [
            self.blue,
            self.orange,
            self.yellow,
            self.green,
            self.pink,
            self.indigo,
        ]
    }
}

const INKS: Theme = Theme {
    ground: BLACK,
    bg: BLACK,
    panel: BLACK,
    deep: BLACK,
    line: BLACK,
    fg: WHITE,
    dim: GRAY,
    accent: BLUE,
    on_accent: WHITE,
    red: RED,
    orange: ORANGE,
    yellow: YELLOW,
    green: GREEN,
    mint: MINT,
    teal: TEAL,
    cyan: CYAN,
    blue: BLUE,
    indigo: INDIGO,
    purple: PURPLE,
    pink: PINK,
    brown: BROWN,
    gray: GRAY,
};

pub const DARK: Theme = Theme {
    ground: BLACK,
    bg: Color::rgb(7, 7, 7),
    panel: Color::rgb(17, 17, 18),
    deep: BLACK,
    line: Color::rgb(31, 31, 32),
    fg: WHITE,
    dim: GRAY,
    ..INKS
};

pub const LIGHT: Theme = Theme {
    ground: WHITE,
    bg: Color::rgb(248, 248, 249),
    panel: Color::rgb(241, 241, 242),
    deep: Color::rgb(232, 232, 233),
    line: Color::rgb(221, 221, 223),
    fg: BLACK,
    dim: Color::rgb(85, 85, 88),
    ..INKS
};

pub fn named(name: &str) -> Result<Color> {
    match NAMES.iter().position(|&n| n == name) {
        Some(i) => Ok(PALETTE[i]),
        None => value_error(format!("unknown color name {name:?}.")),
    }
}

pub fn board(dark: bool) -> [u8; 4] {
    let c = if dark { DARK.ground } else { LIGHT.ground };
    [c.r, c.g, c.b, c.a]
}

pub fn ink(dark: bool) -> [u8; 4] {
    let c = if dark { DARK.fg } else { LIGHT.fg };
    [c.r, c.g, c.b, c.a]
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Color, D::Error> {
        Color::from_hex(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
    pub fn from_hex(hex: &str) -> Result<Color> {
        let code = hex.trim_start_matches('#');
        let byte = |i: usize| -> Result<u8> {
            let pair = code
                .get(i..i + 2)
                .ok_or_else(|| Error::Value(format!("invalid hex code {hex:?}.")))?;
            u8::from_str_radix(pair, 16)
                .map_err(|_| Error::Value(format!("invalid hex code {hex:?}.")))
        };
        if !code.is_ascii() {
            return value_error("Hex code must be in format #RRGGBB or #RRGGBBAA");
        }
        match code.len() {
            6 => Ok(Color::rgb(byte(0)?, byte(2)?, byte(4)?)),
            8 => Ok(Color::rgba(byte(0)?, byte(2)?, byte(4)?, byte(6)?)),
            _ => value_error("Hex code must be in format #RRGGBB or #RRGGBBAA"),
        }
    }
    pub fn css(&self) -> String {
        if self.a == 255 {
            format!("rgb({},{},{})", self.r, self.g, self.b)
        } else {
            format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
        }
    }
    pub fn alpha(&self, level: u8) -> Color {
        Color::rgba(self.r, self.g, self.b, level)
    }
    pub fn invert(&self) -> Color {
        Color::rgba(255 - self.r, 255 - self.g, 255 - self.b, self.a)
    }
    pub fn lightness(&self, level: u8) -> Result<Color> {
        if level > 100 {
            return value_error(format!("Level must be between 0 and 100, got {level}"));
        }
        let scale = |v: u8| -> u8 {
            if level == 50 {
                v
            } else if level < 50 {
                (v as f64 * level as f64 / 50.0) as u8
            } else {
                (v as f64 + (255.0 - v as f64) * (level as f64 - 50.0) / 50.0) as u8
            }
        };
        Ok(Color::rgba(
            scale(self.r),
            scale(self.g),
            scale(self.b),
            self.a,
        ))
    }
    pub fn random(alpha: bool, rng: &mut Rng) -> Color {
        Color::rgba(
            rng.below(256) as u8,
            rng.below(256) as u8,
            rng.below(256) as u8,
            if alpha { rng.below(256) as u8 } else { 255 },
        )
    }
}

pub fn mix(color_1: Color, color_2: Color, ratio: f64) -> Result<Color> {
    if !(0.0..=1.0).contains(&ratio) {
        return value_error(format!("Ratio must be between 0.0 and 1.0, got {ratio}"));
    }
    let lerp = |a: u8, b: u8| -> u8 { (a as f64 + (b as f64 - a as f64) * ratio) as u8 };
    Ok(Color::rgba(
        lerp(color_1.r, color_2.r),
        lerp(color_1.g, color_2.g),
        lerp(color_1.b, color_2.b),
        lerp(color_1.a, color_2.a),
    ))
}

pub fn gradient(colors: &[Color], steps: usize) -> Result<Vec<Color>> {
    if colors.is_empty() {
        return value_error("Cannot create a gradient from an empty list of colors.");
    }
    if steps < 1 {
        return value_error("Steps must be at least 1.");
    }
    if steps == 1 {
        return Ok(vec![colors[0]]);
    }
    if colors.len() == 1 {
        return Ok(vec![colors[0]; steps]);
    }
    let segments = colors.len() - 1;
    let mut result = Vec::with_capacity(steps);
    for i in 0..steps {
        let pos = i as f64 / (steps - 1) as f64;
        let mut seg = (pos * segments as f64) as usize;
        if seg >= segments {
            seg = segments - 1;
        }
        let ratio = pos * segments as f64 - seg as f64;
        result.push(mix(colors[seg], colors[seg + 1], ratio)?);
    }
    Ok(result)
}

pub fn snap(pixels: &[[u8; 4]], palette: &[Color]) -> Vec<[u8; 4]> {
    if palette.is_empty() {
        return pixels.to_vec();
    }
    let inks: Vec<[u8; 4]> = palette.iter().map(|c| [c.r, c.g, c.b, c.a]).collect();
    let far = |a: [u8; 4], b: [u8; 4]| -> u32 {
        (0..4)
            .map(|i| {
                let d = a[i].abs_diff(b[i]) as u32;
                d * d
            })
            .sum()
    };
    pixels
        .iter()
        .map(|&px| {
            let mut best = 0;
            let mut near = far(px, inks[0]);
            for (i, &ink) in inks.iter().enumerate().skip(1) {
                let d = far(px, ink);
                if d < near {
                    near = d;
                    best = i;
                }
            }
            inks[best]
        })
        .collect()
}

pub fn luma_types(pixels: &[[u8; 4]], width: usize, height: usize, level: u8) -> Tensor {
    let mut types = Tensor::new(vec![height, width]);
    for flat in 0..width * height {
        let px = pixels.get(flat).copied().unwrap_or([0, 0, 0, 0]);
        let mean = (px[0] as f64 + px[1] as f64 + px[2] as f64) / 3.0;
        types.put(flat, i64::from(mean < level as f64));
    }
    types
}
