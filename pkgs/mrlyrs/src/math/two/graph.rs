pub use crate::math::dim::graph::{core_graph, edge_graph, tunnel_graph};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::graph::census;
    use crate::math::two::designs;
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
