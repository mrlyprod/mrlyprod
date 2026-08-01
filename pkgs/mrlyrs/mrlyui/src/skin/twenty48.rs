use super::{Skin, Visual};

const LADDER: [&str; 11] = [
    "🌱", "🌿", "🍀", "🌸", "🌼", "🌻", "🍁", "🍄", "🌴", "🌵", "🌟",
];

pub fn skin(variant: &str, design: &str, colors: &[[u8; 4]]) -> Skin {
    let mut visuals = vec![Visual::none()];
    for exp in 1..colors.len() {
        let color = colors[exp];
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(LADDER[(exp - 1).min(LADDER.len() - 1)]),
            "digits" => Visual::motif(design, color).glyph((1u32 << exp).to_string()),
            _ => Visual::motif(design, color),
        });
    }
    Skin::new(visuals)
}
