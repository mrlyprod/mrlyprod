use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};

const CELLS: usize = 4;
const PATTERNS: usize = 16;
const CLASSES: usize = 6;
const FIXED: usize = 48;

fn moves() -> [[usize; CELLS]; 8] {
    let seat = |row: usize, col: usize| 2 * row + col;
    let mut out = [[0usize; CELLS]; 8];
    for row in 0..2 {
        for col in 0..2 {
            let from = seat(row, col);
            out[0][from] = seat(row, col);
            out[1][from] = seat(col, 1 - row);
            out[2][from] = seat(1 - row, 1 - col);
            out[3][from] = seat(1 - col, row);
            out[4][from] = seat(row, 1 - col);
            out[5][from] = seat(1 - row, col);
            out[6][from] = seat(col, row);
            out[7][from] = seat(1 - col, 1 - row);
        }
    }
    out
}

fn act(mask: usize, plan: &[usize; CELLS]) -> usize {
    (0..CELLS)
        .filter(|&i| mask >> i & 1 == 1)
        .fold(0, |acc, i| acc | 1 << plan[i])
}

fn main() -> Result<()> {
    let plans = moves();
    let fixed: usize = plans
        .iter()
        .map(|plan| {
            (0..PATTERNS)
                .filter(|&mask| act(mask, plan) == mask)
                .count()
        })
        .sum();
    assert_eq!(fixed, FIXED);
    assert_eq!(fixed / plans.len(), CLASSES);
    let mut seen = [false; PATTERNS];
    let mut classes: Vec<Vec<usize>> = Vec::new();
    for mask in 0..PATTERNS {
        if seen[mask] {
            continue;
        }
        let mut orbit: Vec<usize> = plans.iter().map(|plan| act(mask, plan)).collect();
        orbit.sort_unstable();
        orbit.dedup();
        for &member in &orbit {
            seen[member] = true;
        }
        classes.push(orbit);
    }
    assert_eq!(classes.len(), CLASSES);
    assert_eq!(classes.iter().map(Vec::len).sum::<usize>(), PATTERNS);
    classes.sort_by_key(|orbit| (orbit[0].count_ones(), PATTERNS - orbit.len()));
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let rows = frame.rows(CLASSES);
    let widest = classes.iter().map(Vec::len).max().unwrap_or(1);
    let slot = frame.w / widest as f64;
    let stamp = (slot * 0.62).min(rows[0].h * 0.74);
    let pad = stamp * 0.06;
    for (class, row) in classes.iter().zip(rows.iter()) {
        let run = class.len() as f64 * slot;
        let left = row.x + (row.w - run) / 2.0;
        let top = row.y + (row.h - stamp) / 2.0;
        for (place, &mask) in class.iter().enumerate() {
            let x = left + place as f64 * slot + (slot - stamp) / 2.0;
            let half = (stamp - pad) / 2.0;
            for cell in 0..CELLS {
                let tone = if mask >> cell & 1 == 1 {
                    ink::blue()
                } else {
                    ink::dim()
                };
                board.rect(
                    x + (cell % 2) as f64 * (half + pad),
                    top + (cell / 2) as f64 * (half + pad),
                    half,
                    half,
                    tone,
                );
            }
        }
    }
    save("wiki-burnsides-lemma", &board)?;
    Ok(())
}
