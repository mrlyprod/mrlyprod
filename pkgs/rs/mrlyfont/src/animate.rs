use super::paths::sequence;
use super::raster::{layout, Block, Layout};
use std::collections::BTreeSet;

/// The playback rate of every animation, in frames per second.
pub const FPS: usize = 25;

/// The default number of frames a cycle rests between movements.
pub const HOLD: usize = 25;

/// A frame-by-frame animation over a fixed board.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anim {
    /// The board height in cells.
    pub rows: usize,
    /// The board width in cells.
    pub cols: usize,
    /// The playback rate in frames per second.
    pub fps: usize,
    /// The frames, each a sorted list of lit row-major cell indices.
    pub frames: Vec<Vec<usize>>,
}

/// Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster.
///
/// ```
/// let write = mrlyfont::animate("MRLYPROD", 1);
/// assert_eq!((write.rows, write.cols, write.fps), (7, 49, 25));
/// assert!(write.frames[0].is_empty());
/// ```
pub fn animate(text: &str, pad: usize) -> Anim {
    let laid = layout(text);
    let (rows, cols) = board(&laid, pad);
    let mut frames = vec![Vec::new()];
    let mut current: Vec<usize> = Vec::new();
    for block in &laid.blocks {
        for (r, c) in sequence(block.char, &block.rows).into_iter().flatten() {
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

/// Folds the written text's glyphs, frame by frame, into one centered stack.
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

/// Chains the write, the merge and their reversals into one loop, resting hold frames after each.
pub fn cycle(write: &Anim, merge: &[Vec<usize>], hold: usize) -> Anim {
    let mut frames: Vec<Vec<usize>> = Vec::new();
    let rest = |frame: &Vec<usize>, out: &mut Vec<Vec<usize>>| {
        for _ in 0..hold {
            out.push(frame.clone());
        }
    };
    frames.extend(write.frames.iter().cloned());
    rest(write.frames.last().unwrap(), &mut frames);
    frames.extend(merge.iter().cloned());
    rest(merge.last().unwrap(), &mut frames);
    frames.extend(merge.iter().rev().cloned());
    rest(merge.first().unwrap(), &mut frames);
    frames.extend(write.frames.iter().rev().cloned());
    rest(write.frames.first().unwrap(), &mut frames);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raster;
    const WORDMARK: &str = "MRLYPROD";

    fn lit(rows: &[Vec<u8>]) -> usize {
        rows.iter().flatten().filter(|&&v| v == 1).count()
    }

    #[test]
    fn the_wordmark_board_is_seven_by_forty_nine() {
        let write = animate(WORDMARK, 1);
        assert_eq!((write.rows, write.cols, write.fps), (7, 49, 25));
    }

    #[test]
    fn writing_starts_empty_and_grows_one_cell_a_frame() {
        let write = animate(WORDMARK, 1);
        assert!(write.frames[0].is_empty());
        for pair in write.frames.windows(2) {
            assert_eq!(pair[1].len(), pair[0].len() + 1);
        }
        assert_eq!(write.frames.len(), 104);
        assert_eq!(
            write.frames.last().unwrap().len(),
            lit(&raster(WORDMARK)),
            "the last frame is the whole wordmark"
        );
    }

    #[test]
    fn the_last_frame_is_the_padded_raster() {
        let write = animate(WORDMARK, 1);
        let grid = raster(WORDMARK);
        let want: Vec<usize> = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &v)| v == 1)
                    .map(move |(c, _)| (1 + r) * 49 + 1 + c)
            })
            .collect();
        assert_eq!(*write.frames.last().unwrap(), want);
    }

    #[test]
    fn merging_collapses_in_twenty_two_frames() {
        let merged = merge(WORDMARK, 1);
        assert_eq!(merged.len(), 22);
        assert_eq!(merged[0], animate(WORDMARK, 1).frames[103]);
        let x = crate::glyph('X').unwrap();
        let stacked: Vec<usize> = x
            .rows
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.chars()
                    .enumerate()
                    .filter(|&(_, ch)| ch == '1')
                    .map(move |(c, _)| (1 + r) * 49 + 22 + c)
            })
            .collect();
        assert_eq!(*merged.last().unwrap(), stacked, "the eight fold into X");
    }

    #[test]
    fn frames_are_sorted_and_in_bounds() {
        let write = animate(WORDMARK, 1);
        let anim = cycle(&write, &merge(WORDMARK, 1), HOLD);
        for frame in &anim.frames {
            assert!(frame.windows(2).all(|w| w[0] < w[1]));
            assert!(frame.iter().all(|&i| i < anim.rows * anim.cols));
        }
    }

    #[test]
    fn the_cycle_loops_through_both_halves() {
        let write = animate(WORDMARK, 1);
        let merged = merge(WORDMARK, 1);
        let anim = cycle(&write, &merged, HOLD);
        assert_eq!(
            anim.frames.len(),
            2 * write.frames.len() + 2 * merged.len() + 4 * HOLD
        );
        assert_eq!(anim.frames.len(), 352);
    }

    #[test]
    fn any_string_writes_itself() {
        for text in ["a", "hi", "mrly.net", "(1)"] {
            let write = animate(text, 2);
            let grid = raster(text);
            assert_eq!(write.rows, grid.len() + 4);
            assert_eq!(write.cols, grid[0].len() + 4);
            assert_eq!(write.frames.last().unwrap().len(), lit(&grid));
            assert_eq!(write.frames.len(), lit(&grid) + 1);
        }
    }

    #[test]
    fn a_lone_glyph_has_nothing_to_merge() {
        assert_eq!(merge("A", 1).len(), 1);
    }
}
