use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlynum::factor::gcd;
use mrlynum::lattice::coprime_pairs;

const N: usize = 100;

fn main() -> Result<()> {
    let mut board = Board::square();
    let grid = Grid::new(board.frame(0.08), N, N, 0.0);
    let mut lit = 0u64;
    let mut deepest = 1usize;
    for row in 0..N {
        for col in 0..N {
            let layer = gcd(col + 1, N - row);
            if layer == 1 {
                lit += 1;
                grid.fill(&mut board, col, row, ink::BLUE);
                continue;
            }
            deepest = deepest.max(layer);
            let (x, y, w, h) = grid.cell(col, row);
            let side = w / layer as f64;
            board.rect(
                x + (w - side) / 2.0,
                y + (h - side) / 2.0,
                side,
                side,
                ink::DIM,
            );
        }
    }
    assert_eq!(lit, coprime_pairs(N));
    assert_eq!(deepest, N);
    save("demo-pi", &board)?;
    Ok(())
}
