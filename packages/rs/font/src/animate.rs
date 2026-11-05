use super::paths::strokes;
use super::raster::{Block, Layout, layout};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const FPS: usize = 25;

pub const HOLD: usize = 25;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anim {
    pub rows: usize,
    pub cols: usize,
    pub fps: usize,
    pub frames: Vec<Vec<usize>>,
}

pub fn animate(text: &str, pad: usize) -> Anim {
    let laid = layout(text);
    let (rows, cols) = board(&laid, pad);
    let mut frames = vec![Vec::new()];
    let mut current: Vec<usize> = Vec::new();
    for block in &laid.blocks {
        for (r, c) in strokes(block.char).into_iter().flatten() {
            current.push((pad + block.offset + r) * cols + (pad + block.col + c));
            let mut frame = current.clone();
            frame.sort_unstable();
            frames.push(frame);
        }
    }
    Anim {
        rows,
        cols,
        fps: FPS,
        frames,
    }
}

pub fn merge(text: &str, pad: usize) -> Vec<Vec<usize>> {
    let laid = layout(text);
    let (rows, cols) = board(&laid, pad);
    let n = laid.blocks.len();
    let phases = n / 2;
    let starts: Vec<(i64, i64)> = laid
        .blocks
        .iter()
        .map(|b| ((pad + b.col) as i64, (pad + b.offset) as i64))
        .collect();
    let targets: Vec<(i64, i64)> = laid
        .blocks
        .iter()
        .map(|b| (((cols - b.width()) / 2) as i64, (pad + b.offset) as i64))
        .collect();
    if phases == 0 {
        return vec![stamp(&laid.blocks, &starts, rows, cols)];
    }
    let total = cols / 2;
    let mut frames: Vec<Vec<usize>> = Vec::new();
    let mut prev: Option<Vec<usize>> = None;
    for i in 0..=total {
        let (phase, progress) = beat(i, total, phases);
        let spots: Vec<(i64, i64)> = (0..n)
            .map(|idx| place(idx, n, phases, phase, progress, &starts, &targets))
            .collect();
        let frame = stamp(&laid.blocks, &spots, rows, cols);
        if prev.as_ref() != Some(&frame) {
            frames.push(frame.clone());
            prev = Some(frame);
        }
    }
    frames
}

pub fn cycle(write: &Anim, merge: &[Vec<usize>], hold: usize) -> Anim {
    let mut frames: Vec<Vec<usize>> = Vec::new();
    let rest = |frame: Option<&Vec<usize>>, out: &mut Vec<Vec<usize>>| {
        if let Some(frame) = frame {
            for _ in 0..hold {
                out.push(frame.clone());
            }
        }
    };
    frames.extend(write.frames.iter().cloned());
    rest(write.frames.last(), &mut frames);
    frames.extend(merge.iter().cloned());
    rest(merge.last(), &mut frames);
    frames.extend(merge.iter().rev().cloned());
    rest(merge.first(), &mut frames);
    frames.extend(write.frames.iter().rev().cloned());
    rest(write.frames.first(), &mut frames);
    Anim {
        rows: write.rows,
        cols: write.cols,
        fps: write.fps,
        frames,
    }
}

fn board(laid: &Layout, pad: usize) -> (usize, usize) {
    if laid.blocks.is_empty() {
        return (0, 0);
    }
    (laid.height + 2 * pad, laid.width + 2 * pad)
}

fn stamp(blocks: &[Block], spots: &[(i64, i64)], rows: usize, cols: usize) -> Vec<usize> {
    let mut active: BTreeSet<usize> = BTreeSet::new();
    for (block, &(cx, cy)) in blocks.iter().zip(spots) {
        for (r, row) in block.rows.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                if ch != '1' {
                    continue;
                }
                let (y, x) = (cy + r as i64, cx + c as i64);
                if y >= 0 && y < rows as i64 && x >= 0 && x < cols as i64 {
                    active.insert(y as usize * cols + x as usize);
                }
            }
        }
    }
    active.into_iter().collect()
}

fn beat(frame: usize, total: usize, phases: usize) -> (usize, f64) {
    let len = total / phases;
    for p in 1..phases {
        if frame < len * p {
            return (p, (frame - len * (p - 1)) as f64 / len as f64);
        }
    }
    let done = len * (phases - 1);
    (phases, (frame - done) as f64 / (total - done) as f64)
}

fn lerp(start: i64, end: i64, p: f64) -> i64 {
    start + ((end - start) as f64 * p).trunc() as i64
}

fn place(
    idx: usize,
    n: usize,
    phases: usize,
    phase: usize,
    progress: f64,
    starts: &[(i64, i64)],
    targets: &[(i64, i64)],
) -> (i64, i64) {
    let slide = |a: (i64, i64), b: (i64, i64)| (lerp(a.0, b.0, progress), lerp(a.1, b.1, progress));
    if phase < phases {
        if idx < phase {
            return slide(starts[phase - 1], starts[phase]);
        }
        if idx >= n - phase {
            return slide(starts[n - phase], starts[n - phase - 1]);
        }
        return starts[idx];
    }
    let anchor = if idx < phases {
        starts[phases - 1]
    } else if idx >= n - phases {
        starts[n - phases]
    } else {
        starts[idx]
    };
    slide(anchor, targets[idx])
}
