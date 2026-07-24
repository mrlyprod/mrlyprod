use super::letters::{bits, strokes, SEQUENCE};
use super::{COLS, ROWS};
use std::collections::BTreeSet;

pub fn writing() -> Vec<Vec<usize>> {
    let mut frames = vec![Vec::new()];
    let mut current: Vec<usize> = Vec::new();
    for (i, &letter) in SEQUENCE.iter().enumerate() {
        let x_offset = 1 + i * 6;
        for stroke in strokes(letter) {
            for &(r, c) in *stroke {
                let index = (1 + r) * COLS + x_offset + c;
                current.push(index);
                let mut frame = current.clone();
                frame.sort_unstable();
                frames.push(frame);
            }
        }
    }
    frames
}

pub fn merging() -> Vec<Vec<usize>> {
    let total = COLS / 2;
    let mut frames: Vec<Vec<usize>> = Vec::new();
    let mut prev: Option<Vec<usize>> = None;
    for i in 0..=total {
        let (phase, progress) = phase(i, total);
        let mut active: BTreeSet<usize> = BTreeSet::new();
        for (idx, &letter) in SEQUENCE.iter().enumerate() {
            let (cx, cy) = position(idx, phase, progress);
            let grid = bits(letter);
            for (r, row) in grid.iter().enumerate() {
                for (c, &bit) in row.iter().enumerate() {
                    if bit == 0 {
                        continue;
                    }
                    let (y, x) = (cy + r as i64, cx + c as i64);
                    if y >= 0 && y < ROWS as i64 && x >= 0 && x < COLS as i64 {
                        active.insert(y as usize * COLS + x as usize);
                    }
                }
            }
        }
        let frame: Vec<usize> = active.into_iter().collect();
        if prev.as_ref() != Some(&frame) {
            frames.push(frame.clone());
            prev = Some(frame);
        }
    }
    frames
}

fn phase(frame: usize, total: usize) -> (usize, f64) {
    let len = total / 4;
    if frame < len {
        (1, frame as f64 / len as f64)
    } else if frame < len * 2 {
        (2, (frame - len) as f64 / len as f64)
    } else if frame < len * 3 {
        (3, (frame - len * 2) as f64 / len as f64)
    } else {
        (4, (frame - len * 3) as f64 / (total - len * 3) as f64)
    }
}

fn lerp(start: i64, end: i64, p: f64) -> i64 {
    start + ((end - start) as f64 * p).trunc() as i64
}

fn position(idx: usize, phase: usize, progress: f64) -> (i64, i64) {
    let start = |i: usize| (1 + i as i64 * 6, 1);
    let (center_x, center_y) = (22, 1);
    let (sx, sy) = start(idx);
    let slide = |(gx, gy): (i64, i64), (tx, ty): (i64, i64)| {
        (lerp(gx, tx, progress), lerp(gy, ty, progress))
    };
    match phase {
        1 if idx == 0 => slide(start(0), start(1)),
        1 if idx == 7 => slide(start(7), start(6)),
        2 if idx <= 1 => slide(start(1), start(2)),
        2 if idx >= 6 => slide(start(6), start(5)),
        3 if idx <= 2 => slide(start(2), start(3)),
        3 if idx >= 5 => slide(start(5), start(4)),
        4 => {
            let anchor = if idx <= 3 { start(3) } else { start(4) };
            slide(anchor, (center_x, center_y))
        }
        _ => (sx, sy),
    }
}
