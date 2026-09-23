use figures::{ink, save, Board};
use mrlyrs::core::error::Result;

const STAGES: usize = 6;
const KEPT: usize = 63;
const CUT: usize = 31;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let bar = frame.h / (STAGES as f64 + (STAGES - 1) as f64 * 0.9);
    let pitch = bar * 1.9;
    let mut kept = 0usize;
    let mut cut = 0usize;
    for stage in 0..STAGES {
        let y = frame.y + stage as f64 * pitch;
        let side = 3f64.powi(-(stage as i32));
        let row = lefts(stage);
        assert_eq!(row.len(), 1 << stage);
        let length: f64 = row.len() as f64 * side;
        assert!((length - (2.0f64 / 3.0).powi(stage as i32)).abs() < 1e-12);
        if stage > 0 {
            for &a in &lefts(stage - 1) {
                let x = frame.x + frame.w * (a + side);
                board.rect(x, y, frame.w * side, bar, ink::dim());
                cut += 1;
            }
        }
        for &a in &row {
            board.rect(frame.x + frame.w * a, y, frame.w * side, bar, ink::blue());
            kept += 1;
        }
    }
    assert_eq!(kept, KEPT);
    assert_eq!(cut, CUT);
    save("wiki-cantor-set", &board)?;
    Ok(())
}

fn lefts(stage: usize) -> Vec<f64> {
    (0..1usize << stage)
        .map(|i| {
            (0..stage)
                .filter(|&l| (i >> (stage - 1 - l)) & 1 == 1)
                .map(|l| 2.0 * 3f64.powi(-(l as i32 + 1)))
                .sum()
        })
        .collect()
}
