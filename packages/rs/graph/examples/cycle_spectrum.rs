// Laplacian eigenvalues of a 6-cycle: 2 - 2cos(2 pi k / 6) = 0, 1, 1, 3, 3, 4.
// clusters() groups equal values and counts how many share each.
// cargo run -p mrly-graph --example cycle_spectrum

use mrly_graph::Network;
use mrly_graph::spectrum::{clusters, laplacian_spectrum};

fn main() {
    let mut network = Network::new(2);
    for k in 0..6 {
        let angle = k as f64 * std::f64::consts::TAU / 6.0;
        network.add_node(vec![angle.cos(), angle.sin()]).unwrap();
    }
    for k in 0..6 {
        network.add_branch(k, (k + 1) % 6, 1.0).unwrap();
    }
    let spectrum = laplacian_spectrum(&network, false).unwrap();
    // The lowest value is zero up to rounding, which can print as -0.0000.
    let clean = |v: f64| if v.abs() < 1e-9 { 0.0 } else { v };
    for value in &spectrum {
        println!("{:.4}", clean(*value));
    }
    for (value, count) in clusters(&spectrum, 1e-6).unwrap() {
        println!("value {:.4} x{count}", clean(value));
    }
}
