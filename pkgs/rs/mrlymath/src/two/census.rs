use super::graph::edge_graph;
use super::models::Cell2d;
use crate::dim::census;
use mrlycore::errors::Result;

pub use crate::dim::census::{edges, vertices};

/// One reading of a cell: its sites, its outline and its topology.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    /// The count of filled sites.
    pub fills: usize,
    /// The count of empty sites.
    pub voids: usize,
    /// The count of filled faces open to emptiness or the border.
    pub perimeter: u128,
    /// The count of distinct corners the filled sites touch.
    pub vertices: usize,
    /// The count of distinct unit edges the filled sites carry.
    pub edges: usize,
    /// The Euler characteristic, vertices less edges plus filled sites.
    pub euler: i64,
}

/// Counts the filled sites of the cell.
pub fn fills(cell: &Cell2d) -> usize {
    census::fills(cell)
}

/// Counts the empty sites of the cell.
pub fn voids(cell: &Cell2d) -> usize {
    census::voids(cell)
}

/// Counts the faces of filled sites open to emptiness or the border.
pub fn perimeter(cell: &Cell2d) -> u128 {
    census::exposure(cell)
}

/// Returns the Euler characteristic of the filled sites, vertices less edges plus faces.
///
/// ```
/// let solid = mrlymath::two::ones(4, 1).unwrap();
/// assert_eq!(mrlymath::two::census::euler(&solid).unwrap(), 1);
/// let ring = mrlymath::two::carpet(3, 1).unwrap();
/// assert_eq!(mrlymath::two::census::euler(&ring).unwrap(), 0);
/// ```
pub fn euler(cell: &Cell2d) -> Result<i64> {
    let outline = edge_graph(cell)?;
    Ok(outline.nodes.len() as i64 - outline.branches.len() as i64 + fills(cell) as i64)
}

/// Takes the cell's full census in one reading.
///
/// ```
/// let cell = mrlymath::two::carpet(3, 1).unwrap();
/// let census = mrlymath::two::census::census(&cell).unwrap();
/// assert_eq!((census.fills, census.voids, census.perimeter), (8, 1, 16));
/// assert_eq!((census.vertices, census.edges, census.euler), (16, 24, 0));
/// ```
pub fn census(cell: &Cell2d) -> Result<Census> {
    let outline = edge_graph(cell)?;
    let (vertices, edges) = (outline.nodes.len(), outline.branches.len());
    let faces = fills(cell);
    Ok(Census {
        fills: faces,
        voids: voids(cell),
        perimeter: perimeter(cell),
        vertices,
        edges,
        euler: vertices as i64 - edges as i64 + faces as i64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas;
    use crate::two::designs;
    #[test]
    fn census_matches_formulas() {
        for code in [7u128, 14, 9, 5] {
            for level in 1..4u32 {
                let cell = designs::create(code, 3, level as usize, 0, 2).unwrap();
                assert_eq!(
                    fills(&cell) as u128,
                    formulas::fill(code, 3, 2, level, 2).unwrap(),
                    "code={code} l={level}"
                );
                assert_eq!(
                    voids(&cell) as u128,
                    formulas::void(code, 3, 2, level, 2).unwrap()
                );
            }
        }
    }
    #[test]
    fn carpet_census() {
        let c = designs::carpet(3, 1).unwrap();
        assert_eq!(
            census(&c).unwrap(),
            Census {
                fills: 8,
                voids: 1,
                perimeter: 16,
                vertices: 16,
                edges: 24,
                euler: 0,
            }
        );
    }
    #[test]
    fn solid_blocks_are_discs() {
        for n in 1..6 {
            let solid = designs::ones(n, 1).unwrap();
            let reading = census(&solid).unwrap();
            assert_eq!(reading.vertices, (n + 1) * (n + 1), "n={n}");
            assert_eq!(reading.edges, 2 * n * (n + 1));
            assert_eq!(reading.euler, 1);
        }
    }
    #[test]
    fn every_corner_is_touched_but_the_net() {
        for n in [3usize, 5, 7] {
            for cell in [designs::carpet(n, 1), designs::void(n, 1)] {
                assert_eq!(vertices(&cell.unwrap()).unwrap(), (n + 1) * (n + 1));
            }
            assert!(vertices(&designs::net(n, 1).unwrap()).unwrap() < (n + 1) * (n + 1));
        }
    }
    #[test]
    fn euler_counts_the_holes() {
        assert_eq!(euler(&designs::carpet(3, 2).unwrap()).unwrap(), -8);
        assert_eq!(edges(&designs::ones(3, 1).unwrap()).unwrap(), 24);
    }
}
