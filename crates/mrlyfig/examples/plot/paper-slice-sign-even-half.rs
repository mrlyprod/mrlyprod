use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};

const THREE: [usize; 20] = [
    0, 1, 2, 4, 6, 8, 11, 14, 18, 22, 27, 32, 38, 44, 50, 57, 64, 72, 81, 89,
];
const OTHERS: [[usize; 3]; 4] = [[5, 4, 16], [7, 4, 26], [9, 4, 0], [11, 4, 0]];

fn depth(first: usize, second: usize, dim: usize) -> f64 {
    if second > 0 && dim >= second {
        2.0
    } else if dim >= first {
        1.0
    } else {
        0.0
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let peak = *THREE.last().unwrap() as f64;
    assert_eq!(THREE.len(), 20);
    assert_eq!(peak, 89.0);
    let at = |dim: f64, depth: f64| {
        (
            frame.x + frame.w * (dim - 2.0) / 38.0,
            frame.y + frame.h * (1.0 - depth / peak),
        )
    };
    board.rect(frame.x, frame.y + frame.h, frame.w, 2.0, ink::LINE);

    for (order, row) in OTHERS.iter().enumerate() {
        let mut steps = Vec::new();
        for index in 0..THREE.len() {
            let dim = 2.0 + 2.0 * index as f64;
            let value = depth(row[1], row[2], dim as usize);
            steps.push(at(dim, value));
            steps.push(at((dim + 2.0).min(40.0), value));
        }
        board.polyline(&steps, 3.0, ink::fade(ink::DIM, 0.95 - 0.1 * order as f64));
    }

    let curve: Vec<(f64, f64)> = THREE
        .iter()
        .enumerate()
        .map(|(index, k)| at(2.0 + 2.0 * index as f64, *k as f64))
        .collect();
    board.polyline(&curve, 3.0, ink::ORANGE);
    for point in &curve {
        board.disc(point.0, point.1, 8.0, ink::ORANGE);
    }
    save("paper-slice-sign-even-half", &board)?;
    Ok(())
}
