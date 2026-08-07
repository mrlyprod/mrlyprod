use super::{Skin, Visual};

const FRUIT: [&str; 9] = ["🍎", "🍋", "🍇", "🍓", "🍑", "🥝", "🍒", "🥥", "🍊"];

pub const FACES: usize = 9;

pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

pub fn skin(variant: &str) -> Skin {
    let mut visuals = vec![Visual::none(), Visual::pen(0)];
    for face in 0..FACES {
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(FRUIT[face]),
            "digits" => Visual::pen(1 + face).design().glyph((face + 1).to_string()),
            _ => Visual::pen(1 + face).design(),
        });
    }
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
