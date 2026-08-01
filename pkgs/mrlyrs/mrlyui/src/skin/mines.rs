use super::{Skin, Visual};

pub const FLAG: usize = 11;

pub fn skin(
    variant: &str,
    design: &str,
    colors: &[[u8; 4]],
    hidden: [u8; 4],
    mine: [u8; 4],
) -> Skin {
    let mut visuals = vec![Visual::solid(hidden)];
    for n in 0..=8usize {
        let color = colors.get(n).copied().unwrap_or(hidden);
        let base = match variant {
            "emojis" => Visual::none(),
            _ => Visual::motif(design, color),
        };
        visuals.push(match (variant, n) {
            (_, 0) | ("tiles", _) => base,
            _ => base.glyph(n.to_string()),
        });
    }
    visuals.push(match variant {
        "emojis" => Visual::solid(mine).emoji("💣"),
        "digits" => Visual::solid(mine).glyph("X"),
        _ => Visual::solid(mine),
    });
    visuals.push(Visual::none().emoji("⛳"));
    Skin::new(visuals)
}
