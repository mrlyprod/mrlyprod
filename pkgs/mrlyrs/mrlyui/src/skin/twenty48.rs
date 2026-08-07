use super::{Skin, Visual};

pub const CAP: usize = 18;

pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

const LADDER: [&str; 11] = [
    "🌱", "🌿", "🍀", "🌸", "🌼", "🌻", "🍁", "🍄", "🌴", "🌵", "🌟",
];

pub fn skin(variant: &str) -> Skin {
    let mut visuals = vec![Visual::none()];
    for exp in 1..=CAP {
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(LADDER[(exp - 1).min(LADDER.len() - 1)]),
            "digits" => Visual::pen(exp - 1)
                .design()
                .glyph((1u32 << exp).to_string()),
            _ => Visual::pen(exp - 1).design(),
        });
    }
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
