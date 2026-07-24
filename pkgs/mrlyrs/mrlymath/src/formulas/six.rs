use crate::bang::code_to_corners;
use crate::bang::universe::Code;
use crate::formulas::counting::positions;
use mrlycore::errors::{value_error, Result};
use std::collections::HashSet;

pub fn grid_triangles(number: usize, level: u32) -> u128 {
    6 * (number as u128).pow(2 * level)
}

fn odd_k(number: usize) -> Result<u128> {
    if number.is_multiple_of(2) {
        return value_error("solid slice closed form is defined for odd number = 2k-1.");
    }
    Ok(number.div_ceil(2) as u128)
}

pub fn solid_slice_core_nodes(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((24 * k * k - 24 * k + 6) as u128)
}

pub fn solid_slice_core_edges(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((36 * k * k - 42 * k + 12) as u128)
}

pub fn solid_slice_triangles(number: usize) -> Result<u128> {
    odd_k(number)?;
    Ok(6 * (number as u128).pow(2))
}

pub fn solid_slice_boundary(number: usize) -> Result<u128> {
    odd_k(number)?;
    Ok(6 * number as u128)
}

pub fn solid_slice_vertices(number: usize) -> Result<u128> {
    let k = odd_k(number)? as i128;
    Ok((12 * k * k - 6 * k + 1) as u128)
}

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

pub fn pro_voids(code: Code, number: usize, level: u32) -> Result<u128> {
    Ok(grid_triangles(number, level) - pro_fills(code, number, level)?)
}

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

pub fn cut_voids(code: Code, number: usize, level: u32) -> Result<u128> {
    Ok(grid_triangles(number, level) - cut_fills(code, number, level)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pro_and_cut_match_census() {
        use crate::census::count;
        use crate::six::{cut, pro};
        use crate::three;
        for code in [0u128, 8, 17, 23, 129, 232, 255] {
            for number in 1..5usize {
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
    }
}
