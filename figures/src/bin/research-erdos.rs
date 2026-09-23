use figures::{ink, save, Board, Color};
use mrlyrs::core::error::Result;

const BASES: [u64; 3] = [3, 4, 5];
const LEAST: u32 = 3;
const LIMIT: u64 = 10_000_000;
const TERMS: usize = 29;
const TOTAL: usize = 24_973_824;
const FROBENIUS: usize = 4_330_731;
const MISSING: usize = 45_704;
const WIDTH: usize = 860;
const PITCH: usize = 29;
const GAP: usize = 3;
const FLOOR: f64 = 0.15;
const REACH: f64 = 0.6;

fn powers() -> Vec<usize> {
    let mut terms: Vec<usize> = BASES
        .iter()
        .flat_map(|&b| (LEAST..).map(move |j| b.pow(j)).take_while(|&p| p <= LIMIT))
        .map(|p| p as usize)
        .collect();
    terms.sort_unstable();
    terms
}

fn shift_or(bits: &mut [u64], a: usize, top: usize) {
    let (q, r) = (a / 64, a % 64);
    for i in (q..=top / 64).rev() {
        let mut word = bits[i - q] << r;
        if r > 0 && i > q {
            word |= bits[i - q - 1] >> (64 - r);
        }
        bits[i] |= word;
    }
}

fn has(bits: &[u64], k: usize) -> bool {
    bits[k / 64] >> (k % 64) & 1 == 1
}

fn count(bits: &[u64], lo: usize, hi: usize) -> usize {
    if lo >= hi {
        return 0;
    }
    let (a, b) = (lo / 64, (hi - 1) / 64);
    let head = !0u64 << (lo % 64);
    let tail = !0u64 >> (63 - (hi - 1) % 64);
    if a == b {
        return (bits[a] & head & tail).count_ones() as usize;
    }
    let inner: u32 = bits[a + 1..b].iter().map(|w| w.count_ones()).sum();
    (inner + (bits[a] & head).count_ones() + (bits[b] & tail).count_ones()) as usize
}

fn shares(bits: &[u64], total: usize) -> Vec<f64> {
    let span = total + 1;
    let mut row = vec![0.0; WIDTH];
    for c in 0..WIDTH / 2 {
        let (lo, hi) = if span >= WIDTH {
            (c * span / WIDTH, ((c + 1) * span).div_ceil(WIDTH))
        } else {
            let k = (2 * c + 1) * span / (2 * WIDTH);
            (k, k + 1)
        };
        let share = count(bits, lo, hi) as f64 / (hi - lo) as f64;
        row[c] = share;
        row[WIDTH - 1 - c] = share;
    }
    row
}

fn tone(share: f64) -> Option<Color> {
    if share >= 1.0 {
        Some(ink::blue())
    } else if share > 0.0 {
        Some(ink::mix(ink::ground(), ink::dim(), FLOOR + REACH * share))
    } else {
        None
    }
}

fn paint(board: &mut Board, row: &[f64], x: usize, y: usize, h: usize) {
    let mut start = 0;
    for c in 1..=row.len() {
        if c < row.len() && tone(row[c]) == tone(row[start]) {
            continue;
        }
        if let Some(tone) = tone(row[start]) {
            board.rect(
                (x + start) as f64,
                y as f64,
                (c - start) as f64,
                h as f64,
                tone,
            );
        }
        start = c;
    }
}

fn main() -> Result<()> {
    let terms = powers();
    assert_eq!(terms.len(), TERMS);
    assert_eq!((terms[0], terms[TERMS - 1]), (27, 9_765_625));
    let mut board = Board::square();
    let x = (board.width - WIDTH) / 2;
    let top = (board.height - (TERMS * PITCH - GAP)) / 2;
    let mut bits = vec![0u64; TOTAL / 64 + 2];
    bits[0] = 1;
    let mut total = 0;
    for (n, &a) in terms.iter().enumerate() {
        total += a;
        shift_or(&mut bits, a, total);
        assert!(has(&bits, 0) && has(&bits, total));
        let row = shares(&bits, total);
        paint(&mut board, &row, x, top + n * PITCH, PITCH - GAP);
    }
    assert_eq!(total, TOTAL);
    assert!((0..=TOTAL / 2).all(|k| has(&bits, k) == has(&bits, TOTAL - k)));
    let hole = (0..=TOTAL / 2).rev().find(|&k| !has(&bits, k));
    assert_eq!(hole, Some(FROBENIUS));
    assert_eq!(FROBENIUS - count(&bits, 1, FROBENIUS + 1), MISSING);
    assert_eq!(
        count(&bits, FROBENIUS + 1, TOTAL - FROBENIUS),
        TOTAL - 2 * FROBENIUS - 1
    );
    save("research-erdos", &board)?;
    Ok(())
}
