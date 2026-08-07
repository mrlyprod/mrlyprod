use super::{Skin, Tint, Visual};
use mrlymath::pick;

pub const SIZE: usize = 8;

pub const VARIANTS: [&str; 2] = ["tiles", "digits"];

fn rows(code: usize) -> Vec<Vec<u8>> {
    let mask = pick::tile(code, SIZE, [255, 255, 255, 255], [0, 0, 0, 0]);
    let bytes = mask.types().bytes();
    (0..SIZE)
        .map(|r| bytes[r * SIZE..(r + 1) * SIZE].to_vec())
        .collect()
}

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

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
