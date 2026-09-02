use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};

const LOWER: f64 = 0.447_597_813_453;
const UPPER: f64 = 0.640_212_193_8;
const WALLS: [f64; 3] = [0.447_931, 0.5, 0.605_303];

fn arm(board: &mut Board, x: f64, y: f64, w: f64, h: f64) {
    board.rect(x, y, w, h, ink::PANEL);
    let edge = [(x, y), (x + w, y), (x + w, y + h), (x, y + h), (x, y)];
    board.polyline(&edge, 2.0, ink::LINE);
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let (_, cy) = frame.center();
    let thick = frame.h / 3.0;
    let top = cy - thick / 2.0;
    let at = |beta: f64| frame.x + frame.w * beta;

    arm(&mut board, at(0.0), top, at(LOWER) - at(0.0), thick);
    arm(&mut board, at(UPPER), top, at(1.0) - at(UPPER), thick);
    board.rect(at(LOWER), top, at(UPPER) - at(LOWER), thick, ink::ORANGE);

    for wall in WALLS {
        let x = at(wall);
        board.rect(x - 1.0, frame.y, 2.0, frame.h, ink::fade(ink::DIM, 0.85));
        board.rect(x - 1.0, top, 2.0, thick, ink::LINE);
    }
    save("paper-lemma-b-pincer", &board)?;
    Ok(())
}
