use crate::bang::factory;
use crate::bang::universe::Code;
use crate::formulas::counting::{fill_from_corners, positions};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use mrlynum::census::exposed;
use std::collections::HashSet;

fn strides(shape: &[usize]) -> Vec<usize> {
    (0..shape.len())
        .map(|axis| shape[axis + 1..].iter().product())
        .collect()
}

fn occupancy(tile: &Tensor) -> u128 {
    tile.bytes().iter().filter(|&&v| v != 0).count() as u128
}

/// Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.
///
/// A level deeper, each adjacent pair buries one face per spanning position of the block, and the
/// spanning positions of the block multiply level by level, so the exposure closes.
pub fn pairs(tile: &Tensor) -> Vec<(u128, u128)> {
    let shape = &tile.shape;
    let bytes = tile.bytes();
    let strides = strides(shape);
    (0..shape.len())
        .map(|axis| {
            let (stride, side) = (strides[axis], shape[axis]);
            let (mut adjacent, mut spanning) = (0u128, 0u128);
            for (flat, &cell) in bytes.iter().enumerate() {
                if cell == 0 {
                    continue;
                }
                let position = flat / stride % side;
                if position == 0 && bytes[flat + (side - 1) * stride] != 0 {
                    spanning += 1;
                }
                if position + 1 < side && bytes[flat + stride] != 0 {
                    adjacent += 1;
                }
            }
            (adjacent, spanning)
        })
        .collect()
}

/// The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions.
///
/// With `occ` filled cells, `V(1)` exposed faces and per axis `P` adjacent pairs and `S` spanning
/// positions, `V(L + 1) = occ V(L) - 2 sum P S^L`: the perimeter in the plane, the surface in space.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exposure {
    /// The filled cells of the tile.
    pub occupancy: u128,
    /// The exposed faces of the tile.
    pub exposed: u128,
    /// Per axis, the adjacent filled pairs and the spanning positions.
    pub axes: Vec<(u128, u128)>,
}

impl Exposure {
    /// Reads the counts off a rendered tile.
    pub fn of_tile(tile: &Tensor) -> Exposure {
        Exposure {
            occupancy: occupancy(tile),
            exposed: exposed(tile),
            axes: pairs(tile),
        }
    }
    /// Folds the counts from the filled residue corners at a side number, without rendering the tile.
    ///
    /// An adjacent pair sits at positions `i, i + 1` whose residues are `r, r + 1 mod q`, and a
    /// spanning position pairs residue 0 with the residue of `n - 1`.
    pub fn from_corners(
        filled: &[Vec<u8>],
        number: usize,
        dimension: usize,
        base: usize,
    ) -> Exposure {
        let set: HashSet<&Vec<u8>> = filled.iter().collect();
        let across = |corner: &[u8], axis: usize| -> u128 {
            (0..dimension)
                .filter(|&b| b != axis)
                .map(|b| positions(corner[b] as usize, number, base))
                .product()
        };
        let occupancy = fill_from_corners(filled, number, dimension, 1, base);
        let axes: Vec<(u128, u128)> = (0..dimension)
            .map(|axis| {
                let (mut adjacent, mut spanning) = (0u128, 0u128);
                for corner in filled {
                    let mut next = corner.clone();
                    next[axis] = ((corner[axis] as usize + 1) % base) as u8;
                    if set.contains(&next) {
                        adjacent +=
                            positions(corner[axis] as usize, number.saturating_sub(1), base)
                                * across(corner, axis);
                    }
                    if corner[axis] == 0 && number > 0 {
                        let mut far = corner.clone();
                        far[axis] = ((number - 1) % base) as u8;
                        if set.contains(&far) {
                            spanning += across(corner, axis);
                        }
                    }
                }
                (adjacent, spanning)
            })
            .collect();
        let buried: u128 = axes.iter().map(|&(adjacent, _)| adjacent).sum();
        Exposure {
            occupancy,
            exposed: 2 * dimension as u128 * occupancy - 2 * buried,
            axes,
        }
    }
    /// Returns the exposed faces of the level-fold Kronecker power, or none past a u128.
    pub fn at(&self, level: u32) -> Option<u128> {
        let mut value = self.exposed;
        for step in 1..level {
            let buried = self
                .axes
                .iter()
                .try_fold(0u128, |sum, &(adjacent, spanning)| {
                    sum.checked_add(adjacent.checked_mul(spanning.checked_pow(step)?)?)
                })?;
            value = self
                .occupancy
                .checked_mul(value)?
                .checked_sub(buried.checked_mul(2)?)?;
        }
        Some(value)
    }
    /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.
    ///
    /// The roots are `occ` and the distinct nonzero spanning counts, `occ` doubled where a
    /// spanning count equals it.
    pub fn recurrence(&self) -> Vec<i128> {
        let mut roots: Vec<u128> = self
            .axes
            .iter()
            .map(|&(_, spanning)| spanning)
            .filter(|&spanning| spanning != 0)
            .collect();
        roots.sort_unstable();
        roots.dedup();
        roots.insert(0, self.occupancy);
        let mut poly: Vec<i128> = vec![1];
        for root in roots {
            let mut next = vec![0i128; poly.len() + 1];
            for (power, &coefficient) in poly.iter().enumerate() {
                next[power] += coefficient;
                next[power + 1] -= root as i128 * coefficient;
            }
            poly = next;
        }
        poly[1..].iter().map(|&coefficient| -coefficient).collect()
    }
}

/// Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.
///
/// ```
/// let carpet = mrlymath::bang::factory::create(7, 3, 2, 2, 1).unwrap();
/// let perimeter: Vec<u128> = (1..5).map(|level| mrlymath::formulas::exposure_of_tile(&carpet, level).unwrap()).collect();
/// assert_eq!(perimeter, [16, 80, 496, 3536]);
/// ```
pub fn exposure_of_tile(tile: &Tensor, level: u32) -> Option<u128> {
    Exposure::of_tile(tile).at(level)
}

/// Returns the coefficients of the recurrence the tile's exposure obeys.
///
/// ```
/// let sponge = mrlymath::bang::factory::create(23, 3, 3, 2, 1).unwrap();
/// assert_eq!(mrlymath::formulas::exposure_recurrence(&sponge), [28, -160]);
/// ```
pub fn exposure_recurrence(tile: &Tensor) -> Vec<i128> {
    Exposure::of_tile(tile).recurrence()
}

/// Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners, or an error past a u128.
pub fn exposure(
    code: Code,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<u128> {
    let filled = factory::code_to_corners(code, dimension, base)?;
    match Exposure::from_corners(&filled, number, dimension, base).at(level) {
        Some(value) => Ok(value),
        None => value_error("the exposure passes a hundred and twenty-eight bits."),
    }
}

/// Returns the exposed face count of the code's 3D fractal at the given level.
pub fn surface(code: Code, number: usize, level: u32, base: usize) -> Result<u128> {
    exposure(code, number, 3, level, base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::atoms;
    #[test]
    fn prediction_matches_census_on_every_cube_code() {
        for code in 0..256u128 {
            for level in 1..4u32 {
                let direct = factory::create(code, 3, 3, 2, level as usize).unwrap();
                assert_eq!(
                    surface(code, 3, level, 2).unwrap(),
                    exposed(&direct),
                    "code={code} l={level}"
                );
            }
        }
    }
    #[test]
    fn prediction_matches_census_in_the_plane_and_beyond() {
        for code in 0..16u128 {
            for number in [2usize, 3, 4, 5] {
                for level in 1..4u32 {
                    let direct = factory::create(code, number, 2, 2, level as usize).unwrap();
                    assert_eq!(
                        exposure(code, number, 2, level, 2).unwrap(),
                        exposed(&direct),
                        "code={code} n={number} l={level}"
                    );
                }
            }
        }
        for code in [1u128, 23, 255, 4369, 65535, 32767] {
            for level in 1..3u32 {
                let direct = factory::create(code, 3, 4, 2, level as usize).unwrap();
                assert_eq!(exposure(code, 3, 4, level, 2).unwrap(), exposed(&direct));
            }
        }
        for code in [7u128, 100, 511] {
            let direct = factory::create(code, 3, 2, 3, 3).unwrap();
            assert_eq!(exposure(code, 3, 2, 3, 3).unwrap(), exposed(&direct));
        }
    }
    #[test]
    fn the_recurrence_holds_on_every_cube_code() {
        for code in 0..256u128 {
            let tile = factory::create(code, 3, 3, 2, 1).unwrap();
            let rule = exposure_recurrence(&tile);
            let terms: Vec<i128> = (1..8u32)
                .map(|level| exposure_of_tile(&tile, level).unwrap() as i128)
                .collect();
            for at in rule.len()..terms.len() {
                let predicted: i128 = rule
                    .iter()
                    .enumerate()
                    .map(|(back, &c)| c * terms[at - back - 1])
                    .sum();
                assert_eq!(predicted, terms[at], "code={code} at={at} rule={rule:?}");
            }
        }
    }
    #[test]
    fn the_corners_fold_what_the_tile_shows() {
        for code in 0..256u128 {
            for number in [1usize, 2, 3, 4, 5, 7] {
                let filled = factory::code_to_corners(code, 3, 2).unwrap();
                let tile = factory::create(code, number, 3, 2, 1).unwrap();
                assert_eq!(
                    Exposure::from_corners(&filled, number, 3, 2),
                    Exposure::of_tile(&tile),
                    "code={code} n={number}"
                );
            }
        }
        for (code, dimension, base) in [
            (7u128, 2usize, 3usize),
            (100, 2, 3),
            (511, 2, 3),
            (4369, 4, 2),
            (32767, 4, 2),
            (1, 1, 2),
            (2, 1, 3),
        ] {
            for number in [2usize, 3, 4, 5, 6, 9] {
                let filled = factory::code_to_corners(code, dimension, base).unwrap();
                let tile = factory::create(code, number, dimension, base, 1).unwrap();
                assert_eq!(
                    Exposure::from_corners(&filled, number, dimension, base),
                    Exposure::of_tile(&tile),
                    "code={code} d={dimension} q={base} n={number}"
                );
            }
        }
    }
    #[test]
    fn the_classics_close() {
        let sponge: Vec<u128> = (1..4).map(|l| surface(23, 3, l, 2).unwrap()).collect();
        assert_eq!(sponge, [72, 1056, 18048]);
        let carpet: Vec<u128> = (1..5).map(|l| exposure(7, 3, 2, l, 2).unwrap()).collect();
        assert_eq!(carpet, [16, 80, 496, 3536]);
        assert_eq!(
            exposure_recurrence(&factory::create(7, 3, 2, 2, 1).unwrap()),
            [11, -24]
        );
        assert_eq!(exposure_of_tile(&atoms::ones_3d(2), 3), Some(384));
        assert_eq!(exposure_of_tile(&atoms::ones_3d(1), 5), Some(6));
        assert!(exposure(23, 3, 3, 120, 2).is_err());
    }
}
