// Sierpinski gasket from three digits in base 2 on the Eisenstein (hexagonal) lattice.
// The lattice point (a, b) is printed at column 2a - b of row b, which shears the grid into triangles.
// cargo run -p mrly-radix --example gasket
use mrly_radix::gasket;
use std::collections::HashSet;

fn main() {
    let radix = gasket().unwrap();
    let words = radix.words(5);
    let seen: HashSet<(i64, i64)> = words.iter().copied().collect();
    println!("{} words, {} distinct points", words.len(), seen.len());
    let ys = seen.iter().map(|p| p.1);
    let (y0, y1) = (ys.clone().min().unwrap(), ys.max().unwrap());
    let cols: Vec<i64> = seen.iter().map(|p| 2 * p.0 - p.1).collect();
    let x0 = *cols.iter().min().unwrap();
    for y in (y0..=y1).rev() {
        let mut row = vec![' '; (cols.iter().max().unwrap() - x0 + 1) as usize];
        for &(a, b) in seen.iter().filter(|p| p.1 == y) {
            row[(2 * a - b - x0) as usize] = '#';
        }
        println!("{}", row.iter().collect::<String>().trim_end());
    }
}
