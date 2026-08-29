use crate::bang::code_to_corners;
use crate::bang::universe::Code;
use crate::formulas::counting::positions;
use mrlycore::errors::{value_error, Result};
use std::collections::HashSet;

/// Returns the triangles of the full hexagon with side number to the level.
pub fn grid_triangles(number: usize, level: u32) -> u128 {
    6 * (number as u128).pow(2 * level)
}

fn odd_k(number: usize) -> Result<u128> {
    if number.is_multiple_of(2) {
        return value_error("solid slice closed form is defined for odd number = 2k-1.");
    }
    Ok(number.div_ceil(2) as u128)
}

/// Returns the core node count of the solid slice, defined for odd number.
pub fn solid_slice_core_nodes(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((24 * k * k - 24 * k + 6) as u128)
}

/// Returns the core edge count of the solid slice, defined for odd number.
pub fn solid_slice_core_edges(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((36 * k * k - 42 * k + 12) as u128)
}

/// Returns the triangle count of the solid slice, defined for odd number.
pub fn solid_slice_triangles(number: usize) -> Result<u128> {
    odd_k(number)?;
    Ok(6 * (number as u128).pow(2))
}

/// Returns the boundary edge count of the solid slice, defined for odd number.
pub fn solid_slice_boundary(number: usize) -> Result<u128> {
    odd_k(number)?;
    Ok(6 * number as u128)
}

/// Returns the vertex count of the solid slice, defined for odd number.
pub fn solid_slice_vertices(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((12 * k * k - 6 * k + 1) as u128)
}

/// Returns the interior edge count of the solid slice, defined for odd number.
pub fn solid_slice_interior(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((36 * k * k - 42 * k + 12) as u128)
}

/// Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.
pub fn centered_hexagonal(m: usize) -> u128 {
    let m = m as u128;
    3 * m * m - 3 * m + 1
}

/// Returns the distinct triangle-edge count of the solid slice, defined for odd number.
pub fn solid_slice_edges(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((36 * k * k - 30 * k + 6) as u128)
}

/// Returns the filled triangle count of the code's pro projection at the given level, without rendering it.
pub fn pro_fills(code: Code, number: usize, level: u32) -> Result<u128> {
    let filled = code_to_corners(code, 3, 2)?;
    let boundary = ((number - 1) % 2) as u8;
    let mut total: u128 = 0;
    for axis in 0..3 {
        let slab: u128 = filled
            .iter()
            .filter(|c| c[axis] == boundary)
            .map(|c| {
                (0..3)
                    .filter(|&j| j != axis)
                    .map(|j| positions(c[j] as usize, number, 2))
                    .product::<u128>()
            })
            .sum();
        total += slab.pow(level);
    }
    Ok(2 * total)
}

/// Returns the empty triangle count of the code's pro projection at the given level.
pub fn pro_voids(code: Code, number: usize, level: u32) -> Result<u128> {
    Ok(grid_triangles(number, level) - pro_fills(code, number, level)?)
}

/// Returns the filled triangle count of the code's cut section at the given level, without rendering it.
pub fn cut_fills(code: Code, number: usize, level: u32) -> Result<u128> {
    let filled: HashSet<Vec<u8>> = code_to_corners(code, 3, 2)?.into_iter().collect();
    let scaled = number.pow(level);
    let size = 4 * scaled;
    let k = (3 * (size - 1)) / 2;
    let mut total: u128 = 0;
    for z in (0..size).step_by(2) {
        let target = k - z;
        let min_x = target.saturating_sub(size - 1);
        let max_x = (size - 1).min(target);
        for x in min_x..=max_x {
            let y = target - x;
            let (mut a, mut b, mut c) = (x / 4, y / 4, z / 4);
            let mut inside = true;
            for _ in 0..level {
                let corner = vec![
                    (a % number % 2) as u8,
                    (b % number % 2) as u8,
                    (c % number % 2) as u8,
                ];
                if !filled.contains(&corner) {
                    inside = false;
                    break;
                }
                a /= number;
                b /= number;
                c /= number;
            }
            if inside {
                total += 1;
            }
        }
    }
    Ok(total)
}

/// Returns the empty triangle count of the code's cut section at the given level.
pub fn cut_voids(code: Code, number: usize, level: u32) -> Result<u128> {
    Ok(grid_triangles(number, level) - cut_fills(code, number, level)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pro_and_cut_match_census() {
        use crate::six::{cut, pro};
        use crate::three;
        use mrlynum::census::count;
        for code in [0u128, 8, 17, 23, 129, 232, 255] {
            for number in [1usize, 2, 3, 4, 5, 7] {
                for level in 1..3u32 {
                    if number.pow(level) > 9 {
                        continue;
                    }
                    let cell = three::create(code, number, level as usize, 2).unwrap();
                    let p = pro(&cell).unwrap();
                    assert_eq!(
                        pro_fills(code, number, level).unwrap(),
                        count(p.cell.types(), 1) as u128,
                        "pro code={code} n={number} l={level}"
                    );
                    assert_eq!(
                        pro_voids(code, number, level).unwrap(),
                        count(p.cell.types(), 0) as u128
                    );
                    let q = cut(&cell).unwrap();
                    assert_eq!(
                        cut_fills(code, number, level).unwrap(),
                        count(q.cell.types(), 1) as u128,
                        "cut code={code} n={number} l={level}"
                    );
                    assert_eq!(
                        cut_voids(code, number, level).unwrap(),
                        count(q.cell.types(), 0) as u128
                    );
                }
            }
        }
    }
    #[test]
    fn menger_projections() {
        assert_eq!(pro_fills(23, 3, 1).unwrap(), 48);
        assert_eq!(pro_fills(23, 3, 2).unwrap(), 384);
        assert_eq!(cut_fills(23, 3, 1).unwrap(), 42);
        assert_eq!(cut_fills(23, 3, 2).unwrap(), 306);
        assert_eq!(cut_fills(255, 3, 1).unwrap(), 54);
    }
    #[test]
    fn closed_forms_at_small_numbers() {
        assert_eq!(grid_triangles(3, 1), 54);
        assert_eq!(solid_slice_triangles(3).unwrap(), 54);
        assert_eq!(solid_slice_boundary(3).unwrap(), 18);
        assert!(solid_slice_vertices(4).is_err());
        assert_eq!(solid_slice_core_nodes(1).unwrap(), 6);
        assert_eq!(solid_slice_core_edges(1).unwrap(), 6);
        assert_eq!(solid_slice_core_nodes(3).unwrap(), 54);
        assert_eq!(solid_slice_core_edges(3).unwrap(), 72);
        assert_eq!(solid_slice_vertices(3).unwrap(), 37);
        assert_eq!(solid_slice_edges(1).unwrap(), 12);
        assert_eq!(solid_slice_edges(3).unwrap(), 90);
        assert!(solid_slice_edges(4).is_err());
    }
}

#[cfg(test)]
mod theorems {
    use super::*;
    use mrlynum::prime::is_prime;

    fn slice_vertices(k: usize) -> u128 {
        solid_slice_vertices(2 * k - 1).unwrap()
    }

    #[test]
    fn the_slice_vertex_count_is_centered_hexagonal_at_even_index() {
        let opening: Vec<u128> = (1..6).map(centered_hexagonal).collect();
        assert_eq!(opening, [1, 7, 19, 37, 61]);
        for k in 1..41usize {
            let m = 2 * k as u128;
            assert_eq!(slice_vertices(k), centered_hexagonal(2 * k), "k={k}");
            assert_eq!(slice_vertices(k) % 3, 1, "k={k}");
            assert_eq!(
                slice_vertices(k),
                m * m + m * (m - 1) + (m - 1) * (m - 1),
                "k={k}"
            );
        }
    }

    #[test]
    fn the_prime_vertex_counts_are_cuban_with_norm_witnesses() {
        let mut prime_at = Vec::new();
        let mut primes = Vec::new();
        let mut composites = Vec::new();
        for k in 1..21usize {
            if is_prime(slice_vertices(k) as usize) {
                prime_at.push(k);
                primes.push(slice_vertices(k));
            } else {
                composites.push(slice_vertices(k));
            }
        }
        assert_eq!(prime_at, [1, 2, 5, 6, 7, 9, 12, 13, 14, 19]);
        assert_eq!(primes, [7, 37, 271, 397, 547, 919, 1657, 1951, 2269, 4219]);
        assert_eq!(
            composites,
            [91, 169, 721, 1141, 1387, 2611, 2977, 3367, 3781, 4681]
        );
        let far: Vec<u128> = (21..41usize)
            .map(slice_vertices)
            .filter(|&v| is_prime(v as usize))
            .collect();
        assert_eq!(far, [5167, 6211, 7351, 9241, 12097, 13669]);
        for value in primes.iter().chain(far.iter()) {
            assert_eq!(value % 3, 1, "value={value}");
        }
        assert_eq!(4219u128, 37 * 37 + 37 * 38 + 38 * 38);
    }
}
