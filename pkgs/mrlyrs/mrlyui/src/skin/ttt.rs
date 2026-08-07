use super::{Skin, Visual};

pub const MARKS: [(&str, &str); 2] = [("X", "\u{274c}"), ("O", "\u{2b55}")];

pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

pub fn skin(variant: &str) -> Skin {
    let mut visuals = vec![Visual::none()];
    for (i, (glyph, emoji)) in MARKS.iter().enumerate() {
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(*emoji),
            "digits" => Visual::pen(i).design().glyph(*glyph),
            _ => Visual::pen(i).design(),
        });
    }
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
