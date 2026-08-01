use super::{Skin, Visual};

pub const MARKS: [(&str, &str); 2] = [("X", "\u{274c}"), ("O", "\u{2b55}")];

pub fn skin(variant: &str, design: &str, colors: [[u8; 4]; 2]) -> Skin {
    let mut visuals = vec![Visual::none()];
    for (i, (glyph, emoji)) in MARKS.iter().enumerate() {
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(*emoji),
            "digits" => Visual::motif(design, colors[i]).glyph(*glyph),
            _ => Visual::motif(design, colors[i]),
        });
    }
    Skin::new(visuals)
}
