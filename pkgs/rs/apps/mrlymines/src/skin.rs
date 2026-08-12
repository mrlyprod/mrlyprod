use mrlyskin::{Skin, Visual};

/// The visual slot the flag marker sits in, past the covered cell, the counts and the mine.
pub const FLAG: usize = 11;

/// The three looks the field can wear: tiles, emojis and digits.
pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

/// Builds one variant's skin: a covered cell, the nine counts, the mine and the flag.
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

/// Collects every variant's skin under its name.
pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyskin::Face;

    fn dressed(variant: &str) -> Skin {
        let (_, skin) = corpus()
            .into_iter()
            .find(|(name, _)| *name == variant)
            .expect("a variant");
        skin
    }

    #[test]
    fn mines_variants_dress_the_roles() {
        let tiles = dressed("tiles");
        assert_eq!(tiles.visuals.len(), 12);
        assert_eq!(tiles.visuals[0], Visual::pen(0));
        assert_eq!(tiles.visuals[2], Visual::pen(2).design());
        assert_eq!(tiles.visuals[10], Visual::pen(10));
        let digits = dressed("digits");
        assert_eq!(digits.visuals[2].face, Some(Face::Glyph("1".into())));
        assert_eq!(digits.visuals[1].face, None);
        assert_eq!(digits.visuals[10].face, Some(Face::Glyph("X".into())));
        let emojis = dressed("emojis");
        assert_eq!(emojis.visuals[1], Visual::none());
        assert_eq!(emojis.visuals[3].face, Some(Face::Glyph("2".into())));
        assert_eq!(emojis.visuals[3].bg, None);
        assert_eq!(emojis.visuals[10].face, Some(Face::Emoji("💣".into())));
        assert_eq!(emojis.visuals[FLAG].face, Some(Face::Emoji("⛳".into())));
    }
}
