use super::{Skin, Visual};

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

pub fn glyphs() -> [[u8; 25]; 6] {
    let mut out = [[0u8; 25]; 6];
    for (kind, rows) in GLYPHS.iter().enumerate() {
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                out[kind][y * 5 + x] = u8::from(ch == '1');
            }
        }
    }
    out
}

fn rows(mask: &[u8; 25]) -> Vec<Vec<u8>> {
    (0..5).map(|y| mask[y * 5..y * 5 + 5].to_vec()).collect()
}

pub fn skin(variant: &str, glyphs: &[[u8; 25]; 6]) -> Skin {
    let mut visuals = vec![Visual::none()];
    for team in 0..2usize {
        for (kind, mask) in glyphs.iter().enumerate() {
            visuals.push(match variant {
                "emojis" => Visual::none().emoji(EMOJIS[kind][team]),
                _ => Visual::none().sprite(rows(mask)),
            });
        }
    }
    Skin::new(visuals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skin::Face;

    #[test]
    fn roles_cover_empty_then_both_teams() {
        let s = skin("digits", &glyphs());
        assert_eq!(s.visuals.len(), 13);
        assert_eq!(s.visuals[0], Visual::none());
        let Some(Face::Sprite(rows)) = &s.visuals[1].face else {
            panic!("pawn should be a sprite");
        };
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[1], vec![0, 0, 1, 0, 0]);
        assert_eq!(s.visuals[1].face, s.visuals[7].face);
    }
    #[test]
    fn emojis_split_the_teams() {
        let s = skin("emojis", &glyphs());
        assert_eq!(s.visuals[1].face, Some(Face::Emoji("♙".into())));
        assert_eq!(s.visuals[7].face, Some(Face::Emoji("♟".into())));
        assert_eq!(s.visuals[6].face, Some(Face::Emoji("♔".into())));
        assert_eq!(s.visuals[12].face, Some(Face::Emoji("♚".into())));
    }
    #[test]
    fn scrambled_masks_flow_through() {
        let mut masks = glyphs();
        masks[0] = [1; 25];
        let s = skin("digits", &masks);
        let Some(Face::Sprite(rows)) = &s.visuals[1].face else {
            panic!("pawn should be a sprite");
        };
        assert_eq!(rows[0], vec![1, 1, 1, 1, 1]);
    }
}
