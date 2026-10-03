use crate::core::error::{overflow_error, value_error, Result};
use crate::math::bang::factory::{code_to_corners, create};
use crate::math::bang::Code;
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

/// Draws level `level` of `bang dim 2, base b, code c` in arcs: cell `(x, y)` of the `b x b` mask is filled when bit `b y + x` of the code is set, level `level` is its Kronecker power built by [`crate::math::bang::factory::create`], and level 0 is one filled cell.
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
    fn a_cell_byte_marks_the_filled_cell_and_its_arcs_on_loops() {
        let arcs = draw(Code::from(9u128), 2, 1).unwrap();
        assert_eq!((arcs.side, arcs.loops, arcs.strands), (2, 1, 4));
        assert_eq!(arcs.cells, [5, 4, 2, 3]);
        assert_eq!(draw(Code::from(1u128), 2, 0).unwrap().cells, [1]);
        assert!(trace(2, &[true; 3]).is_err());
    }
}
