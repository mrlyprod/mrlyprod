use mrlyskin::{Skin, Visual};

/// The two marks, each as a glyph and an emoji.
pub const MARKS: [(&str, &str); 2] = [("X", "\u{274c}"), ("O", "\u{2b55}")];

/// The three looks a mark can take.
pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

/// Builds the named variant's skin: one blank visual, then one per mark.
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

/// Collects every variant's skin under its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
