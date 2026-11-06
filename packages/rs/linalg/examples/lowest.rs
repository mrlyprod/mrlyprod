// The lowest eigenpairs of a large sparse matrix, without building it dense.
// cargo run -p mrly-linalg --example lowest
use mrly_linalg::lowest;
use std::f64::consts::PI;

fn main() {
    // A chain of 400 nodes: 2 on the diagonal, -1 to each neighbor.
    // Rows are lists of (column, value). Only nonzeros are stored.
    let n = 400;
    let rows: Vec<Vec<(usize, f64)>> = (0..n)
        .map(|i| {
            let mut row = vec![(i, 2.0)];
            if i > 0 {
                row.push((i - 1, -1.0));
            }
            if i + 1 < n {
                row.push((i + 1, -1.0));
            }
            row
        })
        .collect();

    // Ask for 4 pairs. gap says how close two values must be to count as a tie and stay together.
    let (values, vectors) = lowest(&rows, 4, 1e-9).unwrap();

    // This matrix has a closed form: 2 - 2 cos(k pi / (n + 1)).
    for (k, value) in values.iter().enumerate() {
        let exact = 2.0 - 2.0 * ((k + 1) as f64 * PI / (n + 1) as f64).cos();
        println!("k={} value {value:.8}  exact {exact:.8}", k + 1);
    }
    println!("vectors {} of length {}", vectors.len(), vectors[0].len());
}
