use figures::{ink, save, Board};
use mrlyrs::core::error::Result;

const LEVEL: usize = 4;
const LAGS: usize = 31;
const PEAK: u64 = 5;
const MARGIN: f64 = 0.08;
const FILL: f64 = 0.46;

fn main() -> Result<()> {
    let field = stern_field(LEVEL);
    assert_eq!(field.len(), LAGS);
    assert_eq!(field.iter().sum::<u64>(), 81);
    assert_eq!(field.iter().max(), Some(&PEAK));
    let peaks: Vec<usize> = (0..LAGS).filter(|&n| field[n] == PEAK).collect();
    assert_eq!(peaks, vec![10, 12, 18, 20]);
    assert_eq!(field.iter().filter(|&&r| r == 1).count(), 2 * LEVEL + 1);

    let mut board = Board::square();
    let frame = board.frame(MARGIN);
    let cell = frame.w / LAGS as f64;
    let top = cell * FILL;
    let mut marked = 0usize;
    for m in 0..LAGS {
        for n in 0..LAGS {
            let count = field[m] * field[n];
            let r = top * (count as f64 / (PEAK * PEAK) as f64).sqrt();
            let (x, y) = (
                frame.x + (n as f64 + 0.5) * cell,
                frame.y + (m as f64 + 0.5) * cell,
            );
            let color = if count == PEAK * PEAK {
                marked += 1;
                ink::yellow()
            } else {
                ink::blue()
            };
            board.disc(x, y, r, color);
        }
    }
    assert_eq!(marked, 16);
    save("research-dilations", &board)?;
    Ok(())
}

fn stern_field(level: usize) -> Vec<u64> {
    let mut r = vec![1u64];
    for j in 0..level {
        let s = 1usize << j;
        let mut out = vec![0u64; r.len() + 2 * s];
        for (n, &v) in r.iter().enumerate() {
            for k in 0..3 {
                out[n + k * s] += v;
            }
        }
        r = out;
    }
    r
}
