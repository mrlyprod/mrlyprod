use crate::skin::PENS;
use mrlycore::colors::hex;
use mrlycore::{json, Json};
use mrlymath::two::Cell2d;

/// Colors a cell in one ink, leaving its void transparent.
pub fn two_tone(cell: &Cell2d, fill: [u8; 4]) -> Vec<[u8; 4]> {
    cell.types()
        .bytes()
        .iter()
        .map(|&v| if v != 0 { fill } else { [0, 0, 0, 0] })
        .collect()
}

/// Packs a colored grid into the cells fact, one pen per distinct color up to the pen limit.
pub fn cells(width: usize, height: usize, colors: Vec<[u8; 4]>) -> Json {
    let clear = [0, 0, 0, 0];
    let mut pens = vec![clear];
    let mut ids = vec![vec![0u8; width]; height];
    for (i, color) in colors.iter().enumerate() {
        let id = if color[3] == 0 {
            0
        } else {
            match pens.iter().position(|p| p == color) {
                Some(at) => at,
                None if pens.len() < PENS => {
                    pens.push(*color);
                    pens.len() - 1
                }
                None => PENS - 1,
            }
        };
        ids[i / width][i % width] = id as u8;
    }
    pens.resize(PENS, clear);
    json!({
        "ids": ids,
        "skin": "tiles",
        "pens": pens.iter().map(|&p| hex(p)).collect::<Vec<_>>(),
    })
}

/// Builds the empty cells fact shown when a tile will not build.
pub fn blank() -> Json {
    cells(1, 1, vec![[0, 0, 0, 0]])
}
