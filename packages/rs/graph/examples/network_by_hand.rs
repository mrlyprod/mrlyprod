// Build a small network node by node and read its basic counts.
// This one is a Y: a stem with a fork at the top. Degree tells the role of each node.
// cargo run -p mrly-graph --example network_by_hand

use mrly_graph::{Network, junctions, roles, tips, total_length};

fn main() {
    let mut network = Network::new(2);
    let root = network.add_node(vec![0.0, 0.0]).unwrap();
    let fork = network.add_node(vec![0.0, 2.0]).unwrap();
    let left = network.add_node(vec![-1.0, 3.0]).unwrap();
    let right = network.add_node(vec![1.0, 3.0]).unwrap();
    network.add_branch(root, fork, 1.0).unwrap();
    network.add_branch(fork, left, 1.0).unwrap();
    network.add_branch(fork, right, 1.0).unwrap();
    println!("degrees: {:?}", network.degree().unwrap());
    println!("roles: {:?}", roles(&network).unwrap());
    println!("tips: {}", tips(&network).unwrap());
    println!("junctions: {}", junctions(&network).unwrap());
    println!("total length: {:.4}", total_length(&network));
}
