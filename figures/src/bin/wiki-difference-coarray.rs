use figures::{ink, save, Board, Color};
use mrlyrs::core::error::Result;

const RULER: [i64; 4] = [0, 1, 4, 6];
const SPAN: i64 = 6;
const PAIRS: usize = 16;
const LAGS: usize = 13;
const OFF: f64 = 1.4;
const GAP: f64 = 1.5;
const SHRINK: f64 = 0.84;
const BASE: f64 = 0.2;
const RULE: f64 = 2.0;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let weights = weights();
    assert_eq!(weights.len(), LAGS);
    assert!(weights.iter().all(|&w| w > 0));
    assert_eq!(weights[SPAN as usize], RULER.len());
    assert_eq!(weights.iter().filter(|&&w| w == 1).count(), LAGS - 1);
    assert_eq!(weights.iter().sum::<usize>(), PAIRS);
    let tall = *weights.iter().max().unwrap_or(&0) as f64;
    let top = -OFF - SHRINK;
    let floor = 2.0 * SPAN as f64 + 1.0 + GAP + tall;
    let base = floor + BASE;
    let wide = 2.0 * (SPAN as f64 + OFF + SHRINK);
    let side = frame.w.min(frame.h) - RULE / 2.0;
    let h = side / wide.max(base - top);
    let bottom = base + RULE / 2.0 / h;
    let (cx, cy) = frame.center();
    let y0 = cy - h * (top + bottom) / 2.0;
    let at = |u: f64, v: f64| (cx + u * h, y0 + v * h);
    let lit = |p: i64| RULER.contains(&p);
    let mut pairs = 0;
    for i in 0..=SPAN {
        for j in 0..=SPAN {
            let on = lit(i) && lit(j);
            pairs += on as usize;
            let color = if on { ink::blue() } else { ink::line() };
            diamond(&mut board, at((j - i) as f64, (i + j) as f64), h, color);
        }
    }
    assert_eq!(pairs, PAIRS);
    for p in 0..=SPAN {
        let color = if lit(p) { ink::yellow() } else { ink::line() };
        let q = p as f64;
        diamond(&mut board, at(-OFF - q, q - OFF), h, color);
        diamond(&mut board, at(q + OFF, q - OFF), h, color);
    }
    let block = h * SHRINK;
    for (k, &w) in weights.iter().enumerate() {
        let (x, _) = at(k as f64 - SPAN as f64, 0.0);
        for s in 0..w {
            let (_, y) = at(0.0, floor - s as f64 - 0.5);
            board.rect(x - block / 2.0, y - block / 2.0, block, block, ink::blue());
        }
    }
    let (left, y) = at(-(SPAN as f64) - 0.5, base);
    let (right, _) = at(SPAN as f64 + 0.5, base);
    board.segment((left, y), (right, y), RULE, ink::line());
    save("wiki-difference-coarray", &board)?;
    Ok(())
}

fn weights() -> Vec<usize> {
    let mut w = vec![0; 2 * SPAN as usize + 1];
    for a in RULER {
        for b in RULER {
            w[(b - a + SPAN) as usize] += 1;
        }
    }
    w
}

fn diamond(board: &mut Board, (x, y): (f64, f64), h: f64, color: Color) {
    let r = h * SHRINK;
    board.polygon(&[(x, y - r), (x + r, y), (x, y + r), (x - r, y)], color);
}
