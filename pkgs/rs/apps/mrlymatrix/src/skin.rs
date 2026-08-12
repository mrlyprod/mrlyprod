use mrlyskin::{Skin, Tint, Visual};

/// The pool of characters a falling column draws from.
pub const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// How many brightness steps a trail fades through.
pub const BANDS: usize = 4;

/// Builds the lettered skin: a blank, then every character once per band, tinted by that band's pen.
pub fn skin() -> Skin {
    let mut visuals = vec![Visual::none()];
    for band in 0..BANDS {
        for ch in ALPHABET.chars() {
            visuals.push(Visual::none().glyph(ch.to_string()).tinted(Tint::Pen(band)));
        }
    }
    Skin::new(visuals)
}

/// Lists every skin the app offers, paired with its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("lettered", skin())]
}
