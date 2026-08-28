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
