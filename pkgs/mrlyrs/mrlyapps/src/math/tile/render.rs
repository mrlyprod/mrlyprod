use mrlymath::two::Cell2d;
use mrlyui::frame;
use serde_json::Value as Json;

pub fn two_tone(cell: &Cell2d, board: [u8; 4], fill: [u8; 4]) -> Vec<[u8; 4]> {
    cell.types()
        .bytes()
        .iter()
        .map(|&v| if v != 0 { fill } else { board })
        .collect()
}

pub fn blank(board: [u8; 4]) -> Json {
    frame::field(1, 1, vec![board], board).fact()
}
