use crate::bang::universe::{apply, corner_index, corners, degree, orbit, symmetries, Code};
use crate::name::{Bang, Named};
use mrlycore::tensor::Tensor;
use std::collections::BTreeSet;

/// Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering.
pub fn output(rule: u8, l: u8, c: u8, r: u8) -> u8 {
    (rule >> (4 * l + 2 * c + r)) & 1
}

/// Advances one row one generation, a constant-0 boundary unless the edges wrap.
pub fn step(row: &[u8], rule: u8, wrap: bool) -> Vec<u8> {
    let width = row.len();
    (0..width)
        .map(|i| {
            let l = if i == 0 {
                if wrap {
                    row[width - 1]
                } else {
                    0
                }
            } else {
                row[i - 1]
            };
            let r = if i + 1 == width {
                if wrap {
                    row[0]
                } else {
                    0
                }
            } else {
                row[i + 1]
            };
            output(rule, l, row[i], r)
        })
        .collect()
}

/// Returns the space-time diagram of a seed row: row 0 the seed, then one row per generation.
pub fn history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Tensor {
    let width = row.len();
    let mut cells = Vec::with_capacity((steps + 1) * width);
    cells.extend_from_slice(row);
    let mut current = row.to_vec();
    for _ in 0..steps {
        current = step(&current, rule, wrap);
        cells.extend_from_slice(&current);
    }
    Tensor::of(cells, vec![steps + 1, width])
}

/// Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.
pub fn single_seed(rule: u8, steps: usize) -> Tensor {
    let window = 2 * steps + 1;
    let width = window + 2 * steps;
    let mut row = vec![0u8; width];
    row[width / 2] = 1;
    let full = history(&row, rule, steps, false);
    let mut cells = Vec::with_capacity((steps + 1) * window);
    for t in 0..=steps {
        let start = t * width + steps;
        cells.extend_from_slice(&full.bytes()[start..start + window]);
    }
    Tensor::of(cells, vec![steps + 1, window])
}

/// Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.
pub fn corner_bits(rule: u8) -> Vec<u8> {
    (0..8).map(|i| (rule >> i) & 1).collect()
}

/// Returns the count of neighbourhoods a rule sends to one.
pub fn popcount(rule: u8) -> u32 {
    rule.count_ones()
}

/// Returns Langton's lambda, the popcount over eight.
pub fn lambda(rule: u8) -> f64 {
    rule.count_ones() as f64 / 8.0
}

/// Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.
pub fn rule_degree(rule: u8) -> i32 {
    degree(rule as Code, 3)
}

/// Returns whether a rule is affine, its algebraic degree at most one.
pub fn affine(rule: u8) -> bool {
    rule_degree(rule) <= 1
}

/// Returns the design name a rule carries, `mrly_bang_d3_<rule>`.
pub fn rule_name(rule: u8) -> String {
    Named::to_str(&Bang::new(rule as Code, 3, 2))
}

fn act(rule: u8, element: &(Vec<usize>, Vec<u8>), complement: bool) -> u8 {
    let cells = corners(3);
    let mut image = 0u8;
    for (i, cell) in cells.iter().enumerate() {
        if (rule >> i) & 1 == 1 {
            image |= 1 << corner_index(&apply(element, cell));
        }
    }
    if complement {
        image ^ 0xff
    } else {
        image
    }
}

/// Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.
pub fn cube_orbit(rule: u8) -> Vec<u8> {
    orbit(rule as Code, 3)
        .into_iter()
        .map(|c| c as u8)
        .collect()
}

/// Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.
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

/// Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.
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

/// Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.
pub fn genus(rule: u8) -> &'static str {
    let class = cube_orbit(rule);
    if class.iter().any(|&c| level_set(c)) {
        "iso"
    } else if class.iter().any(|&c| pinned(c)) {
        "axis"
    } else {
        "comp"
    }
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

/// Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.
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

/// Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.
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

/// Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.
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

/// Returns the base-2 plane design a rule's single seed draws, or None when it draws none.
pub fn gasket(rule: u8) -> Option<&'static str> {
    match rule {
        60 | 90 => Some("mrly_bang_d2_13"),
        102 => Some("mrly_bang_d2_14"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{next_grid, Boundary};
    use crate::two::Cell2d;
    fn rules() -> impl Iterator<Item = u8> {
        0..=255u8
    }
    #[test]
    fn thirty_rules_are_surjective() {
        assert_eq!(rules().filter(|&r| surjective(r)).count(), 30);
    }
    #[test]
    fn the_reversible_rules_are_the_six_single_axis_ones() {
        let six: Vec<u8> = rules().filter(|&r| reversible(r)).collect();
        assert_eq!(six, vec![15, 51, 85, 170, 204, 240]);
        assert!(six.iter().all(|&r| surjective(r)));
    }
    #[test]
    fn the_class_counts_are_eighty_eight_twenty_two_and_fourteen() {
        let count = |f: fn(u8) -> Vec<u8>| rules().map(|r| f(r)[0]).collect::<BTreeSet<u8>>().len();
        assert_eq!(count(wolfram_class), 88);
        assert_eq!(count(cube_orbit), 22);
        assert_eq!(count(npn_class), 14);
    }
    #[test]
    fn rule_110_holds_a_cube_orbit_of_twenty_four() {
        let cube = cube_orbit(110);
        assert_eq!(cube.len(), 24);
        assert!(cube.contains(&94));
        assert!(!cube.contains(&137));
        assert_eq!(wolfram_class(110), vec![110, 124, 137, 193]);
    }
    #[test]
    fn rule_60_draws_the_level_four_gasket() {
        let diagram = single_seed(60, 16);
        let tile = crate::two::create(13, 2, 4, 0, 2).unwrap();
        let centre = diagram.shape[1] / 2;
        for t in 0..16 {
            for j in 0..16 {
                assert_eq!(
                    diagram.get(&[t, centre + j]),
                    tile.types().get(&[t, j]),
                    "t={t} j={j}"
                );
            }
        }
        assert_eq!(gasket(60), Some("mrly_bang_d2_13"));
    }
    #[test]
    fn rule_150_row_populations_are_a071053() {
        let diagram = single_seed(150, 11);
        let width = diagram.shape[1];
        let counts: Vec<u32> = (0..12)
            .map(|t| (0..width).map(|i| diagram.get(&[t, i]) as u32).sum())
            .collect();
        assert_eq!(counts, vec![1, 3, 3, 5, 3, 9, 5, 11, 3, 9, 9, 15]);
    }
    #[test]
    fn sixty_four_rules_read_their_outer_cells_by_count() {
        assert_eq!(
            rules().filter(|&r| outer_totalistic(r).is_some()).count(),
            64
        );
        assert!(outer_totalistic(110).is_none());
        assert_eq!(outer_totalistic(94), Some((vec![1], vec![0, 1])));
    }
    #[test]
    fn the_genus_and_affine_counts_hold() {
        assert_eq!(rules().filter(|&r| affine(r)).count(), 16);
        let genus_of = |name| rules().filter(|&r| genus(r) == name).count();
        assert_eq!(
            (genus_of("iso"), genus_of("axis"), genus_of("comp")),
            (52, 18, 186)
        );
    }
    #[test]
    fn a_ring_steps_around_itself() {
        let row = [1, 0, 0, 0, 0];
        assert_eq!(step(&row, 170, true), vec![0, 0, 0, 0, 1]);
        assert_eq!(step(&row, 170, false), vec![0, 0, 0, 0, 0]);
    }
    #[test]
    fn a_line_steps_through_the_grid_stepper() {
        let row = vec![0, 1, 1, 0, 1, 0, 0];
        let mask = Tensor::of(vec![1, 0, 1], vec![1, 3]);
        let cell = Cell2d::new(Tensor::of(row.clone(), vec![1, 7]));
        let next = next_grid(&cell, &[1], &[0, 1], &mask, Boundary::Constant).unwrap();
        assert_eq!(next.types().bytes(), step(&row, 94, false));
    }
    #[test]
    fn the_card_pieces_read_rule_110() {
        assert_eq!(rule_name(110), "mrly_bang_d3_110");
        assert_eq!(corner_bits(110), vec![0, 1, 1, 1, 0, 1, 1, 0]);
        assert_eq!(
            (popcount(110), rule_degree(110), genus(110)),
            (5, 3, "comp")
        );
        assert!((lambda(110) - 0.625).abs() < 1e-12);
        assert!(!surjective(110) && !reversible(110) && !affine(110));
    }
}
