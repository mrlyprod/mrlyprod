use super::graph::edge_graph;
use super::models::Cell3d;
use crate::dim::census;
use mrlycore::errors::Result;

pub use crate::dim::census::{edges, vertices};

/// The tally of a cube's sites.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    /// The count of filled sites.
    pub fills: usize,
    /// The count of empty sites.
    pub voids: usize,
    /// The count of exposed faces.
    pub surface: u128,
    /// The count of corners the filled sites touch.
    pub vertices: usize,
    /// The count of unit edges the filled sites touch.
    pub edges: usize,
    /// The count of unit faces the filled sites touch, shared ones counted once.
    pub faces: usize,
    /// The Euler characteristic of the filled complex.
    pub euler: i64,
}

/// Returns the count of filled sites.
pub fn fills(cell: &Cell3d) -> usize {
    census::fills(cell)
}

/// Returns the count of empty sites.
pub fn voids(cell: &Cell3d) -> usize {
    census::voids(cell)
}

/// Returns the filled-site count, the cube's volume.
pub fn volume(cell: &Cell3d) -> usize {
    fills(cell)
}

/// Returns the count of filled faces exposed to void or the outside.
pub fn surface(cell: &Cell3d) -> u128 {
    census::exposure(cell)
}

/// Returns the count of faces buried between two filled sites, six per site less the exposed surface.
///
/// ```
/// let block = mrlymath::three::ones(2, 1).unwrap();
/// assert_eq!(mrlymath::three::census::hidden(&block), 24);
/// ```
pub fn hidden(cell: &Cell3d) -> u128 {
    6 * fills(cell) as u128 - surface(cell)
}

/// Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.
///
/// Surface counts only the faces open to void; this counts every face of the complex.
///
/// ```
/// let block = mrlymath::three::ones(2, 1).unwrap();
/// assert_eq!(mrlymath::three::census::faces(&block), 36);
/// assert_eq!(mrlymath::three::census::surface(&block), 24);
/// ```
pub fn faces(cell: &Cell3d) -> usize {
    let grid = cell.types();
    let (dx, dy, dz) = (grid.shape[0], grid.shape[1], grid.shape[2]);
    let mut planes = [
        vec![false; (dx + 1) * dy * dz],
        vec![false; dx * (dy + 1) * dz],
        vec![false; dx * dy * (dz + 1)],
    ];
    for i in 0..dx {
        for j in 0..dy {
            for k in 0..dz {
                if grid.get(&[i, j, k]) == 0 {
                    continue;
                }
                for step in 0..2 {
                    planes[0][((i + step) * dy + j) * dz + k] = true;
                    planes[1][(i * (dy + 1) + j + step) * dz + k] = true;
                    planes[2][(i * dy + j) * (dz + 1) + k + step] = true;
                }
            }
        }
    }
    planes.iter().flatten().filter(|&&on| on).count()
}

/// Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.
///
/// One for a solid block, and one less than twice the genus below zero for a tunnelled one.
///
/// ```
/// assert_eq!(mrlymath::three::census::euler(&mrlymath::three::ones(2, 1).unwrap()).unwrap(), 1);
/// assert_eq!(mrlymath::three::census::euler(&mrlymath::three::carpet(3, 1).unwrap()).unwrap(), -4);
/// ```
pub fn euler(cell: &Cell3d) -> Result<i64> {
    let net = edge_graph(cell)?;
    let (v, e) = (net.nodes.len() as i64, net.branches.len() as i64);
    Ok(v - e + faces(cell) as i64 - fills(cell) as i64)
}

/// Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.
///
/// ```
/// let tally = mrlymath::three::census::census(&mrlymath::three::carpet(3, 1).unwrap()).unwrap();
/// assert_eq!((tally.fills, tally.voids, tally.surface), (20, 7, 72));
/// assert_eq!((tally.vertices, tally.edges, tally.faces, tally.euler), (64, 144, 96, -4));
/// ```
pub fn census(cell: &Cell3d) -> Result<Census> {
    let net = edge_graph(cell)?;
    let (vertices, edges) = (net.nodes.len(), net.branches.len());
    let (fills, faces) = (fills(cell), faces(cell));
    Ok(Census {
        fills,
        voids: voids(cell),
        surface: surface(cell),
        vertices,
        edges,
        faces,
        euler: vertices as i64 - edges as i64 + faces as i64 - fills as i64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas;
    use crate::three::designs;
    #[test]
    fn census_matches_formulas() {
        for code in [23u128, 129, 17, 232] {
            for level in 1..3u32 {
                let cell = designs::create(code, 3, level as usize, 2).unwrap();
                assert_eq!(
                    fills(&cell) as u128,
                    formulas::fill(code, 3, 3, level, 2).unwrap()
                );
                assert_eq!(
                    surface(&cell),
                    formulas::surface(code, 3, level, 2).unwrap(),
                    "code={code} l={level}"
                );
            }
        }
    }
    #[test]
    fn menger_census() {
        let c = designs::carpet(3, 1).unwrap();
        let result = census(&c).unwrap();
        assert_eq!(result.fills, 20);
        assert_eq!(result.voids, 7);
        assert_eq!(result.surface, 72);
        assert_eq!(result.vertices, 64);
        assert_eq!(result.edges, 144);
        assert_eq!(result.faces, 96);
        assert_eq!(result.euler, -4);
    }
    #[test]
    fn a_solid_block_is_contractible() {
        for n in [1, 2, 3] {
            let block = designs::ones(n, 1).unwrap();
            assert_eq!(euler(&block).unwrap(), 1, "n={n}");
        }
        let one = census(&designs::ones(1, 1).unwrap()).unwrap();
        assert_eq!((one.vertices, one.edges, one.faces), (8, 12, 6));
    }
    #[test]
    fn the_sponge_deepens_its_genus() {
        let level_two = census(&designs::carpet(3, 2).unwrap()).unwrap();
        assert_eq!(level_two.fills, 400);
        assert_eq!(level_two.vertices, 896);
        assert_eq!(level_two.edges, 2304);
        assert_eq!(level_two.faces, 1728);
        assert_eq!(level_two.euler, -80);
    }
    #[test]
    fn the_parts_agree_with_the_whole() {
        for cell in [
            designs::net(3, 1).unwrap(),
            designs::void(4, 1).unwrap(),
            designs::xtree(3, 2).unwrap(),
        ] {
            let tally = census(&cell).unwrap();
            assert_eq!(tally.vertices, vertices(&cell).unwrap());
            assert_eq!(tally.edges, edges(&cell).unwrap());
            assert_eq!(tally.faces, faces(&cell));
            assert_eq!(tally.euler, euler(&cell).unwrap());
            assert_eq!(tally.fills, volume(&cell));
        }
    }
}

#[cfg(test)]
mod theorems {
    use super::*;
    use crate::bang::universe::{orbit, total_exposure, touches_every_corner};
    use crate::three::designs;
    use crate::two;
    use std::collections::BTreeSet;

    fn tile_fill(code: u128, number: usize) -> u128 {
        fills(&designs::create(code, number, 1, 2).unwrap()) as u128
    }

    fn state(code: u128, number: usize, level: usize) -> (i128, i128) {
        let cell = designs::create(code, number, level, 2).unwrap();
        (surface(&cell) as i128, hidden(&cell) as i128)
    }

    fn second_eigenvalue(code: u128, number: usize) -> i128 {
        let half = (number / 2) as i128;
        let side = number as i128;
        match code {
            23 => side * side - half * half,
            232 => half * half,
            _ => (side - half) * (side - half),
        }
    }

    #[test]
    fn the_face_ledger_prints_the_family_closed_forms() {
        let visible = [
            (23u128, [72u128, 1056, 18048, 336384]),
            (232, [30, 198, 1374, 9606]),
            (3, [56, 608, 7040, 83456]),
            (129, [54, 486, 4374, 39366]),
        ];
        for (code, faces) in visible {
            let fc = tile_fill(code, 3);
            for (index, &want) in faces.iter().enumerate() {
                let level = index + 1;
                let cell = designs::create(code, 3, level, 2).unwrap();
                assert_eq!(surface(&cell), want, "code={code} l={level}");
                assert_eq!(
                    hidden(&cell),
                    6 * fc.pow(level as u32) - want,
                    "code={code} l={level}"
                );
            }
        }
        let carpet: Vec<u128> = (1..5)
            .map(|level| hidden(&designs::create(23, 3, level, 2).unwrap()))
            .collect();
        assert_eq!(carpet, [48, 1344, 29952, 623616]);
    }

    #[test]
    fn the_face_matrix_fits_its_eigenvalues_and_predicts() {
        let mut fitted = 0;
        for (number, top) in [(3usize, 4usize), (5, 3), (7, 2)] {
            for code in [23u128, 232, 3, 129] {
                let fc = tile_fill(code, number) as i128;
                let l2 = second_eigenvalue(code, number);
                let states: Vec<(i128, i128)> =
                    (1..=top).map(|level| state(code, number, level)).collect();
                let work = states[0].1 / 2;
                for (index, &(v, h)) in states.iter().enumerate() {
                    let level = index as u32 + 1;
                    assert_eq!(v + h, 6 * fc.pow(level), "code={code} n={number} l={level}");
                    let want = if work == 0 {
                        0
                    } else {
                        2 * work * (fc.pow(level) - l2.pow(level)) / (fc - l2)
                    };
                    assert_eq!(h, want, "code={code} n={number} l={level}");
                }
                if top < 3 || work == 0 {
                    continue;
                }
                let ((v1, h1), (v2, h2), (v3, h3)) = (states[0], states[1], states[2]);
                let base = v1 * h2 - v2 * h1;
                assert_eq!(
                    v1 * h3 - h1 * v3,
                    (fc + l2) * base,
                    "code={code} n={number}"
                );
                assert_eq!(v2 * h3 - v3 * h2, fc * l2 * base, "code={code} n={number}");
                if code == 23 && number == 3 {
                    let entries = [
                        v2 * h2 - v3 * h1,
                        v1 * v3 - v2 * v2,
                        h2 * h2 - h3 * h1,
                        v1 * h3 - v2 * h2,
                    ];
                    for entry in entries {
                        assert_eq!(entry % base, 0, "code={code} n={number}");
                    }
                    let matrix = [
                        [entries[0] / base, entries[1] / base],
                        [entries[2] / base, entries[3] / base],
                    ];
                    assert_eq!(matrix, [[12, 4], [8, 16]]);
                }
                let numerator = (v2 * h2 - v3 * h1) * v3 + (v3 * v1 - v2 * v2) * h3;
                assert_eq!(numerator % base, 0, "code={code} n={number}");
                let next = numerator / base;
                let closed = 6 * fc.pow(4) - 2 * work * (fc.pow(4) - l2.pow(4)) / (fc - l2);
                assert_eq!(next, closed, "code={code} n={number}");
                if number == 3 {
                    assert_eq!(next, states[3].0, "code={code}");
                }
                fitted += 1;
            }
        }
        assert_eq!(fitted, 6);
    }

    #[test]
    fn the_antipodal_design_buries_no_face() {
        for k in 1..13u128 {
            let number = 2 * k as usize - 1;
            let cell = designs::create(129, number, 1, 2).unwrap();
            let cells = k * k * k + (k - 1) * (k - 1) * (k - 1);
            assert_eq!(fills(&cell) as u128, cells, "k={k}");
            assert_eq!(surface(&cell), 6 * cells, "k={k}");
            assert_eq!(hidden(&cell), 0, "k={k}");
            let flat = two::create(9, number, 1, 0, 2).unwrap();
            let tally = two::census::census(&flat).unwrap();
            assert_eq!(tally.edges as u128, tally.perimeter, "k={k}");
        }
        for level in 1..5u32 {
            let cell = designs::create(129, 3, level as usize, 2).unwrap();
            assert_eq!(surface(&cell), 6 * 9u128.pow(level), "l={level}");
        }
    }

    #[test]
    fn total_exposure_holds_for_the_independent_corner_sets() {
        for number in [3usize, 5, 7] {
            for code in 0..256u128 {
                let cell = designs::create(code, number, 1, 2).unwrap();
                let open = surface(&cell) == 6 * fills(&cell) as u128;
                assert_eq!(open, total_exposure(code, 3), "code={code} n={number}");
            }
        }
        let exposed: Vec<u128> = (0..256).filter(|&c| total_exposure(c, 3)).collect();
        let classes: BTreeSet<u128> = exposed
            .iter()
            .map(|&c| *orbit(c, 3).iter().next().unwrap())
            .collect();
        assert_eq!(exposed.len(), 35);
        assert_eq!(
            classes.into_iter().collect::<Vec<u128>>(),
            [0, 1, 6, 22, 24, 105]
        );
    }

    #[test]
    fn the_all_even_rule_touches_every_grid_corner() {
        for k in 1..4usize {
            let number = 2 * k - 1;
            let grid = (number + 1).pow(3);
            for code in 0..256u128 {
                let cell = designs::create(code, number, 1, 2).unwrap();
                let whole = vertices(&cell).unwrap() == grid;
                assert_eq!(whole, touches_every_corner(code, 3), "code={code} k={k}");
            }
        }
        assert_eq!(
            (0..256u128).filter(|&c| touches_every_corner(c, 3)).count(),
            128
        );
    }

    #[test]
    fn the_net_falls_short_of_the_grid_corners() {
        for k in 1..21usize {
            let cell = designs::create(232, 2 * k - 1, 1, 2).unwrap();
            let m = k - 1;
            assert_eq!(vertices(&cell).unwrap(), 8 * m * m * (k + 2), "k={k}");
            assert_eq!(
                8 * k * k * k - vertices(&cell).unwrap(),
                24 * k - 16,
                "k={k}"
            );
        }
        for k in 1..25usize {
            let flat = two::net(2 * k - 1, 1).unwrap();
            assert_eq!(
                two::census::vertices(&flat).unwrap(),
                4 * k * k - 4,
                "k={k}"
            );
        }
    }
}
