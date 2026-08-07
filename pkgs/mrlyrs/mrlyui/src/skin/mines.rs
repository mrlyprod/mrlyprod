use super::{Skin, Visual};

pub const FLAG: usize = 11;

pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

pub fn skin(variant: &str) -> Skin {
    let mut visuals = vec![Visual::pen(0)];
    for n in 0..=8usize {
        let base = match variant {
            "emojis" => Visual::none(),
            _ => Visual::pen(1 + n).design(),
        };
        visuals.push(match (variant, n) {
            (_, 0) | ("tiles", _) => base,
            _ => base.glyph(n.to_string()),
        });
    }
    visuals.push(match variant {
        "emojis" => Visual::pen(10).emoji("💣"),
        "digits" => Visual::pen(10).glyph("X"),
        _ => Visual::pen(10),
    });
    visuals.push(Visual::none().emoji("⛳"));
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
