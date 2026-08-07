use super::{Skin, Tint, Visual};

pub const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const BANDS: usize = 4;

pub fn skin() -> Skin {
    let mut visuals = vec![Visual::none()];
    for band in 0..BANDS {
        for ch in ALPHABET.chars() {
            visuals.push(Visual::none().glyph(ch.to_string()).tinted(Tint::Pen(band)));
        }
    }
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    vec![("lettered", skin())]
}
