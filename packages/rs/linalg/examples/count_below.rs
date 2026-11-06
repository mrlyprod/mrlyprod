// Count how many eigenvalues sit below a number, without finding them.
// cargo run -p mrly-linalg --example count_below
use mrly_linalg::below;

fn main() {
    // The 2, -1 chain again, 10 nodes. Eigenvalues are 2 - 2 cos(k pi / 11), k = 1..10.
    let n = 10;
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

    // The eigenvalues run from 0.08 to 3.92 and are spread evenly in the angle.
    for shift in [0.0, 0.1, 1.0, 2.0, 3.0, 4.0] {
        println!("below {shift:<4} {}", below(&rows, shift));
    }
}
