// Eigenvalues and eigenvectors of a small symmetric matrix.
// cargo run -p mrly-linalg --example eigen
use mrly_linalg::symmetric;

fn main() {
    // The matrix 2 on the diagonal, -1 beside it. Its eigenvalues are known exactly:
    // 2 - sqrt(2), 2, 2 + sqrt(2).
    let matrix = vec![
        vec![2.0, -1.0, 0.0],
        vec![-1.0, 2.0, -1.0],
        vec![0.0, -1.0, 2.0],
    ];
    let (values, vectors) = symmetric(&matrix).unwrap();
    let exact = [2.0 - 2f64.sqrt(), 2.0, 2.0 + 2f64.sqrt()];

    // Values come back sorted, smallest first, and vectors[k] belongs to values[k].
    for k in 0..3 {
        let v = &vectors[k];
        // Rounded so a tiny negative zero does not print as -0.0000.
        let shown: Vec<f64> = v.iter().map(|x| (x * 1e4).round() / 1e4 + 0.0).collect();
        println!(
            "value {:.6}  exact {:.6}  vector {:.4?}",
            values[k], exact[k], shown
        );

        // Check A v = value * v.
        let worst = (0..3)
            .map(|i| {
                let av: f64 = (0..3).map(|j| matrix[i][j] * v[j]).sum();
                (av - values[k] * v[i]).abs()
            })
            .fold(0.0, f64::max);
        println!("  A v = value v within 1e-12: {}", worst < 1e-12);
    }
}
