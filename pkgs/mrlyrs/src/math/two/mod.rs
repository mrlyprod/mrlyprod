//! The flat-cell pipeline.
//!
//! The shared cell pipeline pinned to two dimensions: coded and carpet cells, their censuses,
//! their payloads, their text and PNG renderings and their JSON.

/// The fill, void, perimeter, corner, edge and Euler counts of a flat cell.
pub mod census;
/// The builders of coded, corner, noise and carpet cells.
pub mod designs;
/// The merges, magic folds, masked mosaics and the cube lift of flat cells.
pub mod geometry;
/// The payload a cell's filled sites carry, framed in a sheet.
pub mod payload;
/// The text and PNG renderings of a flat cell.
pub mod renderer;
/// The JSON form of a flat cell.
pub mod serializer;

pub use crate::math::cell::graph::{core_graph, edge_graph, tunnel_graph};
pub use crate::math::cell::models::Cell2d;
pub use crate::math::cell::paint;
pub use census::{census, euler, fills, Census};
pub use designs::{
    carpet, create, dust, from_corners, hline, htree, level_set, levels_code, named, net, ones,
    point, star, vline, void, vtree, zeros,
};
pub use geometry::{magic, merge, mosaic, special, to_3d};
pub use payload::{capacity, embed, extract, read, sheet};
pub use renderer::{png, text};
pub use serializer::{from_json, to_json};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::colors::{BLACK, WHITE};
    use crate::math::round_trip;
    #[test]
    fn serde_round_trips() {
        let cell = designs::carpet(3, 1).unwrap();
        round_trip(census(&cell).unwrap());
        round_trip(cell);
    }
    #[test]
    fn default_paint_is_black_on_white() {
        let c = paint(designs::carpet(3, 1).unwrap(), None, None);
        let colors = c.cell.colors.as_ref().unwrap();
        assert_eq!(colors[0], [BLACK.r, BLACK.g, BLACK.b, 255]);
        assert_eq!(colors[4], [WHITE.r, WHITE.g, WHITE.b, 255]);
        let blacks = colors
            .iter()
            .filter(|c| **c == [BLACK.r, BLACK.g, BLACK.b, 255])
            .count();
        assert_eq!(blacks as u64, designs::carpet(3, 1).unwrap().types().sum());
    }
}

#[cfg(test)]
mod spectra {
    use super::*;
    use crate::math::bang::Code;
    use crate::math::graph::census;
    use crate::math::spectrum::{clusters, laplacian_spectrum, multiplicity};

    #[test]
    #[ignore = "level six is 729 nodes through the cubic eigensolver, 2.8 s; run it in release"]
    fn the_deep_sierpinski_spectrum_holds_its_degeneracy_row() {
        let root = 30f64.sqrt() / 6.0;
        let cell = designs::create(Code(7), 2, 6, 0, 2).unwrap();
        let graph = core_graph(&cell).unwrap();
        assert_eq!(graph.nodes.len(), 729);
        let spectrum = laplacian_spectrum(&graph, true).unwrap();
        let groups = clusters(&spectrum, 1e-9).unwrap();
        let repeated: usize = groups.iter().filter(|g| g.1 > 1).map(|g| g.1).sum();
        assert_eq!(groups.len(), 289);
        assert_eq!(groups.iter().filter(|g| g.1 > 1).count(), 67);
        assert_eq!(
            (repeated as f64 / 729.0 * 10000.0).round() / 10000.0,
            0.6955
        );
        assert_eq!(multiplicity(&spectrum, 1.0, 1e-12), 243);
        assert_eq!(multiplicity(&spectrum, 1.0 - root, 1e-12), 28);
        assert_eq!(multiplicity(&spectrum, 1.0 + root, 1e-12), 28);
    }

    #[test]
    fn the_sierpinski_normalised_spectrum_holds_its_degeneracy_table() {
        let root = 30f64.sqrt() / 6.0;
        let rows = [
            (1usize, 3usize, 3usize, 0usize, 0.0000, 1usize, 0usize),
            (2, 9, 7, 1, 0.3333, 3, 1),
            (3, 27, 17, 3, 0.4815, 9, 2),
            (4, 81, 43, 9, 0.5802, 27, 4),
            (5, 243, 111, 25, 0.6461, 81, 10),
        ];
        for (level, nodes, distinct, classes, fraction, one, pair) in rows {
            let cell = designs::create(Code(7), 2, level, 0, 2).unwrap();
            let graph = core_graph(&cell).unwrap();
            assert_eq!(graph.nodes.len(), nodes, "l={level}");
            assert_eq!(census(&graph).unwrap().components, 1, "l={level}");
            let spectrum = laplacian_spectrum(&graph, true).unwrap();
            let groups = clusters(&spectrum, 1e-9).unwrap();
            let repeated: usize = groups.iter().filter(|g| g.1 > 1).map(|g| g.1).sum();
            assert_eq!(groups.len(), distinct, "l={level}");
            assert_eq!(
                groups.iter().filter(|g| g.1 > 1).count(),
                classes,
                "l={level}"
            );
            let got = (repeated as f64 / nodes as f64 * 10000.0).round() / 10000.0;
            assert!((got - fraction).abs() < 1e-9, "l={level} {got}");
            assert_eq!(multiplicity(&spectrum, 1.0, 1e-12), one, "l={level}");
            assert_eq!(
                multiplicity(&spectrum, 1.0 - root, 1e-12),
                pair,
                "l={level}"
            );
            assert_eq!(
                multiplicity(&spectrum, 1.0 + root, 1e-12),
                pair,
                "l={level}"
            );
        }
    }
}
