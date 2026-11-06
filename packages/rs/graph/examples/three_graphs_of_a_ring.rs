// Three graphs read from the same 3x3 grid: a ring of 8 filled cells around one empty cell.
// core: filled cells joined to filled neighbors. tunnel: the same for empty cells.
// edge: the corners and edges of the filled cells, as drawn lines.
// cargo run -p mrly-graph --example three_graphs_of_a_ring

use mrly_graph::{core_graph, edge_graph, tunnel_graph};
use mrly_tensor::Tensor;

fn main() {
    let ring = Tensor::of(vec![1, 1, 1, 1, 0, 1, 1, 1, 1], vec![3, 3]).unwrap();
    for (name, network) in [
        ("core", core_graph(&ring).unwrap()),
        ("tunnel", tunnel_graph(&ring).unwrap()),
        ("edge", edge_graph(&ring).unwrap()),
    ] {
        println!(
            "{name:<7} nodes {:>2} branches {:>2}",
            network.nodes.len(),
            network.branches.len()
        );
    }
}
