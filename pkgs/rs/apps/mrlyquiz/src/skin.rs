use mrlymath::pick;
use mrlyskin::{Skin, Tint, Visual};

/// The side length of each picture sprite in cells.
pub const SIZE: usize = 8;

/// The legal skin names a round may pick.
pub const VARIANTS: [&str; 2] = ["tiles", "digits"];

fn rows(code: usize) -> Vec<Vec<u8>> {
    let mask = pick::tile(code, SIZE, [255, 255, 255, 255], [0, 0, 0, 0]);
    let bytes = mask.types().bytes();
    (0..SIZE)
        .map(|r| bytes[r * SIZE..(r + 1) * SIZE].to_vec())
        .collect()
}

/// Builds the named variant's skin, one sprite per design code.
pub fn skin(variant: &str) -> Skin {
    let visuals = (0..pick::NAMES.len())
        .map(|code| {
            let sprite = Visual::none().sprite(rows(code));
            match variant {
                "tiles" => sprite.tinted(Tint::Pen(0)),
                _ => sprite,
            }
        })
        .collect();
    Skin::new(visuals)
}

/// Returns the app's skins paired with their names.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
