use mrlycore::errors::{value_error, Result};
use mrlynum::classics::{factorial, gcd};
use mrlynum::factor::{divisors, mobius};
use std::collections::BTreeMap;

type Cycles = BTreeMap<usize, u128>;

fn pos_block_cycles(length: usize) -> Cycles {
    let mut out = Cycles::new();
    for period in divisors(length) {
        let strings: i128 = divisors(period)
            .iter()
            .map(|&d| i128::from(mobius(period / d)) * (1i128 << d))
            .sum();
        *out.entry(period).or_insert(0) += (strings / period as i128) as u128;
    }
    out
}

fn neg_block_cycles(length: usize) -> Cycles {
    let states = 1usize << length;
    let step = |x: usize| -> usize {
        let mut new = vec![0usize; length];
        for (i, item) in new.iter_mut().enumerate() {
            *item = (x >> ((i + length - 1) % length)) & 1;
        }
        new[0] ^= 1;
        new.iter().enumerate().map(|(i, &b)| b << i).sum()
    };
    let mut seen = vec![false; states];
    let mut out = Cycles::new();
    for start in 0..states {
        if seen[start] {
            continue;
        }
        let mut run = 0;
        let mut j = start;
        while !seen[j] {
            seen[j] = true;
            j = step(j);
            run += 1;
        }
        *out.entry(run).or_insert(0) += 1;
    }
    out
}

fn combine(c1: &Cycles, c2: &Cycles) -> Cycles {
    let mut out = Cycles::new();
    for (&l1, &n1) in c1 {
        for (&l2, &n2) in c2 {
            let g = gcd(l1 as u128, l2 as u128) as usize;
            *out.entry(l1 * l2 / g).or_insert(0) += n1 * n2 * g as u128;
        }
    }
    out
}

fn class_cycles(pos: &[usize], neg: &[usize]) -> u32 {
    let blocks: Vec<Cycles> = pos
        .iter()
        .map(|&l| pos_block_cycles(l))
        .chain(neg.iter().map(|&l| neg_block_cycles(l)))
        .collect();
    if blocks.is_empty() {
        return 1;
    }
    let mut acc = blocks[0].clone();
    for b in &blocks[1..] {
        acc = combine(&acc, b);
    }
    acc.values().sum::<u128>() as u32
}

fn partitions(n: usize, m: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for k in (1..=n.min(m)).rev() {
        for rest in partitions(n - k, k) {
            let mut item = vec![k];
            item.extend(rest);
            out.push(item);
        }
    }
    out
}

fn bipartitions(dimension: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut out = Vec::new();
    for s in 0..=dimension {
        for pos in partitions(s, s.max(1)) {
            for neg in partitions(dimension - s, (dimension - s).max(1)) {
                out.push((pos.clone(), neg));
            }
        }
    }
    out
}

fn class_size(pos: &[usize], neg: &[usize], dimension: usize) -> u128 {
    let mut centralizer: u128 = 1;
    for part in [pos, neg] {
        let mut mults = BTreeMap::new();
        for &length in part {
            *mults.entry(length).or_insert(0usize) += 1;
        }
        for (length, mult) in mults {
            centralizer *= factorial(mult) * (2 * length as u128).pow(mult as u32);
        }
    }
    ((1u128 << dimension) * factorial(dimension)) / centralizer
}

/// Returns the raw design count of a dimension, two to the number of corners.
pub fn total_designs(dimension: usize) -> u128 {
    assert!(dimension <= 6, "total exceeds u128 above dimension 6");
    1 << (1 << dimension)
}

/// Counts designs distinct under the full symmetry group class by class, or an error when the sums break.
pub fn distinct_designs(dimension: usize) -> Result<u128> {
    let order = (1u128 << dimension) * factorial(dimension);
    let mut total: u128 = 0;
    let mut checked: u128 = 0;
    for (pos, neg) in bipartitions(dimension) {
        let size = class_size(&pos, &neg, dimension);
        checked += size;
        total += size * (1u128 << class_cycles(&pos, &neg));
    }
    if checked != order {
        return value_error("class sizes do not sum to the group order.");
    }
    if !total.is_multiple_of(order) {
        return value_error("Burnside average is not an integer.");
    }
    Ok(total / order)
}

/// Returns the distinct-design counts for dimensions 1 through max_dimension.
///
/// ```
/// assert_eq!(mrlymath::bang::counting::sequence(4).unwrap(), vec![3, 6, 22, 402]);
/// ```
pub fn sequence(max_dimension: usize) -> Result<Vec<u128>> {
    (1..=max_dimension).map(distinct_designs).collect()
}

/// Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.
pub fn classes(dimension: usize) -> u128 {
    let mut row = vec![1u128];
    for _ in 0..dimension {
        let mut next = vec![1u128; row.len() + 1];
        for k in 1..row.len() {
            next[k] = row[k - 1] + row[k];
        }
        row = next;
    }
    row.iter().map(|corners| corners + 1).product()
}

/// Returns the fill-class counts for dimensions 1 through max_dimension.
///
/// ```
/// assert_eq!(mrlymath::bang::counting::class_sequence(4), vec![4, 12, 64, 700]);
/// ```
pub fn class_sequence(max_dimension: usize) -> Vec<u128> {
    (1..=max_dimension).map(classes).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn burnside_matches_enumeration() {
        use super::super::universe::bang;
        for d in 1..=3 {
            assert_eq!(distinct_designs(d).unwrap(), bang(d).distinct() as u128);
        }
    }
    #[test]
    fn sequence_is_a000616() {
        assert_eq!(
            sequence(6).unwrap(),
            vec![3, 6, 22, 402, 1228158, 400507806843728]
        );
    }
    #[test]
    fn classes_are_a129824() {
        assert_eq!(classes(0), 2);
        assert_eq!(
            class_sequence(7),
            vec![4, 12, 64, 700, 17424, 1053696, 160579584]
        );
    }
    #[test]
    fn totals_doubly_exponential() {
        let totals: Vec<u128> = (1..=4).map(total_designs).collect();
        assert_eq!(totals, vec![4, 16, 256, 65536]);
    }
}
