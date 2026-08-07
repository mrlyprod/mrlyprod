use super::{Skin, Visual};

pub const FACES: usize = 20;

pub const VARIANTS: [&str; 3] = ["tiles", "emojis", "digits"];

const DIE: [&str; 6] = [
    "\u{2680}", "\u{2681}", "\u{2682}", "\u{2683}", "\u{2684}", "\u{2685}",
];

fn pips(value: usize) -> &'static [(usize, usize)] {
    match value {
        1 => &[(1, 1)],
        2 => &[(0, 0), (2, 2)],
        3 => &[(0, 0), (1, 1), (2, 2)],
        4 => &[(0, 0), (0, 2), (2, 0), (2, 2)],
        5 => &[(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        6 => &[(0, 0), (0, 2), (1, 0), (1, 2), (2, 0), (2, 2)],
        _ => &[],
    }
}

fn rows(value: usize) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0u8; 3]; 3];
    for &(r, c) in pips(value) {
        out[r][c] = 1;
    }
    out
}

pub fn skin(variant: &str) -> Skin {
    let mut visuals = vec![Visual::none()];
    for face in 1..=FACES {
        visuals.push(match variant {
            "tiles" if face <= 6 => Visual::none().sprite(rows(face)),
            "emojis" if face <= 6 => Visual::none().glyph(DIE[face - 1]),
            _ => Visual::none().glyph(face.to_string()),
        });
    }
    Skin::new(visuals)
}

pub fn corpus() -> Vec<(&'static str, Skin)> {
    VARIANTS.into_iter().map(|v| (v, skin(v))).collect()
}
