use super::errors::{value_error, MrlyError, Result};
use super::state;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An rgba color with byte channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    /// The red channel.
    pub r: u8,
    /// The green channel.
    pub g: u8,
    /// The blue channel.
    pub b: u8,
    /// The alpha channel, 255 for opaque.
    pub a: u8,
}

/// The fully transparent color.
pub const ALPHA: Color = Color::rgba(0, 0, 0, 0);

pub use crate::palette::*;

/// Returns the palette color a name spells, or an error for a stranger.
pub fn named(name: &str) -> Result<Color> {
    match NAMES.iter().position(|&n| n == name) {
        Some(i) => Ok(PALETTE[i]),
        None => value_error(format!("unknown color name {name:?}.")),
    }
}

/// Returns the ground rgba of the dark or the light theme.
pub fn board(dark: bool) -> [u8; 4] {
    let c = if dark { DARK.ground } else { LIGHT.ground };
    [c.r, c.g, c.b, c.a]
}

/// Returns the foreground rgba of the dark or the light theme.
pub fn ink(dark: bool) -> [u8; 4] {
    let c = if dark { DARK.fg } else { LIGHT.fg };
    [c.r, c.g, c.b, c.a]
}

/// Formats a raw rgba as a hex string.
pub fn hex(c: [u8; 4]) -> String {
    Color::rgba(c[0], c[1], c[2], c[3]).to_hex()
}

/// Parses a hex string to raw rgba, falling back to opaque black.
pub fn hex_of(hex: &str) -> [u8; 4] {
    match Color::from_hex(hex) {
        Ok(c) => [c.r, c.g, c.b, c.a],
        Err(_) => [0, 0, 0, 255],
    }
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
    /// Builds an opaque color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    /// Builds a color with an explicit alpha.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    /// Formats the color as lowercase hex, appending the alpha pair only when not opaque.
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
    /// Parses a #RRGGBB or #RRGGBBAA code, hash optional, or an error for anything else.
    ///
    /// ```
    /// assert_eq!(mrlycore::Color::from_hex("#ff3d40").unwrap(), mrlycore::colors::RED);
    /// ```
    pub fn from_hex(hex: &str) -> Result<Color> {
        let code = hex.trim_start_matches('#');
        let byte = |i: usize| -> Result<u8> {
            let pair = code
                .get(i..i + 2)
                .ok_or_else(|| MrlyError::Value(format!("invalid hex code {hex:?}.")))?;
            u8::from_str_radix(pair, 16)
                .map_err(|_| MrlyError::Value(format!("invalid hex code {hex:?}.")))
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
    /// Formats the color as a css rgb or rgba call.
    pub fn css(&self) -> String {
        if self.a == 255 {
            format!("rgb({},{},{})", self.r, self.g, self.b)
        } else {
            format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
        }
    }
    /// Returns the color with its alpha set to level.
    pub fn alpha(&self, level: u8) -> Color {
        Color::rgba(self.r, self.g, self.b, level)
    }
    /// Returns the color with every channel flipped and the alpha kept.
    pub fn invert(&self) -> Color {
        Color::rgba(255 - self.r, 255 - self.g, 255 - self.b, self.a)
    }
    /// Returns the color scaled toward black below level 50 and toward white above, or an error past 100.
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
    /// Draws a color from the shared rng, opaque unless alpha is asked for.
    pub fn random(alpha: bool) -> Color {
        Color::rgba(
            state::randint(0, 255) as u8,
            state::randint(0, 255) as u8,
            state::randint(0, 255) as u8,
            if alpha {
                state::randint(0, 255) as u8
            } else {
                255
            },
        )
    }
}

/// Blends two colors linearly by ratio, or an error outside the unit interval.
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

/// Builds a gradient of steps colors sweeping evenly through the given stops.
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

#[cfg(test)]
mod tests {
    #[test]
    fn from_hex_errors_on_non_ascii_instead_of_panicking() {
        use super::*;
        for bad in [
            "a\u{e9}bcd",
            "\u{e9}\u{e9}\u{e9}",
            "#a\u{e9}bcd",
            "\u{1f600}\u{1f600}",
        ] {
            assert!(Color::from_hex(bad).is_err(), "{bad:?} must be an error");
        }
        assert_eq!(Color::from_hex("#ff0000").unwrap(), Color::rgb(255, 0, 0));
    }

    use super::*;
    #[test]
    fn named_palette() {
        assert_eq!(named("black").unwrap(), BLACK);
        assert_eq!(named("white").unwrap(), WHITE);
        assert_eq!(named("red").unwrap(), RED);
        assert_eq!(named("blue").unwrap(), BLUE);
        assert!(named("chartreuse").is_err());
        assert_eq!(NAMES.len(), PALETTE.len());
    }
    #[test]
    fn hex_round_trip() {
        for color in PALETTE {
            assert_eq!(Color::from_hex(&color.to_hex()).unwrap(), color);
        }
        assert_eq!(RED.to_hex(), "#ff3d40");
        assert_eq!(ALPHA.to_hex(), "#00000000");
    }
    #[test]
    fn gradient_endpoints() {
        let g = gradient(&[BLACK, WHITE], 5).unwrap();
        assert_eq!(g.len(), 5);
        assert_eq!(g[0], BLACK);
        assert_eq!(g[4], WHITE);
        assert_eq!(g[2], Color::rgb(127, 127, 127));
    }
    #[test]
    fn raw_hex_helpers_never_fail() {
        assert_eq!(hex([255, 61, 64, 255]), "#ff3d40");
        assert_eq!(hex([0, 0, 0, 0]), "#00000000");
        assert_eq!(hex_of("#ff3d40"), [255, 61, 64, 255]);
        assert_eq!(hex_of("#00000000"), [0, 0, 0, 0]);
        assert_eq!(hex_of("junk"), [0, 0, 0, 255]);
        assert_eq!(ink(true), [255, 255, 255, 255]);
        assert_eq!(board(false), [255, 255, 255, 255]);
    }
    #[test]
    fn mix_and_lightness() {
        assert_eq!(mix(BLACK, WHITE, 0.5).unwrap(), Color::rgb(127, 127, 127));
        assert!(mix(BLACK, WHITE, 1.5).is_err());
        assert_eq!(BLACK.lightness(100).unwrap(), WHITE);
        assert_eq!(WHITE.lightness(0).unwrap(), BLACK);
        assert_eq!(RED.lightness(50).unwrap(), RED);
    }
}
