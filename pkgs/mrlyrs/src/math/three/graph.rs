pub use crate::math::dim::graph::{core_graph, edge_graph, tunnel_graph};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::three::designs;
    #[test]
    fn menger_graphs() {
        let cell = designs::carpet(3, 1).unwrap();
        let core = core_graph(&cell).unwrap();
        assert_eq!(core.nodes.len(), 20);
        let edges = edge_graph(&designs::ones(1, 1).unwrap()).unwrap();
        assert_eq!(edges.nodes.len(), 8);
        assert_eq!(edges.branches.len(), 12);
        let tunnels = tunnel_graph(&cell).unwrap();
        assert_eq!(tunnels.nodes.len(), 7);
    }
}
