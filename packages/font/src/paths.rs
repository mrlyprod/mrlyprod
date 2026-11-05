use super::pens::{self, Pen};
use std::collections::BTreeSet;

const STEPS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn penned(c: char) -> Option<Vec<Vec<(usize, usize)>>> {
    pens::table()
        .into_iter()
        .find(|&(key, _)| key == c)
        .map(|pen| parse(&pen))
}

pub fn strokes(c: char) -> Vec<Vec<(usize, usize)>> {
    penned(c).unwrap_or_default()
}

pub fn path(c: char) -> Vec<(usize, usize)> {
    strokes(c).into_iter().flatten().collect()
}

fn parse((_, strokes): &Pen) -> Vec<Vec<(usize, usize)>> {
    strokes
        .iter()
        .map(|stroke| stroke.split_whitespace().map(cell).collect())
        .collect()
}

fn cell(token: &str) -> (usize, usize) {
    let digits: Vec<usize> = token
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();
    (digits[0], digits[1])
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

fn opening(left: &BTreeSet<(usize, usize)>) -> Option<(usize, usize)> {
    left.iter()
        .min_by_key(|&&cell| (degree(cell, left) != 1, usize::MAX - cell.0, cell.1))
        .copied()
}

pub fn draft(rows: &[String]) -> Vec<Vec<(usize, usize)>> {
    let mut left = lit_of(rows);
    let mut out = Vec::new();
    while let Some(start) = opening(&left) {
        left.remove(&start);
        let mut stroke = vec![start];
        let mut cur = start;
        let mut heading: Option<(i64, i64)> = None;
        loop {
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
            cur = next;
        }
        out.push(stroke);
    }
    out
}

pub fn floor(rows: &[String]) -> usize {
    let cells: Vec<(usize, usize)> = lit_of(rows).into_iter().collect();
    let n = cells.len();
    if n == 0 {
        return 0;
    }
    let index = |cell: (usize, usize)| cells.binary_search(&cell).ok();
    let mut edges = Vec::new();
    for (i, &(r, c)) in cells.iter().enumerate() {
        if let Some(j) = index((r, c + 1)) {
            edges.push((i, j));
        }
        if let Some(j) = index((r + 1, c)) {
            edges.push((i, j));
        }
    }
    let mut deg = vec![0u8; n];
    let mut parent: Vec<usize> = (0..n).collect();
    let mut best = n - draft(rows).len();
    forest(&edges, 0, 0, &mut deg, &mut parent, &mut best);
    n - best
}

fn root(parent: &[usize], mut v: usize) -> usize {
    while parent[v] != v {
        v = parent[v];
    }
    v
}

fn forest(
    edges: &[(usize, usize)],
    at: usize,
    taken: usize,
    deg: &mut [u8],
    parent: &mut [usize],
    best: &mut usize,
) {
    *best = (*best).max(taken);
    if at == edges.len() || taken + edges.len() - at <= *best {
        return;
    }
    let (a, b) = edges[at];
    if deg[a] < 2 && deg[b] < 2 {
        let (ra, rb) = (root(parent, a), root(parent, b));
        if ra != rb {
            deg[a] += 1;
            deg[b] += 1;
            parent[ra] = rb;
            forest(edges, at + 1, taken + 1, deg, parent, best);
            parent[ra] = ra;
            deg[a] -= 1;
            deg[b] -= 1;
        }
    }
    forest(edges, at + 1, taken, deg, parent, best);
}
