pub use crate::dim::graph::{core_graph, edge_graph, tunnel_graph};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;
    use mrlynum::graph::census;
    #[test]
    fn carpet_graphs() {
        let cell = designs::carpet(3, 1).unwrap();
        let core = core_graph(&cell).unwrap();
        assert_eq!(core.nodes.len(), 8);
        assert_eq!(core.branches.len(), 8);
        assert_eq!(census(&core).components, 1);
        assert_eq!(tunnel_graph(&cell).unwrap().nodes.len(), 1);
        assert!(edge_graph(&cell).unwrap().nodes.len() > 8);
    }
}

#[cfg(test)]
mod spectra {
    use super::*;
    use crate::two::designs;
    use mrlynum::graph::census;
    use mrlynum::spectrum::{clusters, laplacian_spectrum, multiplicity};

    #[test]
    fn the_sierpinski_normalised_spectrum_holds_its_degeneracy_table() {
        let root = 30f64.sqrt() / 6.0;
        let rows = [
            (1usize, 3usize, 3usize, 0usize, 0.0000, 1usize, 0usize),
            (2, 9, 7, 1, 0.3333, 3, 1),
            (3, 27, 17, 3, 0.4815, 9, 2),
            (4, 81, 43, 9, 0.5802, 27, 4),
            (5, 243, 111, 25, 0.6461, 81, 10),
            (6, 729, 289, 67, 0.6955, 243, 28),
        ];
        for (level, nodes, distinct, classes, fraction, one, pair) in rows {
            let cell = designs::create(7, 2, level, 0, 2).unwrap();
            let graph = core_graph(&cell).unwrap();
            assert_eq!(graph.nodes.len(), nodes, "l={level}");
            assert_eq!(census(&graph).components, 1, "l={level}");
            let spectrum = laplacian_spectrum(&graph, true).unwrap();
            let groups = clusters(&spectrum, 1e-9);
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
