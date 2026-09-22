use super::error::{value_error, Error, Result};
use super::rng::Rng;
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

// PALETTE

/// The palette black, #000000.
pub const BLACK: Color = Color::rgb(0, 0, 0);
/// The palette white, #ffffff.
pub const WHITE: Color = Color::rgb(255, 255, 255);
/// The palette red, #ff3d40.
pub const RED: Color = Color::rgb(255, 61, 64);
/// The palette orange, #ff8f2c.
pub const ORANGE: Color = Color::rgb(255, 143, 44);
/// The palette yellow, #ffd100.
pub const YELLOW: Color = Color::rgb(255, 209, 0);
/// The palette green, #32cc58.
pub const GREEN: Color = Color::rgb(50, 204, 88);
/// The palette mint, #00d1bb.
pub const MINT: Color = Color::rgb(0, 209, 187);
/// The palette teal, #00cad8.
pub const TEAL: Color = Color::rgb(0, 202, 216);
/// The palette cyan, #1ec9f3.
pub const CYAN: Color = Color::rgb(30, 201, 243);
/// The palette blue, #008cff.
pub const BLUE: Color = Color::rgb(0, 140, 255);
/// The palette indigo, #6768fa.
pub const INDIGO: Color = Color::rgb(103, 104, 250);
/// The palette purple, #d332e9.
pub const PURPLE: Color = Color::rgb(211, 50, 233);
/// The palette pink, #ff325a.
pub const PINK: Color = Color::rgb(255, 50, 90);
/// The palette brown, #b18462.
pub const BROWN: Color = Color::rgb(177, 132, 98);
/// The palette gray, #8e8e93.
pub const GRAY: Color = Color::rgb(142, 142, 147);

/// The fifteen names, in palette order.
#[rustfmt::skip]
pub const NAMES: [&str; 15] = ["black", "white", "red", "orange", "yellow", "green", "mint", "teal", "cyan", "blue", "indigo", "purple", "pink", "brown", "gray"];

/// The fifteen colors, in name order.
#[rustfmt::skip]
pub const PALETTE: [Color; 15] = [BLACK, WHITE, RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN, GRAY];

// PALETTE END

// SHADES

/// Returns a hue one shade lighter, itself, and one shade darker.
pub fn shades(hue: Color) -> [Color; 3] {
    let lerp = |to: Color, t: f64| {
        let step = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * t).round() as u8;
        Color::rgb(step(hue.r, to.r), step(hue.g, to.g), step(hue.b, to.b))
    };
    [lerp(WHITE, 0.5), hue, lerp(BLACK, 0.4)]
}

// THEME

/// One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    /// The ground every figure is painted on.
    pub ground: Color,
    /// The page background, one step off the ground.
    pub bg: Color,
    /// The raised panel.
    pub panel: Color,
    /// The sunken well.
    pub deep: Color,
    /// The hairline between things.
    pub line: Color,
    /// The foreground, the strongest tone.
    pub fg: Color,
    /// The dimmed foreground, for anything secondary.
    pub dim: Color,
    /// The interactive accent.
    pub accent: Color,
    /// The tone written on the accent.
    pub on_accent: Color,
    /// The red ink.
    pub red: Color,
    /// The orange ink.
    pub orange: Color,
    /// The yellow ink.
    pub yellow: Color,
    /// The green ink.
    pub green: Color,
    /// The mint ink.
    pub mint: Color,
    /// The teal ink.
    pub teal: Color,
    /// The cyan ink.
    pub cyan: Color,
    /// The blue ink.
    pub blue: Color,
    /// The indigo ink.
    pub indigo: Color,
    /// The purple ink.
    pub purple: Color,
    /// The pink ink.
    pub pink: Color,
    /// The brown ink.
    pub brown: Color,
    /// The gray ink.
    pub gray: Color,
}

impl Theme {
    /// The thirteen inks in name order.
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
    /// The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo.
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

/// The dark theme.
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

/// The light theme.
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

/// Returns the palette color a name spells.
///
/// # Errors
///
/// Errs when no palette color carries the name.
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
    ///
    /// ```
    /// use mrlyrs::core::Color;
    /// assert_eq!(Color::rgba(0, 140, 255, 128).to_hex(), "#008cff80");
    /// ```
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
    /// Parses a #RRGGBB or #RRGGBBAA code, hash optional.
    ///
    /// # Errors
    ///
    /// Errs when the text is not six or eight ascii hex digits after an optional hash.
    ///
    /// ```
    /// assert_eq!(mrlyrs::core::Color::from_hex("#ff3d40").unwrap(), mrlyrs::core::colors::RED);
    /// ```
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
    /// Returns the color scaled toward black below level 50 and toward white above.
    ///
    /// # Errors
    ///
    /// Errs when the level is past 100.
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
    /// Draws a color from the stream, opaque unless alpha is asked for.
    pub fn random(alpha: bool, rng: &mut Rng) -> Color {
        Color::rgba(
            rng.below(256) as u8,
            rng.below(256) as u8,
            rng.below(256) as u8,
            if alpha { rng.below(256) as u8 } else { 255 },
        )
    }
}

/// Blends two colors linearly by ratio.
///
/// # Errors
///
/// Errs when the ratio falls outside zero to one.
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
///
/// # Errors
///
/// Errs on an empty stop list or fewer than one step.
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
    fn refuses_from_hex() {
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
    fn theme_rgba_reads_the_two_themes() {
        assert_eq!(ink(true), [255, 255, 255, 255]);
        assert_eq!(board(false), [255, 255, 255, 255]);
        assert_eq!(ink(false), [0, 0, 0, 255]);
        assert_eq!(board(true), [0, 0, 0, 255]);
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
