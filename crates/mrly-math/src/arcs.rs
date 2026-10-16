use crate::bang::Code;
use crate::bang::factory::{code_to_corners, create};
use mrly_core::error::{Result, overflow_error, value_error};
use serde::{Deserialize, Serialize};

// THE ARCS

/// One level of a flat design drawn in Truchet arcs: its side, its cells and the curves its arcs join into.
///
/// Cell `(x, y)` is column `x` and row `y`, row 0 at the bottom, stored at `y side + x`. Every cell carries two quarter arcs, each joining the midpoints of two adjacent edges: a filled cell takes the arcs around its lower-left and upper-right corners, a deleted cell the arcs around its lower-right and upper-left corners. The lower arc of a cell is the one ending on its bottom edge, the upper arc the one ending on its top edge. Arcs meeting at an edge midpoint join into curves, closed loops and open strands.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arcs {
    /// The cells per axis.
    pub side: usize,
    /// One byte a cell: bit 0 set when the cell is filled, bit 1 when its lower arc lies on a loop, bit 2 when its upper arc does.
    pub cells: Vec<u8>,
    /// The closed loops.
    pub loops: u64,
    /// The open strands, `2 side` at every level: each of the `4 side` boundary midpoints ends one.
    pub strands: u64,
}

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn find(&mut self, mut i: usize) -> usize {
        while self.parent[i] != i {
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        self.parent[ra] = rb;
    }
}

/// Draws a `side x side` grid of filled and deleted cells in arcs and counts its curves by union-find over the edge midpoints, a curve being a loop when no midpoint of it lies on the boundary.
///
/// # Errors
///
/// Errors on side zero or a cell list that is not `side^2` long.
pub fn trace(side: usize, on: &[bool]) -> Result<Arcs> {
    if side == 0 || on.len() != side * side {
        return value_error("arcs need side^2 cells and a side of at least one.");
    }
    let rows = side * (side + 1);
    let across = |x: usize, y: usize| y * side + x;
    let upright = |x: usize, y: usize| rows + y * (side + 1) + x;
    let pairs = |x: usize, y: usize| {
        let (bottom, top) = (across(x, y), across(x, y + 1));
        let (left, right) = (upright(x, y), upright(x + 1, y));
        if on[y * side + x] {
            [(bottom, left), (top, right)]
        } else {
            [(bottom, right), (top, left)]
        }
    };
    let mut dsu = Dsu {
        parent: (0..2 * rows).collect(),
    };
    let mut ends = vec![0u8; 2 * rows];
    for y in 0..side {
        for x in 0..side {
            for (a, b) in pairs(x, y) {
                ends[a] += 1;
                ends[b] += 1;
                dsu.union(a, b);
            }
        }
    }
    let mut open = vec![false; 2 * rows];
    let mut root = vec![false; 2 * rows];
    for (point, &count) in ends.iter().enumerate() {
        let r = dsu.find(point);
        root[r] = true;
        open[r] |= count == 1;
    }
    let strands = (0..2 * rows).filter(|&r| root[r] && open[r]).count() as u64;
    let loops = (0..2 * rows).filter(|&r| root[r] && !open[r]).count() as u64;
    let mut cells = vec![0u8; side * side];
    for y in 0..side {
        for x in 0..side {
            let [(lower, _), (upper, _)] = pairs(x, y);
            let mut byte = u8::from(on[y * side + x]);
            if !open[dsu.find(lower)] {
                byte |= 2;
            }
            if !open[dsu.find(upper)] {
                byte |= 4;
            }
            cells[y * side + x] = byte;
        }
    }
    Ok(Arcs {
        side,
        cells,
        loops,
        strands,
    })
}

/// Draws level `level` of `bang dim 2, base b, code c` in arcs: cell `(x, y)` of the `b x b` mask is filled when bit `b y + x` of the code is set, level `level` is its Kronecker power built by [`crate::bang::factory::create`], and level 0 is one filled cell.
///
/// # Errors
///
/// Errors on a code outside the digit space of the base, or a side past the machine word.
pub fn draw(code: Code, base: usize, level: usize) -> Result<Arcs> {
    code_to_corners(code, 2, base)?;
    let Some(side) = u32::try_from(level).ok().and_then(|n| base.checked_pow(n)) else {
        return overflow_error(format!("base {base} at level {level} is too wide to draw."));
    };
    if level == 0 {
        return trace(1, &[true]);
    }
    let tensor = create(code, base, 2, base, level)?;
    let on: Vec<bool> = (0..side * side).map(|f| tensor.at(f) == 1).collect();
    trace(side, &on)
}

// THE CURVES

/// One curve of an arcs drawing: its arcs in walking order, the way each is walked, and whether it closes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Curve {
    /// The arcs in walking order, arc `2 i` the lower and arc `2 i + 1` the upper arc of cell `i`.
    pub arcs: Vec<usize>,
    /// One sign an arc: 1 when walked from its end on a left or right edge to its end on a bottom or top edge, -1 the other way.
    pub dir: Vec<i8>,
    /// True for a loop, false for a strand.
    pub closed: bool,
}

fn corner(filled: bool, which: usize, x: usize, y: usize) -> (usize, usize) {
    match (which == 1, filled) {
        (true, true) => (x + 1, y + 1),
        (true, false) => (x, y + 1),
        (false, true) => (x, y),
        (false, false) => (x + 1, y),
    }
}

fn walk(
    ends: &[(usize, usize)],
    at: &[[usize; 2]],
    seen: &mut [bool],
    start: usize,
    from: usize,
) -> Curve {
    let (mut a, mut node) = (start, from);
    let mut curve = Curve {
        arcs: Vec::new(),
        dir: Vec::new(),
        closed: false,
    };
    loop {
        seen[a] = true;
        let (upright, across) = ends[a];
        let entered = upright == node;
        curve.arcs.push(a);
        curve.dir.push(if entered { 1 } else { -1 });
        node = if entered { across } else { upright };
        let next = if at[node][0] == a {
            at[node][1]
        } else {
            at[node][0]
        };
        if next == usize::MAX {
            return curve;
        }
        if next == start {
            curve.closed = true;
            return curve;
        }
        a = next;
    }
}

/// Walks level `level` of `bang dim 2, base b, code c` in arcs into its curves: first the strands, from each boundary midpoint in turn, across edges before upright ones, then the loops, each from its first unwalked arc.
///
/// The corner of an arc is its cell's lower-left or upper-right for a filled cell, lower-right or upper-left for a deleted one, as in [`draw`].
///
/// # Errors
///
/// Errors on a code outside the digit space of the base, or a side past the machine word.
pub fn curves(code: Code, base: usize, level: usize) -> Result<Vec<Curve>> {
    let arcs = draw(code, base, level)?;
    let n = arcs.side;
    let rows = n * (n + 1);
    let count = 2 * n * n;
    let mut ends = vec![(0, 0); count];
    let mut at = vec![[usize::MAX; 2]; 2 * rows];
    for i in 0..n * n {
        let (x, y) = (i % n, i / n);
        for which in 0..2 {
            let (cx, cy) = corner(arcs.cells[i] & 1 == 1, which, x, y);
            let a = 2 * i + which;
            let (upright, across) = (rows + y * (n + 1) + cx, cy * n + x);
            ends[a] = (upright, across);
            for node in [upright, across] {
                let slot = usize::from(at[node][0] != usize::MAX);
                at[node][slot] = a;
            }
        }
    }
    let edge = |node: usize| match node < rows {
        true => node < n || node >= n * n,
        false => [0, n].contains(&((node - rows) % (n + 1))),
    };
    let mut seen = vec![false; count];
    let mut out = Vec::new();
    for node in (0..2 * rows).filter(|&node| edge(node)) {
        let a = at[node][0];
        if a != usize::MAX && !seen[a] {
            out.push(walk(&ends, &at, &mut seen, a, node));
        }
    }
    for a in 0..count {
        if !seen[a] {
            out.push(walk(&ends, &at, &mut seen, a, ends[a].0));
        }
    }
    Ok(out)
}

// THE LAWS

/// A proved closed form of a design's loop count, read at one level.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Law {
    /// The closed form in the level `n`, one line of text.
    pub formula: String,
    /// The loops the law gives at the level asked.
    pub loops: u64,
}

/// Returns the proved loop law of `bang dim 2, base b, code c` at the level, or none where no law is proved: the carpet, base 3 code 495, has `(8^n - 1)/7 - 3^n + n + 1` loops; at base 2 codes 7 and 14 have `3^(n-1) - 2^n + 1`, codes 11 and 13 have `3^(n-1) - 2^(n-1)`, both from level 1 on, code 9 has `2^n - 1`, and the other eleven codes never loop.
///
/// # Errors
///
/// Errors on a code outside the digit space of the base, or a count past 64 bits.
pub fn law(code: Code, base: usize, level: usize) -> Result<Option<Law>> {
    code_to_corners(code, 2, base)?;
    let Ok(n) = u32::try_from(level) else {
        return overflow_error(format!("level {level} is past any loop law."));
    };
    let power = |b: i128, e: u32| b.checked_pow(e);
    let value = |terms: &[Option<i128>]| -> Option<i128> {
        terms
            .iter()
            .try_fold(0i128, |sum, term| sum.checked_add((*term)?))
    };
    let (formula, loops) = match (base, code.get()) {
        (3, 495) => (
            "(8^n - 1)/7 - 3^n + n + 1",
            power(8, n).map(|p| (p - 1) / 7).and_then(|head| {
                value(&[Some(head), power(3, n).map(|p| -p), Some(i128::from(n) + 1)])
            }),
        ),
        (2, 7 | 14) if n == 0 => ("3^(n-1) - 2^n + 1, n >= 1", Some(0)),
        (2, 7 | 14) => (
            "3^(n-1) - 2^n + 1, n >= 1",
            value(&[power(3, n - 1), power(2, n).map(|p| -p), Some(1)]),
        ),
        (2, 11 | 13) if n == 0 => ("3^(n-1) - 2^(n-1), n >= 1", Some(0)),
        (2, 11 | 13) => (
            "3^(n-1) - 2^(n-1), n >= 1",
            value(&[power(3, n - 1), power(2, n - 1).map(|p| -p)]),
        ),
        (2, 9) => ("2^n - 1", power(2, n).map(|p| p - 1)),
        (2, _) => ("0", Some(0)),
        _ => return Ok(None),
    };
    match loops.and_then(|count| u64::try_from(count).ok()) {
        Some(loops) => Ok(Some(Law {
            formula: formula.to_string(),
            loops,
        })),
        None => overflow_error(format!("the loop law passes 64 bits at level {level}.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loops(code: u128, base: usize, levels: std::ops::RangeInclusive<usize>) -> Vec<u64> {
        levels
            .map(|n| draw(Code::from(code), base, n).unwrap().loops)
            .collect()
    }

    #[test]
    fn the_carpet_and_code_seven_close_their_loops() {
        assert_eq!(loops(495, 3, 1..=4), [0, 3, 50, 509]);
        assert_eq!(loops(7, 2, 1..=4), [0, 0, 2, 12]);
    }

    #[test]
    fn every_proved_law_meets_the_count_and_strands_are_twice_the_side() {
        let mut cases: Vec<(u128, usize, usize)> = (0..16).map(|code| (code, 2, 7)).collect();
        cases.push((495, 3, 5));
        for (code, base, top) in cases {
            for n in 0..=top {
                let arcs = draw(Code::from(code), base, n).unwrap();
                let law = law(Code::from(code), base, n).unwrap().unwrap();
                assert_eq!(law.loops, arcs.loops, "code {code} base {base} level {n}");
                assert_eq!(arcs.strands, 2 * arcs.side as u64);
            }
        }
        assert_eq!(law(Code::from(511u128), 3, 2).unwrap(), None);
        assert!(law(Code::from(16u128), 2, 2).is_err());
    }

    #[test]
    fn the_walk_finds_the_loops_and_the_strands_the_count_finds_each_arc_once() {
        let mut cases: Vec<(u128, usize, usize)> = Vec::new();
        for code in 0..16 {
            cases.extend((1..=4).map(|level| (code, 2, level)));
        }
        for code in [495, 186, 455, 341, 170, 16] {
            cases.extend((1..=3).map(|level| (code, 3, level)));
        }
        for (code, base, level) in cases {
            let drawn = draw(Code::from(code), base, level).unwrap();
            let walked = curves(Code::from(code), base, level).unwrap();
            let loops = walked.iter().filter(|curve| curve.closed).count() as u64;
            let strands = walked.len() as u64 - loops;
            let mut order: Vec<usize> =
                walked.iter().flat_map(|curve| curve.arcs.clone()).collect();
            order.sort_unstable();
            assert_eq!(
                (loops, strands),
                (drawn.loops, drawn.strands),
                "{code} {base} {level}"
            );
            assert_eq!(order, (0..2 * drawn.side * drawn.side).collect::<Vec<_>>());
            for curve in &walked {
                assert_eq!(curve.arcs.len(), curve.dir.len());
                for &a in &curve.arcs {
                    let bit = (drawn.cells[a >> 1] >> (1 + (a & 1))) & 1;
                    assert_eq!(bit == 1, curve.closed, "{code} {base} {level} arc {a}");
                }
            }
        }
    }

    #[test]
    fn the_arcs_of_a_curve_chain_through_shared_midpoints_a_loop_closes_and_a_strand_ends_on_the_border()
     {
        let drawn = draw(Code::from(495u128), 3, 2).unwrap();
        let n = drawn.side;
        let ends = |a: usize, dir: i8| {
            let (i, which) = (a >> 1, a & 1);
            let (x, y) = (i % n, i / n);
            let (cx, cy) = corner(drawn.cells[i] & 1 == 1, which, x, y);
            let (upright, across) = ((2 * cx, 2 * y + 1), (2 * x + 1, 2 * cy));
            if dir > 0 {
                (upright, across)
            } else {
                (across, upright)
            }
        };
        let border = |(u, v): (usize, usize)| u == 0 || v == 0 || u == 2 * n || v == 2 * n;
        for curve in curves(Code::from(495u128), 3, 2).unwrap() {
            let steps: Vec<_> = curve
                .arcs
                .iter()
                .zip(&curve.dir)
                .map(|(&a, &d)| ends(a, d))
                .collect();
            for pair in steps.windows(2) {
                assert_eq!(pair[0].1, pair[1].0);
            }
            let (first, last) = (steps[0].0, steps[steps.len() - 1].1);
            match curve.closed {
                true => assert_eq!(last, first),
                false => assert_eq!((border(first), border(last)), (true, true)),
            }
        }
    }

    #[test]
    fn the_one_loop_of_the_diagonal_is_the_circle_round_the_centre() {
        let walked = curves(Code::from(9u128), 2, 1).unwrap();
        let loops: Vec<&Curve> = walked.iter().filter(|curve| curve.closed).collect();
        assert_eq!((loops.len(), walked.len() - loops.len()), (1, 4));
        let cells = draw(Code::from(9u128), 2, 1).unwrap().cells;
        let corners: Vec<(usize, usize)> = loops[0]
            .arcs
            .iter()
            .map(|&a| corner(cells[a >> 1] & 1 == 1, a & 1, (a >> 1) % 2, (a >> 1) / 2))
            .collect();
        assert_eq!(corners, [(1, 1); 4]);
        let mut held: Vec<usize> = loops[0].arcs.iter().map(|&a| a >> 1).collect();
        held.sort_unstable();
        assert_eq!(held, [0, 1, 2, 3]);
    }

    #[test]
    fn a_cell_byte_marks_the_filled_cell_and_its_arcs_on_loops() {
        let arcs = draw(Code::from(9u128), 2, 1).unwrap();
        assert_eq!((arcs.side, arcs.loops, arcs.strands), (2, 1, 4));
        assert_eq!(arcs.cells, [5, 4, 2, 3]);
        assert_eq!(draw(Code::from(1u128), 2, 0).unwrap().cells, [1]);
        assert!(trace(2, &[true; 3]).is_err());
    }
}
