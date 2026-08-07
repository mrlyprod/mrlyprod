use super::{Skin, Tint, Visual};

pub const VARIANTS: [&str; 2] = ["digits", "emojis"];

pub const BLOCK: usize = 13;

const GLYPHS: [[&str; 5]; 6] = [
    ["00000", "00100", "01110", "00100", "00000"],
    ["01010", "10001", "00100", "10001", "01010"],
    ["10001", "01010", "00100", "01010", "10001"],
    ["00100", "00100", "11111", "00100", "00100"],
    ["10101", "01110", "11111", "01110", "10101"],
    ["00000", "01110", "01110", "01110", "00000"],
];

const EMOJIS: [[&str; 2]; 6] = [
    ["♙", "♟"],
    ["♘", "♞"],
    ["♗", "♝"],
    ["♖", "♜"],
    ["♕", "♛"],
    ["♔", "♚"],
];

fn rows(mask: &[&str; 5]) -> Vec<Vec<u8>> {
    mask.iter()
        .map(|row| row.chars().map(|ch| u8::from(ch == '1')).collect())
        .collect()
}

pub fn skin(variant: &str) -> Skin {
    let mut visuals = Vec::new();
    for parity in 0..2usize {
        visuals.push(Visual::pen(2 + parity));
        for team in 0..2usize {
            for kind in 0..6usize {
                let square = Visual::pen(2 + parity);
                visuals.push(match variant {
                    "emojis" => square.glyph(EMOJIS[kind][team]).tinted(Tint::Pen(team)),
                    _ => square.sprite(rows(&GLYPHS[kind])).tinted(Tint::Pen(team)),
                });
            }
        }
    }
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skin::{Face, Ink};

    #[test]
    fn roles_cover_both_parities_and_teams() {
        let s = skin("digits");
        assert_eq!(s.visuals.len(), 2 * BLOCK);
        assert_eq!(s.visuals[0], Visual::pen(2));
        assert_eq!(s.visuals[BLOCK], Visual::pen(3));
        let Some(Face::Sprite(rows)) = &s.visuals[1].face else {
            panic!("pawn should be a sprite");
        };
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[1], vec![0, 0, 1, 0, 0]);
        assert_eq!(s.visuals[1].face, s.visuals[7].face);
        assert_eq!(s.visuals[1].face, s.visuals[BLOCK + 1].face);
        assert_eq!(s.visuals[1].tint, Some(Tint::Pen(0)));
        assert_eq!(s.visuals[7].tint, Some(Tint::Pen(1)));
        assert_eq!(s.visuals[BLOCK + 7].bg, Some(Ink::Pen(3)));
    }
    #[test]
    fn emojis_split_the_teams() {
        let s = skin("emojis");
        assert_eq!(s.visuals[1].face, Some(Face::Glyph("♙".into())));
        assert_eq!(s.visuals[7].face, Some(Face::Glyph("♟".into())));
        assert_eq!(s.visuals[6].face, Some(Face::Glyph("♔".into())));
        assert_eq!(s.visuals[12].face, Some(Face::Glyph("♚".into())));
        assert_eq!(s.visuals[12].tint, Some(Tint::Pen(1)));
    }
    #[test]
    fn corpus_lists_every_variant() {
        let names: Vec<&str> = corpus().into_iter().map(|(v, _)| v).collect();
        assert_eq!(names, VARIANTS.to_vec());
    }
}
