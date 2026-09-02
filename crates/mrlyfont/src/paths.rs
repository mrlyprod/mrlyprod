use super::{glyph, trim};
use std::collections::BTreeSet;

type Pen = (char, &'static [&'static [(usize, usize)]]);

/// The hand-penned stroke orders of the seven wordmark letters.
#[rustfmt::skip]
pub const PATHS: &[Pen] = &[
    ('M', &[
        &[(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)],
        &[(0, 1), (0, 2), (0, 3), (0, 4)],
        &[(1, 4), (2, 4), (3, 4), (4, 4)],
        &[(1, 2), (2, 2), (3, 2), (4, 2)],
    ]),
    ('R', &[
        &[(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)],
        &[(0, 1), (0, 2), (0, 3), (0, 4)],
    ]),
    ('L', &[
        &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
        &[(4, 1), (4, 2), (4, 3), (4, 4)],
    ]),
    ('Y', &[
        &[(0, 0), (1, 0), (2, 0)],
        &[(2, 1), (2, 2), (2, 3)],
        &[(0, 4), (1, 4), (2, 4)],
        &[(3, 4), (4, 4), (4, 3), (4, 2), (4, 1), (4, 0)],
    ]),
    ('P', &[
        &[(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)],
        &[(0, 1), (0, 2), (0, 3), (0, 4)],
        &[(1, 4)],
        &[(2, 4), (2, 3), (2, 2), (2, 1)],
    ]),
    ('O', &[
        &[(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)],
        &[(0, 1), (0, 2), (0, 3), (0, 4)],
        &[(1, 4), (2, 4), (3, 4), (4, 4)],
        &[(4, 3), (4, 2), (4, 1)],
    ]),
    ('D', &[
        &[(2, 3), (2, 2), (2, 1), (2, 0)],
        &[(3, 0), (4, 0)],
        &[(4, 1), (4, 2), (4, 3), (4, 4)],
        &[(3, 4), (2, 4), (1, 4), (0, 4)],
    ]),
];

const STEPS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Returns the hand-penned strokes of a wordmark letter, or None for a character drawn by derivation.
pub fn penned(c: char) -> Option<Vec<Vec<(usize, usize)>>> {
    PATHS
        .iter()
        .find(|&&(key, _)| key == c)
        .map(|&(_, strokes)| strokes.iter().map(|s| s.to_vec()).collect())
}

/// Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font.
pub fn strokes(c: char) -> Vec<Vec<(usize, usize)>> {
    match glyph(c) {
        Some(g) => sequence(c, &trim(&g.rows)),
        None => Vec::new(),
    }
}

/// Flattens the character's strokes into one cell-by-cell drawing order.
pub fn path(c: char) -> Vec<(usize, usize)> {
    strokes(c).into_iter().flatten().collect()
}

pub(crate) fn sequence(c: char, rows: &[String]) -> Vec<Vec<(usize, usize)>> {
    penned(c).unwrap_or_else(|| derive(rows))
}

fn lit_of(rows: &[String]) -> BTreeSet<(usize, usize)> {
    let mut lit = BTreeSet::new();
    for (r, row) in rows.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '1' {
                lit.insert((r, c));
            }
        }
    }
    lit
}

fn step(cell: (usize, usize), (dr, dc): (i64, i64)) -> Option<(usize, usize)> {
    let (r, c) = (cell.0 as i64 + dr, cell.1 as i64 + dc);
    (r >= 0 && c >= 0).then_some((r as usize, c as usize))
}

fn degree(cell: (usize, usize), left: &BTreeSet<(usize, usize)>) -> usize {
    STEPS
        .iter()
        .filter(|&&d| step(cell, d).is_some_and(|n| left.contains(&n)))
        .count()
}

fn opening(left: &BTreeSet<(usize, usize)>) -> (usize, usize) {
    *left
        .iter()
        .min_by_key(|&&cell| (degree(cell, left) != 1, usize::MAX - cell.0, cell.1))
        .expect("opening is only asked of a non-empty set")
}

fn derive(rows: &[String]) -> Vec<Vec<(usize, usize)>> {
    let mut left = lit_of(rows);
    let mut out = Vec::new();
    while !left.is_empty() {
        let start = opening(&left);
        left.remove(&start);
        let mut stroke = vec![start];
        let mut heading: Option<(i64, i64)> = None;
        loop {
            let cur = *stroke.last().unwrap();
            let ahead = heading
                .and_then(|d| step(cur, d))
                .filter(|n| left.contains(n));
            let next = ahead.or_else(|| {
                STEPS
                    .iter()
                    .find_map(|&d| step(cur, d).filter(|n| left.contains(n)))
            });
            let Some(next) = next else { break };
            heading = Some((next.0 as i64 - cur.0 as i64, next.1 as i64 - cur.1 as i64));
            left.remove(&next);
            stroke.push(next);
        }
        out.push(stroke);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::supported;

    #[test]
    fn every_path_is_a_permutation_of_the_glyph() {
        for c in supported() {
            let rows = trim(&glyph(c).unwrap().rows);
            let walk = path(c);
            let seen: BTreeSet<(usize, usize)> = walk.iter().copied().collect();
            assert_eq!(seen.len(), walk.len(), "{c} repeats a cell");
            assert_eq!(seen, lit_of(&rows), "{c} misses or invents a cell");
        }
    }

    #[test]
    fn the_wordmark_letters_are_penned_never_derived() {
        for c in "MRLYPROD".chars() {
            assert!(penned(c).is_some(), "{c} lost its pen order");
        }
        assert_eq!(penned('M').unwrap()[0][0], (4, 0));
        assert_eq!(penned('D').unwrap().len(), 4);
    }

    #[test]
    fn every_penned_char_is_supported() {
        for &(c, _) in PATHS {
            assert!(glyph(c).is_some(), "{c} penned but unsupported");
        }
    }

    #[test]
    fn strokes_are_four_adjacent_walks() {
        for c in supported() {
            for stroke in strokes(c) {
                for pair in stroke.windows(2) {
                    let (a, b) = (pair[0], pair[1]);
                    let gap = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
                    assert_eq!(gap, 1, "{c} jumps inside a stroke");
                }
            }
        }
    }
}
