//! The spatial network.
//!
//! A grid lifts into nodes and branches, as a core, an edge or a tunnel network, and the census
//! reads its roles, tips, junctions and components back.

/// The role tags, lengths, tip, junction and component counts of a network.
pub mod census;
/// The core, edge and tunnel networks lifted from a grid.
pub mod extract;
/// The force-directed layout that lets a lattice relax.
pub mod layout;
/// The node, branch and network types.
pub mod models;

pub use census::{
    census, components, fractal_dimension, junctions, largest_component, roles, tips, total_length,
    Census, Role,
};
pub use extract::{core_graph, edge_graph, tunnel_graph};
pub use layout::Layout;
pub use models::{Branch, Network, Node};

#[cfg(test)]
mod tests {
    use super::{census, Network, Role};
    use crate::math::round_trip;

    #[test]
    fn serde_round_trips() {
        let mut net = Network::new(2);
        for corner in [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] {
            net.add_node(corner.to_vec()).unwrap();
        }
        for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
            net.add_branch(a, b, 1.0).unwrap();
        }
        round_trip(net.nodes[0].clone());
        round_trip(net.branches[0].clone());
        round_trip(Role::Junction);
        round_trip(census(&net).unwrap());
        round_trip(net);
    }
}
