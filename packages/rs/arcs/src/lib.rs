use mrly_error::{Result, overflow_error, value_error};
use mrly_math::code::Code;
use mrly_math::factory::{code_to_corners, create};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arcs {
    pub side: usize,
    pub cells: Vec<u8>,
    pub loops: u64,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Curve {
    pub arcs: Vec<usize>,
    pub dir: Vec<i8>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Law {
    pub formula: String,
    pub loops: u64,
}

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
