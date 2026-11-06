// Force layout: nodes repel, branches pull. Start an 8-cycle in a line and let it relax with a fixed seed.
// A ring has equal branches at rest, so the branch lengths should come out close to equal.
// cargo run -p mrly-graph --example ring_layout

use mrly_graph::{Layout, Network};

fn main() {
    let mut network = Network::new(2);
    for k in 0..8 {
        network.add_node(vec![k as f64, 0.0]).unwrap();
    }
    for k in 0..8 {
        network.add_branch(k, (k + 1) % 8, 1.0).unwrap();
    }
    let mut layout = Layout::from_network(&network, 7).unwrap();
    layout.step(400);
    let p = layout.positions();
    for k in 0..8 {
        let j = (k + 1) % 8;
        let d = ((p[2 * k] - p[2 * j]).powi(2) + (p[2 * k + 1] - p[2 * j + 1]).powi(2)).sqrt();
        println!("branch {k}-{j} length {d:.2}");
    }
}
