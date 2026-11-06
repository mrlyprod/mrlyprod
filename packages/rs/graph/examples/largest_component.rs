// A network with two separate pieces: a path of 4 nodes and a path of 2.
// largest_component() keeps only the bigger piece, renumbered from 0.
// cargo run -p mrly-graph --example largest_component

use mrly_graph::{Network, components, largest_component};

fn main() {
    let mut network = Network::new(2);
    for k in 0..6 {
        network.add_node(vec![k as f64, 0.0]).unwrap();
    }
    for (a, b) in [(0, 1), (1, 2), (2, 3), (4, 5)] {
        network.add_branch(a, b, 1.0).unwrap();
    }
    let giant = largest_component(&network).unwrap();
    println!(
        "before: {} nodes in {} pieces",
        network.nodes.len(),
        components(&network).unwrap()
    );
    println!(
        "after:  {} nodes in {} piece",
        giant.nodes.len(),
        components(&giant).unwrap()
    );
}
