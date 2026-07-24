use super::factory::residue_corners;
use super::universe::permutations;
use mrlycore::errors::{value_error, Result};
use std::collections::HashMap;

pub fn axis_maps(base: usize) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = Vec::new();
    for b in 0..base {
        let rot: Vec<usize> = (0..base).map(|r| (r + b) % base).collect();
        let ref_: Vec<usize> = (0..base).map(|r| (base + b - r) % base).collect();
        if !out.contains(&rot) {
            out.push(rot);
        }
        if !out.contains(&ref_) {
            out.push(ref_);
        }
    }
    out
}

fn factorial(n: usize) -> u128 {
    (1..=n as u128).product()
}

pub fn group_order(base: usize, dimension: usize) -> u128 {
    (axis_maps(base).len() as u128).pow(dimension as u32) * factorial(dimension)
}

pub fn predicted_group_order(base: usize, dimension: usize) -> u128 {
    let per_axis = if base == 2 { 2u128 } else { 2 * base as u128 };
    per_axis.pow(dimension as u32) * factorial(dimension)
}

fn choices(axis: &[Vec<usize>], dimension: usize) -> Vec<Vec<usize>> {
    let mut out = vec![vec![]];
    for _ in 0..dimension {
        let mut next = Vec::new();
        for prefix in &out {
            for (i, _) in axis.iter().enumerate() {
                let mut item = prefix.clone();
                item.push(i);
                next.push(item);
            }
        }
        out = next;
    }
    out
}

fn cycles(
    perm: &[usize],
    choice: &[usize],
    axis: &[Vec<usize>],
    cells: &[Vec<u8>],
    index: &HashMap<Vec<u8>, usize>,
) -> u32 {
    let apply = |corner: &[u8]| -> Vec<u8> {
        (0..corner.len())
            .map(|i| axis[choice[i]][corner[perm[i]] as usize] as u8)
            .collect()
    };
    let mut seen = vec![false; cells.len()];
    let mut count = 0;
    for start in 0..cells.len() {
        if seen[start] {
            continue;
        }
        count += 1;
        let mut j = start;
        while !seen[j] {
            seen[j] = true;
            j = index[&apply(&cells[j])];
        }
    }
    count
}

pub fn distinct_designs(base: usize, dimension: usize) -> Result<u128> {
    if base < 1 {
        return value_error("base must be at least 1.");
    }
    if dimension < 1 {
        return value_error("dimension must be at least 1.");
    }
    let cells = residue_corners(dimension, base);
    let index: HashMap<Vec<u8>, usize> = cells
        .iter()
        .enumerate()
        .map(|(i, c)| (c.clone(), i))
        .collect();
    let axis = axis_maps(base);
    let mut order: u128 = 0;
    let mut total: u128 = 0;
    for perm in permutations(dimension) {
        for choice in choices(&axis, dimension) {
            order += 1;
            total += 1u128 << cycles(&perm, &choice, &axis, &cells, &index);
        }
    }
    if !total.is_multiple_of(order) {
        return value_error("Burnside average is not an integer.");
    }
    Ok(total / order)
}

pub fn total_designs(base: usize, dimension: usize) -> u128 {
    let cells = base.pow(dimension as u32);
    assert!(cells < 128, "too many cells for a u128 count");
    1 << cells
}

pub fn sequence(base: usize, max_dimension: usize) -> Result<Vec<u128>> {
    (1..=max_dimension)
        .map(|d| distinct_designs(base, d))
        .collect()
}

pub fn bracelets(max_base: usize) -> Result<Vec<u128>> {
    (1..=max_base).map(|q| distinct_designs(q, 1)).collect()
}

pub fn fill_from_corners(filled: &[Vec<u8>], number: usize, dimension: usize) -> u128 {
    let even = number.div_ceil(2) as u128;
    let odd = (number / 2) as u128;
    filled
        .iter()
        .map(|corner| {
            let popcount = corner.iter().filter(|&&b| b != 0).count();
            even.pow((dimension - popcount) as u32) * odd.pow(popcount as u32)
        })
        .sum()
}

pub fn even_fill_is_balanced(number: usize, dimension: usize, popcount: u128) -> Result<u128> {
    if !number.is_multiple_of(2) {
        return value_error("the duality collapse holds only at even number.");
    }
    Ok(((number / 2) as u128).pow(dimension as u32) * popcount)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn group_orders_match_prediction() {
        for base in 2..=5 {
            for dimension in 1..=3 {
                assert_eq!(
                    group_order(base, dimension),
                    predicted_group_order(base, dimension)
                );
            }
        }
    }
    #[test]
    fn base2_matches_bang() {
        use super::super::universe::bang;
        for d in 1..=3 {
            assert_eq!(distinct_designs(2, d).unwrap(), bang(d).distinct() as u128);
        }
    }
    #[test]
    fn bracelet_sequence_is_a000029() {
        assert_eq!(bracelets(8).unwrap(), vec![2, 3, 4, 6, 8, 13, 18, 30]);
    }
}
