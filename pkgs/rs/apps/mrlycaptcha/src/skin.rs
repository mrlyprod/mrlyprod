use mrlymath::pick;
use mrlyskin::{Skin, Tint, Visual};

/// The looks on offer: tinted tiles or plain ink.
pub const VARIANTS: [&str; 2] = ["tiles", "digits"];

const SIZE: usize = 16;

fn rows(code: usize) -> Vec<Vec<u8>> {
    let cell = pick::tile(code, SIZE, [255, 255, 255, 255], [0, 0, 0, 0]);
    let mask = cell.types();
    (0..SIZE)
        .map(|r| (0..SIZE).map(|c| mask.get(&[r, c])).collect())
        .collect()
}

/// Builds the named variant's skin, one sprite per catalogue picture.
pub fn skin(variant: &str) -> Skin {
    let visuals = (0..pick::NAMES.len())
        .map(|code| {
            let face = Visual::none().sprite(rows(code));
            match variant {
                "digits" => face,
                _ => face.tinted(Tint::Pen(code)),
            }
        })
        .collect();
    Skin::new(visuals)
}

/// Lists every skin the app offers, paired with its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
