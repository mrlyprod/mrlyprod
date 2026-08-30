use super::factory::residue_corners;
use super::universe::{permutations, Code};
use mrlycore::errors::{value_error, Result};
use mrlynum::classics::factorial;
use std::collections::{BTreeSet, HashMap};

/// The most cells a code walk visits, so that the walk stays within `2^20` codes.
pub const WALK_LIMIT: usize = 20;

/// Returns the distinct rotation and reflection maps of a base-q axis.
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

/// Returns the symmetry group order counted from the enumerated axis maps.
pub fn group_order(base: usize, dimension: usize) -> u128 {
    (axis_maps(base).len() as u128).pow(dimension as u32) * factorial(dimension)
}

/// Returns the closed-form group order the axis-map count must match.
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

/// Counts base-q designs distinct under symmetry, or an error when the Burnside average breaks.
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

/// Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.
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

/// Carries a code through one group element.
pub fn carry(element: &[usize], code: Code) -> Code {
    element
        .iter()
        .enumerate()
        .filter(|(index, _)| code >> index & 1 == 1)
        .map(|(_, &image)| 1u128 << image)
        .sum()
}

/// Returns every code a design reaches under the group.
pub fn orbit(group: &[Vec<usize>], code: Code) -> BTreeSet<Code> {
    group.iter().map(|element| carry(element, code)).collect()
}

/// Returns the least code of the design's orbit.
pub fn canonical(group: &[Vec<usize>], code: Code) -> Code {
    orbit(group, code)
        .into_iter()
        .next()
        .expect("the group is not empty")
}

/// Walks every code of a base and dimension and returns each orbit's least code with the orbit's size, or an error past the walk limit.
///
/// ```
/// assert_eq!(mrlymath::bang::baseq::representatives(3, 1).unwrap().len(), 4);
/// ```
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
    for code in 0..1u128 << cells {
        if seen[code as usize] {
            continue;
        }
        let orbit = orbit(&group, code);
        out.push((code, orbit.len()));
        for member in orbit {
            seen[member as usize] = true;
        }
    }
    Ok(out)
}

/// Returns the raw design count before symmetry, two to the number of cells.
pub fn total_designs(base: usize, dimension: usize) -> u128 {
    let cells = base.pow(dimension as u32);
    assert!(cells < 128, "too many cells for a u128 count");
    1 << cells
}

/// Returns the distinct-design counts for dimensions 1 through max_dimension.
pub fn sequence(base: usize, max_dimension: usize) -> Result<Vec<u128>> {
    (1..=max_dimension)
        .map(|d| distinct_designs(base, d))
        .collect()
}

/// Returns the distinct one-dimensional design counts for bases 1 through max_base.
pub fn bracelets(max_base: usize) -> Result<Vec<u128>> {
    (1..=max_base).map(|q| distinct_designs(q, 1)).collect()
}

/// Returns the filled-cell count of a binary design at a side number, folded from its filled corners.
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

/// Returns the collapsed fill count at an even side number, or an error at odd.
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
    fn the_walk_agrees_with_burnside_and_the_base_two_universe() {
        use super::super::catalog::universe_codes;
        for (base, dimension) in [(3usize, 1usize), (3, 2), (4, 1), (4, 2), (5, 1)] {
            let walk = representatives(base, dimension).unwrap();
            assert_eq!(
                walk.len() as u128,
                distinct_designs(base, dimension).unwrap(),
                "q={base} d={dimension}"
            );
            let total: usize = walk.iter().map(|&(_, size)| size).sum();
            assert_eq!(total, 1 << base.pow(dimension as u32));
        }
        for dimension in 1..=4 {
            let codes: Vec<Code> = representatives(2, dimension)
                .unwrap()
                .into_iter()
                .map(|(code, _)| code)
                .collect();
            assert_eq!(codes, universe_codes(dimension));
        }
        assert_eq!(representatives(3, 2).unwrap().len(), 26);
        assert_eq!(representatives(4, 2).unwrap().len(), 805);
        assert!(representatives(3, 3).is_err());
    }
    #[test]
    fn bracelet_sequence_is_a000029() {
        assert_eq!(bracelets(8).unwrap(), vec![2, 3, 4, 6, 8, 13, 18, 30]);
    }
}
