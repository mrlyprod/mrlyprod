use mrlycore::colors::hex;
use mrlycore::{json, Json};
use mrlymath::two::Cell2d;
use mrlyui::skin::pixel::PENS;

pub fn two_tone(cell: &Cell2d, fill: [u8; 4]) -> Vec<[u8; 4]> {
    cell.types()
        .bytes()
        .iter()
        .map(|&v| if v != 0 { fill } else { [0, 0, 0, 0] })
        .collect()
}

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

pub fn blank() -> Json {
    cells(1, 1, vec![[0, 0, 0, 0]])
}
