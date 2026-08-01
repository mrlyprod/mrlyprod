use super::{Skin, Visual};

const FRUIT: [&str; 9] = ["🍎", "🍋", "🍇", "🍓", "🍑", "🥝", "🍒", "🥥", "🍊"];

pub fn skin(variant: &str, design: &str, colors: &[[u8; 4]], back: [u8; 4]) -> Skin {
    let mut visuals = vec![Visual::none(), Visual::solid(back)];
    for (face, &color) in colors.iter().enumerate() {
        visuals.push(match variant {
            "emojis" => Visual::none().emoji(FRUIT[face % FRUIT.len()]),
            "digits" => Visual::motif(design, color).glyph((face + 1).to_string()),
            _ => Visual::motif(design, color),
        });
    }
    Skin::new(visuals)
}
