use mrlyskin::{Skin, Visual};

const FRUIT: [&str; 9] = ["🍎", "🍋", "🍇", "🍓", "🍑", "🥝", "🍒", "🥥", "🍊"];

/// How many distinct card faces every skin must draw.
pub const FACES: usize = 9;

/// The three looks a card face can take.
pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

/// Builds one variant's skin: the empty slot, the card back, then a visual per face.
pub fn skin(variant: &str) -> Skin {
    let mut visuals = vec![Visual::none(), Visual::pen(0)];
    for (face, fruit) in FRUIT.iter().enumerate().take(FACES) {
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(*fruit),
            "digits" => Visual::pen(1 + face).design().glyph((face + 1).to_string()),
            _ => Visual::pen(1 + face).design(),
        });
    }
    Skin::new(visuals)
}

/// Collects every variant's skin under its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
