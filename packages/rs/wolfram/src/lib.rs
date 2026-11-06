use mrly_bang::universe::{apply, corner_index, corners, degree, orbit, symmetries};
use mrly_error::{Result, value_error};
use mrly_math::code::Code;
use mrly_math::name::Bang;
use mrly_name::Named;
use mrly_tensor::Tensor;
use std::collections::BTreeSet;

pub fn output(rule: u8, l: u8, c: u8, r: u8) -> u8 {
    (rule >> (4 * (l & 1) + 2 * (c & 1) + (r & 1))) & 1
}

fn bits(row: &[u8]) -> Result<()> {
    match row.iter().find(|&&cell| cell > 1) {
        Some(cell) => value_error(format!("a row holds bits, not {cell}.")),
        None => Ok(()),
    }
}

pub fn step(row: &[u8], rule: u8, wrap: bool) -> Result<Vec<u8>> {
    bits(row)?;
    Ok(advance(row, rule, wrap))
}

fn advance(row: &[u8], rule: u8, wrap: bool) -> Vec<u8> {
    let width = row.len();
    (0..width)
        .map(|i| {
            let l = if i == 0 {
                if wrap { row[width - 1] } else { 0 }
            } else {
                row[i - 1]
            };
            let r = if i + 1 == width {
                if wrap { row[0] } else { 0 }
            } else {
                row[i + 1]
            };
            output(rule, l, row[i], r)
        })
        .collect()
}

pub fn history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Result<Tensor> {
    bits(row)?;
    let width = row.len();
    let mut cells = Vec::with_capacity((steps + 1) * width);
    cells.extend_from_slice(row);
    let mut current = row.to_vec();
    for _ in 0..steps {
        current = advance(&current, rule, wrap);
        cells.extend_from_slice(&current);
    }
    Tensor::of(cells, vec![steps + 1, width])
}

pub fn single_seed(rule: u8, steps: usize) -> Result<Tensor> {
    let window = 2 * steps + 1;
    let width = window + 2 * steps;
    let mut row = vec![0u8; width];
    row[width / 2] = 1;
    let mut cells = Vec::with_capacity((steps + 1) * window);
    for _ in 0..=steps {
        cells.extend_from_slice(&row[steps..steps + window]);
        row = advance(&row, rule, false);
    }
    Tensor::of(cells, vec![steps + 1, window])
}

pub fn corner_bits(rule: u8) -> Vec<u8> {
    (0..8).map(|i| (rule >> i) & 1).collect()
}

pub fn popcount(rule: u8) -> u32 {
    rule.count_ones()
}

pub fn lambda(rule: u8) -> f64 {
    rule.count_ones() as f64 / 8.0
}

pub fn rule_degree(rule: u8) -> i32 {
    degree(Code::from(rule as u64), 3)
}

pub fn affine(rule: u8) -> bool {
    rule_degree(rule) <= 1
}

pub fn rule_name(rule: u8) -> Result<String> {
    Bang::new(rule as u128, 3, 2).to_prose()
}

fn act(rule: u8, element: &(Vec<usize>, Vec<u8>), complement: bool) -> u8 {
    let cells = corners(3);
    let mut image = 0u8;
    for (i, cell) in cells.iter().enumerate() {
        if (rule >> i) & 1 == 1 {
            image |= 1 << corner_index(&apply(element, cell));
        }
    }
    if complement { image ^ 0xff } else { image }
}

pub fn cube_orbit(rule: u8) -> Vec<u8> {
    orbit(Code::from(rule as u64), 3)
        .into_iter()
        .map(|c| c.get() as u8)
        .collect()
}

pub fn wolfram_class(rule: u8) -> Vec<u8> {
    let plain = (vec![0, 1, 2], vec![0, 0, 0]);
    let mirror = (vec![2, 1, 0], vec![0, 0, 0]);
    let mut out = BTreeSet::new();
    out.insert(act(rule, &plain, false));
    out.insert(act(rule, &mirror, false));
    out.insert(act(rule, &(plain.0.clone(), vec![1, 1, 1]), true));
    out.insert(act(rule, &(mirror.0.clone(), vec![1, 1, 1]), true));
    out.into_iter().collect()
}

pub fn npn_class(rule: u8) -> Vec<u8> {
    let mut out = BTreeSet::new();
    for element in symmetries(3) {
        out.insert(act(rule, &element, false));
        out.insert(act(rule, &element, true));
    }
    out.into_iter().collect()
}

fn level_set(rule: u8) -> bool {
    let mut value = [-1i8; 4];
    for i in 0..8usize {
        let bit = ((rule >> i) & 1) as i8;
        let weight = i.count_ones() as usize;
        if value[weight] != -1 && value[weight] != bit {
            return false;
        }
        value[weight] = bit;
    }
    true
}

fn pinned(rule: u8) -> bool {
    let cells: Vec<usize> = (0..8).filter(|&i| (rule >> i) & 1 == 1).collect();
    if cells.is_empty() {
        return false;
    }
    let fixed = (0..3)
        .filter(|axis| {
            cells
                .iter()
                .map(|c| (c >> axis) & 1)
                .collect::<BTreeSet<usize>>()
                .len()
                == 1
        })
        .count();
    cells.len() == 1 << (3 - fixed)
}

pub fn genus(rule: u8) -> String {
    let class = cube_orbit(rule);
    let word = if class.iter().any(|&c| level_set(c)) {
        "iso"
    } else if class.iter().any(|&c| pinned(c)) {
        "axis"
    } else {
        "comp"
    };
    word.to_string()
}

fn arrows(rule: u8) -> Vec<(usize, usize, u8)> {
    let mut out = Vec::new();
    for a in 0..2u8 {
        for b in 0..2u8 {
            for c in 0..2u8 {
                out.push((
                    2 * a as usize + b as usize,
                    2 * b as usize + c as usize,
                    output(rule, a, b, c),
                ));
            }
        }
    }
    out
}

pub fn surjective(rule: u8) -> bool {
    let edges = arrows(rule);
    let start = 0b1111usize;
    let mut seen = BTreeSet::from([start]);
    let mut stack = vec![start];
    while let Some(set) = stack.pop() {
        for label in 0..2u8 {
            let mut next = 0usize;
            for &(u, v, l) in &edges {
                if l == label && (set >> u) & 1 == 1 {
                    next |= 1 << v;
                }
            }
            if next == 0 {
                return false;
            }
            if seen.insert(next) {
                stack.push(next);
            }
        }
    }
    true
}

pub fn reversible(rule: u8) -> bool {
    let edges = arrows(rule);
    let mut adjacency = vec![BTreeSet::new(); 16];
    for &(u1, v1, l1) in &edges {
        for &(u2, v2, l2) in &edges {
            if l1 == l2 {
                adjacency[4 * u1 + u2].insert(4 * v1 + v2);
            }
        }
    }
    let mut core: BTreeSet<usize> = (0..16).collect();
    loop {
        let outs: BTreeSet<usize> = core
            .iter()
            .filter(|p| adjacency[**p].iter().any(|q| core.contains(q)))
            .copied()
            .collect();
        let ins: BTreeSet<usize> = outs
            .iter()
            .flat_map(|p| adjacency[*p].iter().filter(|q| outs.contains(q)).copied())
            .collect();
        let next: BTreeSet<usize> = outs.intersection(&ins).copied().collect();
        if next == core {
            break;
        }
        core = next;
    }
    !core.iter().any(|p| p / 4 != p % 4)
}

pub fn outer_totalistic(rule: u8) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut table = [[-1i8; 3]; 2];
    for l in 0..2u8 {
        for c in 0..2u8 {
            for r in 0..2u8 {
                let bit = output(rule, l, c, r) as i8;
                let slot = &mut table[c as usize][(l + r) as usize];
                if *slot != -1 && *slot != bit {
                    return None;
                }
                *slot = bit;
            }
        }
    }
    let birth = (0..3).filter(|&n| table[0][n] == 1).collect();
    let survive = (0..3).filter(|&n| table[1][n] == 1).collect();
    Some((birth, survive))
}

pub fn gasket(rule: u8) -> Option<String> {
    match rule {
        60 | 90 => Some("bang dim 2, code 13".to_string()),
        102 => Some("bang dim 2, code 14".to_string()),
        _ => None,
    }
}
