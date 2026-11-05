use super::code::Code;
use super::factory::residue_corners;
use super::universe::permutations;
use mrly_core::error::{Result, overflow_error, value_error};
use mrly_num::factor::factorial;
use std::collections::{BTreeSet, HashMap};

pub const WALK_LIMIT: usize = 20;

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

pub fn group_order(base: usize, dimension: usize) -> Result<u128> {
    order(axis_maps(base).len() as u128, dimension)
}

pub fn predicted_group_order(base: usize, dimension: usize) -> Result<u128> {
    let per_axis = if base == 2 { 2u128 } else { 2 * base as u128 };
    order(per_axis, dimension)
}

fn order(per_axis: u128, dimension: usize) -> Result<u128> {
    let axes = match u32::try_from(dimension)
        .ok()
        .and_then(|d| per_axis.checked_pow(d))
    {
        Some(axes) => axes,
        None => return overflow_error(format!("a group over {dimension} axes passes u128")),
    };
    match axes.checked_mul(factorial(dimension)?) {
        Some(out) => Ok(out),
        None => overflow_error(format!("a group over {dimension} axes passes u128")),
    }
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
            let fixed = 1u128.checked_shl(cycles(&perm, &choice, &axis, &cells, &index));
            total = match fixed.and_then(|fixed| total.checked_add(fixed)) {
                Some(total) => total,
                None => return value_error("the Burnside sum overflows a u128."),
            };
        }
    }
    if !total.is_multiple_of(order) {
        return value_error("Burnside average is not an integer.");
    }
    Ok(total / order)
}

pub fn group(base: usize, dimension: usize) -> Vec<Vec<usize>> {
    let cells = residue_corners(dimension, base);
    let axis = axis_maps(base);
    let mut out = Vec::new();
    for perm in permutations(dimension) {
        for choice in choices(&axis, dimension) {
            out.push(
                cells
                    .iter()
                    .map(|cell| {
                        (0..dimension).fold(0, |acc, i| {
                            acc * base + axis[choice[i]][cell[perm[i]] as usize]
                        })
                    })
                    .collect(),
            );
        }
    }
    out
}

pub fn carry(element: &[usize], code: Code) -> Code {
    Code(
        element
            .iter()
            .enumerate()
            .filter(|(index, _)| code.get() >> index & 1 == 1)
            .map(|(_, &image)| 1u128 << image)
            .sum(),
    )
}

pub fn orbit(group: &[Vec<usize>], code: Code) -> BTreeSet<Code> {
    group.iter().map(|element| carry(element, code)).collect()
}

pub fn canonical(group: &[Vec<usize>], code: Code) -> Result<Code> {
    match orbit(group, code).into_iter().next() {
        Some(least) => Ok(least),
        None => value_error("a canonical code needs a group of at least one element."),
    }
}

pub fn representatives(base: usize, dimension: usize) -> Result<Vec<(Code, usize)>> {
    let cells = base.pow(dimension as u32);
    if cells > WALK_LIMIT {
        return value_error(format!(
            "base {base} dimension {dimension} has {cells} cells, past the walk limit of {WALK_LIMIT}."
        ));
    }
    let group = group(base, dimension);
    let mut seen = vec![false; 1 << cells];
    let mut out = Vec::new();
    for bits in 0..1u128 << cells {
        if seen[bits as usize] {
            continue;
        }
        let code = Code::from(bits);
        let orbit = orbit(&group, code);
        out.push((code, orbit.len()));
        for member in orbit {
            seen[member.get() as usize] = true;
        }
    }
    Ok(out)
}

pub fn total_designs(base: usize, dimension: usize) -> Result<u128> {
    let cells = base.pow(dimension as u32);
    if cells >= 128 {
        return overflow_error(format!(
            "base {base} dimension {dimension} has {cells} cells, past the 127 a u128 count holds."
        ));
    }
    Ok(1 << cells)
}

pub fn sequence(base: usize, max_dimension: usize) -> Result<Vec<u128>> {
    (1..=max_dimension)
        .map(|d| distinct_designs(base, d))
        .collect()
}

pub fn bracelets(max_base: usize) -> Result<Vec<u128>> {
    (1..=max_base).map(|q| distinct_designs(q, 1)).collect()
}

pub fn classes(dimension: usize) -> u128 {
    let mut row = vec![1u128];
    for _ in 0..dimension {
        let mut next = vec![1u128; row.len() + 1];
        for k in 1..row.len() {
            next[k] = row[k - 1] + row[k];
        }
        row = next;
    }
    row.iter()
        .try_fold(1u128, |product, corners| product.checked_mul(corners + 1))
        .unwrap_or(u128::MAX)
}

pub fn class_sequence(max_dimension: usize) -> Vec<u128> {
    (1..=max_dimension).map(classes).collect()
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
